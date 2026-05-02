use rand::Rng;

use crate::matches::CustomGameLogic;

struct WinTracker {
    ticker: super::trackers::TickTracker<{Self::TICK_MS}>,
}

impl WinTracker {
    const TICK_MS: i64 = 50;

    fn new() -> Self {
        Self {
            ticker: super::trackers::TickTracker::new(),
        }
    }

    async fn do_win(generic: &crate::matches::GenericGamemodeEngine<PitLogic>, game: &PitLogic, winning_team: u8) {
        generic.game_done();
        game.abort_timer_sync().await;
        let data = rlnl::events::ingame::GameLoseWin {
            winning_team,
            end_reason: rlnl::types::GameEndReason::PitMaxKillsAchieved,
        };
        let winning_team_i32 = winning_team as i32;
        for (player_id, player_info) in generic.user_descriptors().iter() {
            if player_info.descriptor.user_id.is_none() { continue; } // skip non-user players
            let event = if player_info.descriptor.team == winning_team_i32 {
                rlnl::event_code::NetworkEvent::GameWon
            } else {
                rlnl::event_code::NetworkEvent::GameLost
            };
            generic.send_to_player(
                *player_id,
                event,
                literustlib::packet::Property::ReliableOrdered,
                &data,
            ).await;
        }
    }

    async fn check_win(generic: &crate::matches::GenericGamemodeEngine<PitLogic>, game: &PitLogic) {
        for win_condition in game.settings.wins.iter() {
            match win_condition {
                oj_rc_core::persist::config::PitWinCondition::StreakKills(streak_threshold) => {
                    for (player_id, streak) in game.player_tracking.streaks.iter() {
                        let player_streak = streak.load(std::sync::atomic::Ordering::Relaxed);
                        if player_streak >= *streak_threshold {
                            if let Some(conn) = generic.user_descriptor(*player_id) {
                                log::info!("Player {} has reached the streak win condition in game {}", player_id, generic.game_guid());
                                Self::do_win(generic, game, conn.descriptor.team as u8).await;
                                break;
                            } else {
                                log::warn!("Player {} has a winning kill streak but is not in game {}", player_id, generic.game_guid());
                            }
                        }
                    }
                },
                oj_rc_core::persist::config::PitWinCondition::TotalKills(kills_threshold) => {
                    for (player_id, player_info) in generic.user_descriptors() {
                        if player_info.counters.kills.load(std::sync::atomic::Ordering::Relaxed) >= *kills_threshold {
                            log::info!("Player {} has reached the total kills win condition in game {}", player_id, generic.game_guid());
                            Self::do_win(generic, game, player_info.descriptor.team as u8).await;
                            break;
                        }
                    }
                },
                oj_rc_core::persist::config::PitWinCondition::Score(score_threshold) => {
                    for (player_id, player_info) in generic.user_descriptors() {
                        if player_info.counters.generic_score() >= *score_threshold {
                            log::info!("Player {} has reached the total score win condition in game {}", player_id, generic.game_guid());
                            Self::do_win(generic, game, player_info.descriptor.team as u8).await;
                            break;
                        }
                    }
                },
                oj_rc_core::persist::config::PitWinCondition::Damage(dmg_threshold) => {
                    for (player_id, player_info) in generic.user_descriptors() {
                        if player_info.counters.cubes.load(std::sync::atomic::Ordering::Relaxed) >= *dmg_threshold {
                            log::info!("Player {} has reached the total damage win condition in game {}", player_id, generic.game_guid());
                            Self::do_win(generic, game, player_info.descriptor.team as u8).await;
                            break;
                        }
                    }
                },
                oj_rc_core::persist::config::PitWinCondition::Time => { /* handled elsewhere */},
            }
        }
    }

    async fn tick(&self, generic: &crate::matches::GenericGamemodeEngine<PitLogic>, game: &PitLogic) {
        let delta = self.ticker.tick();
        if delta == 0 { return; }
        Self::check_win(generic, game).await;
    }
}

struct PlayerTracker {
    streaks: std::collections::HashMap<u8, std::sync::atomic::AtomicU32>,
    best_streaks: std::collections::HashMap<u8, std::sync::atomic::AtomicU32>,
    respawns: std::collections::HashMap<u8, std::sync::atomic::AtomicI64>,
}

impl PlayerTracker {
    fn new(players: &[oj_rc_core::persist::user::PlayerDescriptor]) -> Self {
        Self {
            streaks: players.iter().map(|p| (p.player_id, std::sync::atomic::AtomicU32::new(0))).collect(),
            best_streaks: players.iter().map(|p| (p.player_id, std::sync::atomic::AtomicU32::new(0))).collect(),
            respawns: players.iter().map(|p| (p.player_id, std::sync::atomic::AtomicI64::new(0))).collect(),
        }
    }

    fn streak_leader(&self) -> Option<u8> {
        let mut max = None;
        for (player_id, streak) in self.streaks.iter() {
            if let Some((_, cur_max_streak)) = max {
                let streak = streak.load(std::sync::atomic::Ordering::Relaxed);
                if streak > cur_max_streak {
                    max = Some((*player_id, streak));
                }
            } else {
                max = Some((*player_id, streak.load(std::sync::atomic::Ordering::Relaxed)));
            }
        }
        if let Some((player_id, streak)) = max {
            if streak != 0 {
                Some(player_id)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn pit_stats(&self, users: &std::collections::HashMap<u8, crate::matches::generic::UserDescriptor>) -> rlnl::events::ingame::PitModeState {
        let mut player_stats = Vec::with_capacity(self.streaks.len());
        for (player_id, streak) in self.streaks.iter() {
            if let Some(user) = users.get(player_id) {
                player_stats.push(rlnl::types::PitPlayerStats {
                    player_id: *player_id,
                    score: user.counters.kills.load(std::sync::atomic::Ordering::Relaxed),
                    streak: streak.load(std::sync::atomic::Ordering::Relaxed),
                });
            }
        }
        rlnl::events::ingame::PitModeState {
            num_scores: player_stats.len() as u8,
            scores: player_stats,
            leader_id: self.streak_leader().map(|x| x as i32).unwrap_or(-1),
        }
    }
}

pub struct PitLogic {
    respawn_full_heal_duration: f32,
    respawn_heal_duration: f32,
    player_tracking: PlayerTracker,
    win_tracking: WinTracker,
    settings: std::sync::Arc<oj_rc_core::persist::config::PitSettings>,
    timer_task: tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
    initial_spawns: Vec<(u8, oj_rc_core::persist::config::Point)>, // (player_id, spawn_point)
}

impl PitLogic {
    pub fn new(config: &oj_rc_core::data::game_mode::GameModeConfig, map: &oj_rc_core::persist::config::MapConfig, players: &[oj_rc_core::persist::user::PlayerDescriptor], pit_settings: std::sync::Arc<oj_rc_core::persist::config::PitSettings>) -> Self {
        PitLogic {
            respawn_full_heal_duration: config.respawn_full_heal_duration,
            respawn_heal_duration: config.respawn_heal_duration,
            player_tracking: PlayerTracker::new(players),
            win_tracking: WinTracker::new(),
            timer_task: tokio::sync::Mutex::new(None),
            settings: pit_settings,
            initial_spawns: Self::generate_first_spawns(map, players),
        }
    }

    async fn abort_timer_sync(&self) {
        let mut lock = self.timer_task.lock().await;
        if let Some(timer_t) = &*lock {
            timer_t.abort();
            log::debug!("Aborted elimination match timer task");
        }
        *lock = None;
    }

    async fn do_leader_update(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, killer: u8, victim: u8) {
        let leader = if let Some(leader_id) = self.player_tracking.streak_leader() {
            leader_id
        } else {
            log::warn!("Pit game {} has no leader", generic.game_guid());
            return;
        };
        let killer_score = if let Some(user) = generic.user_descriptor(killer) {
            user.counters.kills.load(std::sync::atomic::Ordering::Relaxed)
        } else {
            log::warn!("Player {} score not found", killer);
            return;
        };
        let killer_streak = if let Some(streak) = self.player_tracking.streaks.get(&killer) {
            streak.load(std::sync::atomic::Ordering::Relaxed)
        } else {
            log::warn!("Player {} streak not found", killer);
            return;
        };
        let payload = rlnl::events::ingame::UpdatePitScore {
            player_id: killer,
            score: killer_score,
            streak: killer_streak,
            destroyed_id: victim,
            leader_id: leader,
        };
        generic.broadcast(
            rlnl::event_code::NetworkEvent::PitLeaderBoardUpdate,
            literustlib::packet::Property::ReliableOrdered,
            &payload,
            true,
        ).await;
    }

    async fn do_leaderboard_update(&self, generic: &crate::matches::GenericGamemodeEngine<Self>) {
        let latest_stats = self.player_tracking.pit_stats(generic.user_descriptors());
        generic.broadcast(
            rlnl::event_code::NetworkEvent::PitModeState,
            literustlib::packet::Property::ReliableOrdered,
            &latest_stats,
            true,
        ).await;
    }

    async fn do_selfdestruct_tasks(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, player_id: u8) {
        if let Some(player_streak) = self.player_tracking.streaks.get(&player_id) {
            player_streak.store(0, std::sync::atomic::Ordering::Relaxed);
        }
        self.do_leader_update(generic, player_id, player_id).await;
    }

    async fn do_kill_tasks(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, killer: u8, victim: u8) {
        if let Some(player_streak) = self.player_tracking.streaks.get(&victim) {
            player_streak.store(0, std::sync::atomic::Ordering::Relaxed);
        }
        if let Some(player_streak) = self.player_tracking.streaks.get(&killer) {
            if let Some(victim_respawn) = self.player_tracking.respawns.get(&victim) {
                let now = chrono::Utc::now().timestamp();
                if victim_respawn.load(std::sync::atomic::Ordering::Relaxed) <= now {
                    let killer_score = generic.user_descriptor(killer).map(|desc| desc.counters.generic_score()).unwrap_or_default();
                    let old_streak = player_streak.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if let Some(player_best_streak) = self.player_tracking.best_streaks.get(&killer) {
                        let old_best_streak = player_best_streak.fetch_max(old_streak + 1, std::sync::atomic::Ordering::Relaxed);
                        if old_best_streak == old_streak {
                            let data = rlnl::events::ingame::UpdateGameStats {
                                player_id: killer,
                                stat_id: rlnl::types::IngameStatId::BestKillStreak,
                                amount: old_streak + 1,
                                score: killer_score,
                                delta_score: 0,
                            };
                            generic.broadcast(
                                rlnl::event_code::NetworkEvent::UpdateGameStats,
                                literustlib::packet::Property::ReliableOrdered,
                                &data,
                                true,
                            ).await;
                        }
                    }
                    let data = rlnl::events::ingame::UpdateGameStats {
                        player_id: killer,
                        stat_id: rlnl::types::IngameStatId::CurrentKillStreak,
                        amount: old_streak + 1,
                        score: killer_score,
                        delta_score: 0,
                    };
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::UpdateGameStats,
                        literustlib::packet::Property::ReliableOrdered,
                        &data,
                        true,
                    ).await;
                }
            }
        }
        self.do_leader_update(generic, killer, victim).await;
    }

    fn choose_spawn_point(map_config: &oj_rc_core::persist::config::MapConfig, player_id: u8) -> (Option<usize>, oj_rc_core::persist::config::Point) {
        if map_config.pit_spawns.is_empty() {
            (None, oj_rc_core::persist::config::Point {
                x: 10.0 * (player_id as f32),
                y: 100.0,
                z: 10.0,
            })
        } else {
            let spawn_index = {
                rand::rng().random_range(0..map_config.pit_spawns.len())
            };
            (Some(spawn_index), map_config.pit_spawns[spawn_index].clone())
        }
    }

    fn generate_first_spawns(map_config: &oj_rc_core::persist::config::MapConfig, players: &[oj_rc_core::persist::user::PlayerDescriptor]) -> Vec<(u8, oj_rc_core::persist::config::Point)> {
        if map_config.pit_spawns.is_empty() {
            log::warn!("Map is missing pit spawn points, spawn points will be (poorly) generated");
        }
        let mut spawns = Vec::with_capacity(players.len());
        let mut seen_spawns = std::collections::HashSet::new();
        for player in players {
            'choose_loop: loop {
                let (spawn_i, spawn_point) = Self::choose_spawn_point(map_config, player.player_id);
                if let Some(spawn_i) = spawn_i {
                    if seen_spawns.contains(&spawn_i) {
                        continue;
                    } else {
                        seen_spawns.insert(spawn_i);
                    }
                }
                spawns.push((player.player_id, spawn_point));
                /*spawns.push(crate::matches::engine::RlnlPacket {
                    event: rlnl::event_code::NetworkEvent::FreeSpawnPoint,
                    property: literustlib::packet::Property::ReliableOrdered,
                    data: Box::new(rlnl::events::sync::SpawnPoint {
                        pos: rlnl::types::PosQuatPair {
                            pos: rlnl::types::CompressedVec3::from((spawn_point.x, spawn_point.y, spawn_point.z)),
                            rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
                        },
                        owner: player_i as u8,
                    })
                });*/
                break 'choose_loop;
            }
        }
        spawns
    }

    fn initial_spawns_to_packets(&self) -> Vec<crate::matches::RlnlPacket> {
        self.initial_spawns.iter().map(|(player_id, spawn_point)| crate::matches::engine::RlnlPacket {
            event: rlnl::event_code::NetworkEvent::FreeSpawnPoint,
            property: literustlib::packet::Property::ReliableOrdered,
            data: Box::new(rlnl::events::sync::SpawnPoint {
                pos: rlnl::types::PosQuatPair {
                    pos: rlnl::types::CompressedVec3::from((spawn_point.x, spawn_point.y, spawn_point.z)),
                    rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
                },
                owner: *player_id,
            })
        }).collect()
    }

    async fn do_respawn_tasks(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, player_id: u8) {
        log::info!("Handling respawn player {} in game {}", player_id, generic.game_guid());
        let respawn_time = std::time::Duration::from_secs(self.settings.respawn_time_seconds);
        let now = chrono::Utc::now();
        let respawn_timestamp = now + respawn_time;
        if let Some(player_respawn) = self.player_tracking.respawns.get(&player_id) {
            player_respawn.store(respawn_timestamp.timestamp(), std::sync::atomic::Ordering::Relaxed);
        }
        if let Some(player_desc) = generic.user_descriptor(player_id) {
            let respawn_payload = rlnl::events::ingame::RespawnTime {
                owner: player_id,
                waiting_time: self.settings.respawn_time_seconds as i16,
            };
            generic.broadcast(
                rlnl::event_code::NetworkEvent::SetRespawnWaitingTime,
                literustlib::packet::Property::ReliableOrdered,
                &respawn_payload,
                true
            ).await;
            let spawn_point = Self::choose_spawn_point(&generic.map_config, player_id).1;
            let connections = generic.users.read().await.values().map(|player_info| player_info.connection.clone()).collect();
            tokio::task::spawn(super::respawn_player_after(
                respawn_timestamp,
                connections,
                spawn_point,
                player_id,
                player_desc.machine.is_alive.clone(),
            ));
        }
    }
}

#[async_trait::async_trait]
impl CustomGameLogic for PitLogic {
    async fn on_player_join(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _conn: &crate::matches::generic::UserConnection, _player: &crate::matches::generic::UserDescriptor) -> bool {
        true
    }

    async fn on_player_end(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _connection: &crate::matches::generic::UserConnection, _player: &crate::matches::generic::UserDescriptor) -> bool {
        if generic.is_game_done() {
            return true;
        }
        let game_start = generic.game_start.load(std::sync::atomic::Ordering::Relaxed);
        if game_start == i64::MIN || game_start > chrono::Utc::now().timestamp() {
            // game has not started yet, player probably timed out while loading (which we can ignore)
            return true;
        }
        let read_lock = generic.users.read().await;
        if read_lock.len() == 1 {
            // nobody to play against, automatically end the game
            let player_id = read_lock.keys().next().unwrap();
            let user_info = generic.user_descriptor(*player_id).unwrap();
            WinTracker::do_win(generic, self, user_info.descriptor.team as u8).await;
        } else {
            let mut single_client = None;
            for conn in read_lock.values() {
                if let Some(conn_id) = single_client {
                    if conn.connection.connection.id() != conn_id {
                        single_client = None;
                        break;
                    }
                } else {
                    single_client = Some(conn.connection.connection.id());
                }
            }
            if single_client.is_some() {
                let user_id = read_lock.values().next().unwrap().user.account_id();
                let player_id = generic.user_key_by_user_id(user_id).unwrap();
                let user_info = generic.user_descriptor(player_id).unwrap();
                WinTracker::do_win(generic, self, user_info.descriptor.team as u8).await;
            }
        }
        true
    }

    async fn on_vehicle_destroyed(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, killer: u8, victim: u8) -> bool {
        self.do_kill_tasks(generic, killer, victim).await;
        self.do_respawn_tasks(generic, victim).await;
        true
    }

    async fn on_vehicle_self_destruct(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, user: u8, _is_classic: bool) -> bool {
        self.do_selfdestruct_tasks(generic, user).await;
        self.do_respawn_tasks(generic, user).await;
        true
    }

    async fn on_kill_bonus(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, killer: u8, victim: u8) -> bool {
        if let Some(to_reward) = generic.user_descriptor(killer) {
            to_reward.counters.kills.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            generic.send_to_player(
                killer,
                rlnl::event_code::NetworkEvent::ConfirmedKill,
                literustlib::packet::Property::ReliableOrdered,
                &rlnl::events::ingame::Kill {
                    killee_player_id: victim,
                    killer_player_id: killer,
                },
            ).await;
            /*crate::events::log_lnl_send_failure(to_reward.connection.rlnl().send_data(
                &rlnl::events::ingame::Kill {
                    killee_player_id: victim,
                    killer_player_id: killer,
                },
                rlnl::event_code::NetworkEvent::ConfirmedKill,
                literustlib::packet::Property::ReliableOrdered,
                &to_reward.connection.connection
            ).await);*/
            self.do_leaderboard_update(generic).await;
            // TODO make this a merged packet because this is a lot of network spam
            let generic_packet = to_reward.counters.get_generic_packet(killer, rlnl::types::IngameStatId::Kill, None);
            let data = rlnl::events::ingame::UpdateGameStats {
                player_id: killer,
                stat_id: rlnl::types::IngameStatId::Points,
                amount: to_reward.counters.kills.load(std::sync::atomic::Ordering::Relaxed),
                score: generic_packet.score,
                delta_score: generic_packet.delta_score,
            };
            generic.broadcast(
                rlnl::event_code::NetworkEvent::UpdateGameStats,
                literustlib::packet::Property::ReliableOrdered,
                &data,
                true,
            ).await;
            generic.broadcast(
                rlnl::event_code::NetworkEvent::UpdateGameStats,
                literustlib::packet::Property::ReliableOrdered,
                &generic_packet,
                true,
            ).await;
        }
        false
    }

    async fn extra_sync_events(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _connection: &crate::matches::generic::UserConnection, _player: &crate::matches::generic::UserDescriptor) -> Vec<crate::matches::RlnlPacket> {
        let mut initial_spawn_packets = self.initial_spawns_to_packets();
        initial_spawn_packets.push(
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::GameModeSettings,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::sync::UpdateGameModeSettings {
                    respawn_heal_duration: self.respawn_heal_duration,
                    respawn_full_heal_duration: self.respawn_full_heal_duration,
                }),
            },
        );
        initial_spawn_packets
    }

    async fn on_countdown_start(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, game_start: chrono::DateTime<chrono::Utc>) -> bool {
        let read_lock = generic.users.read().await;
        let game_end = game_start + generic.game_duration;
        let mut senders = Vec::with_capacity(read_lock.len());
        for (player_id, conn) in read_lock.iter() {
            let state = generic.user_descriptor(*player_id).unwrap().state.clone();
            senders.push((conn.connection.clone(), state));
        }
        drop(read_lock);
        let end_packets = if self.settings.wins.iter().any(|cond| matches!(cond, oj_rc_core::persist::config::PitWinCondition::Time)) {
            vec![
                crate::matches::RlnlPacket {
                    event: rlnl::event_code::NetworkEvent::EndGame,
                    property: literustlib::packet::Property::ReliableOrdered,
                    data: Box::new(rlnl::events::ingame::GameEnd {
                        reason: rlnl::types::GameEndReason::TimeOut,
                    }),
                }
            ]
        } else {
            Vec::default()
        };
        let new_timer_task = crate::matches::timer::match_time_syncer(senders, game_start, game_end, Vec::default(), end_packets);
        let mut timer_lock = self.timer_task.lock().await;
        if let Some(timer_t) = &*timer_lock { // this is quite unlikely (i.e. impossible), but I've done it for completeness
            log::warn!("Aborting an existing timer task for pit mode suggests an assumption was wrong");
            timer_t.abort();
        }
        *timer_lock = Some(new_timer_task);
        self.do_leaderboard_update(generic).await;
        true
    }

    async fn on_game_completed(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>) -> bool {
        self.abort_timer_sync().await;
        true
    }

    async fn on_broadcast(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event_out: rlnl::event_code::NetworkEvent, _event_in: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, _data: &Option<Box<dyn crate::Broadcastable>>, _skip_user: bool) -> bool {
        true
    }

    async fn on_motion(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _motion: &rlnl::machine_motion::MachineMotion, _location: (f32, f32, f32)) -> bool {
        if generic.is_game_done() {
            self.abort_timer_sync().await;
            return true;
        }
        if generic.is_game_past_end_time() {
            generic.game_done();
            self.abort_timer_sync().await;
            return true;
        }
        self.win_tracking.tick(generic, self).await;
        true
    }

    async fn on_custom(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, _data: Box<dyn crate::Broadcastable>) {}

    async fn on_spot_vehicle(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _remote_player: u8) -> bool {
        false
    }
}
