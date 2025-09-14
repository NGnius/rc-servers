use crate::matches::CustomGameLogic;

struct PlayerTracker {
    alive: tokio::sync::Mutex<std::collections::HashMap<u8, std::collections::HashSet<u8>>>, // team -> set of player_id
    in_base: tokio::sync::RwLock<std::collections::HashMap<u8, std::sync::atomic::AtomicU16>>, // player_id -> in base state (if base > u8::MAX then not in a base)
}

impl PlayerTracker {
    async fn track_vehicle(&self, player: &oj_rc_core::persist::user::PlayerDescriptor) {
        let mut alive_lock = self.alive.lock().await;
        if let Some(team) = alive_lock.get_mut(&(player.team as u8)) {
            team.insert(player.player_id);
        } else {
            let mut new_team = std::collections::HashSet::new();
            new_team.insert(player.player_id);
            alive_lock.insert(player.team as u8, new_team);
        }
        self.in_base.write().await.insert(player.player_id, std::sync::atomic::AtomicU16::new(u16::MAX));
    }

    async fn destroy_vehicle(&self, player: &oj_rc_core::persist::user::PlayerDescriptor) {
        let mut alive_lock = self.alive.lock().await;
        if let Some(team) = alive_lock.get_mut(&(player.team as u8)) {
            team.remove(&player.player_id);
        } else {
            log::warn!("Destroyed player's vehicle for previously-unseen team");
        }
    }

    async fn winner_team(&self) -> Option<u8> {
        let alive_lock = self.alive.lock().await;
        let mut only_alive_team = None;
        for (team, players) in alive_lock.iter() {
            if !players.is_empty() {
                if only_alive_team.is_some() {
                    // more than one team is alive, game is not over
                    return None;
                } else {
                    only_alive_team = Some(*team);
                }
            }
        }
        only_alive_team
    }

    async fn alive_count(&self) -> std::collections::HashMap<u8, u8> {
        let alive_lock = self.alive.lock().await;
        let mut alive_players = std::collections::HashMap::new();
        for (team, players) in alive_lock.iter() {
            if !players.is_empty() {
                alive_players.insert(*team, players.len() as u8);
            }
        }
        alive_players
    }

    /*async fn teams(&self) -> std::collections::HashSet<u8> {
        self.alive.lock().await.keys().map(|x| *x).collect()
    }*/

    async fn player_team(&self, player_id: u8) -> Option<u8> {
        for (team, players) in self.alive.lock().await.iter() {
            if players.contains(&player_id) {
                return Some(*team);
            }
        }
        None
    }

    async fn swap_is_in_base(&self, player_id: u8, base: Option<u8>) -> Option<u8> {
        self.in_base.read().await.get(&player_id).and_then(|x| {
            let base = x.swap(base.map(|x| x as u16).unwrap_or(u16::MAX), std::sync::atomic::Ordering::Relaxed);
            if base > u8::MAX as u16 {
                None
            } else {
                Some(base as u8)
            }
        })
    }
}

struct BaseCounters {
    enemies: std::sync::atomic::AtomicI16,
    friendlies: std::sync::atomic::AtomicI16,
    capture: atomic_float::AtomicF32,
    percent_per_second: f32,
}

impl BaseCounters {
    fn new(percent_per_second: f32) -> Self {
        Self {
            enemies: std::sync::atomic::AtomicI16::new(0),
            friendlies: std::sync::atomic::AtomicI16::new(0),
            capture: atomic_float::AtomicF32::new(0.0),
            percent_per_second,
        }
    }
}

struct BaseTracker {
    bases: std::collections::HashMap<u8, BaseCounters>,
    ticker: super::trackers::TickTracker<{Self::TICK_MS}>,
    is_baseless: bool,
}

impl BaseTracker {
    const TICK_MS: i64 = 50;
    //const CAPTURE_PER_TICK: f32 = 0.007;

    async fn on_enter(&self, generic: &crate::matches::GenericGamemodeEngine<EliminationLogic>, base_id: u8, is_friendly: bool, player_id: u8) {
        if let Some(base) = self.bases.get(&base_id) {
            if let Some(conn) = generic.users.read().await.get(&player_id) {
                crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                        &rlnl::events::ingame::TeamBaseBoolean {
                        team: base_id,
                        value: 1,
                    },
                    rlnl::event_code::NetworkEvent::PlayerInsideBase,
                    literustlib::packet::Property::ReliableOrdered,
                    &conn.connection.connection,
                ).await);
            } else { return; }
            if is_friendly {
                let friendlies = base.friendlies.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if friendlies == 0 {
                    // newly contesting
                    if base.enemies.load(std::sync::atomic::Ordering::SeqCst) > 0 {
                        generic.broadcast(
                            rlnl::event_code::NetworkEvent::TeamBaseContested,
                            literustlib::packet::Property::ReliableOrdered,
                            &rlnl::events::ingame::TeamBaseBoolean {
                                team: base_id,
                                value: 1,
                            },
                            true
                        ).await;
                        generic.broadcast(
                            rlnl::event_code::NetworkEvent::TeamBaseCaptureStop,
                            literustlib::packet::Property::ReliableOrdered,
                            &rlnl::events::ingame::TeamBaseState {
                                base_team_or_mining_point_index: base_id,
                                current_progress: rlnl::types::ByteFloat::from(base.capture.load(std::sync::atomic::Ordering::SeqCst)),
                                max_progress: rlnl::types::ByteFloat::from(4.0),
                            },
                            true
                        ).await;
                    }
                }
            } else {
                let enemies = base.enemies.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if enemies == 0 {
                    // newly capturing
                    if base.friendlies.load(std::sync::atomic::Ordering::SeqCst) > 0 {
                        // contested
                        generic.broadcast(
                            rlnl::event_code::NetworkEvent::TeamBaseContested,
                            literustlib::packet::Property::ReliableOrdered,
                            &rlnl::events::ingame::TeamBaseBoolean {
                                team: base_id,
                                value: 1,
                            },
                            true
                        ).await;
                    } else {
                        generic.broadcast(
                            rlnl::event_code::NetworkEvent::TeamBaseCaptureStart,
                            literustlib::packet::Property::ReliableOrdered,
                            &rlnl::events::ingame::TeamBaseState {
                                base_team_or_mining_point_index: base_id,
                                current_progress: rlnl::types::ByteFloat::from(base.capture.load(std::sync::atomic::Ordering::SeqCst)),
                                max_progress: rlnl::types::ByteFloat::from(4.0),
                            },
                            true
                        ).await;
                    }
                }
            }
        }
    }

    async fn on_exit(&self, generic: &crate::matches::GenericGamemodeEngine<EliminationLogic>, base_id: u8, is_friendly: bool, player_id: u8) {
        if let Some(base) = self.bases.get(&base_id) {
            if let Some(conn) = generic.users.read().await.get(&player_id) {
                crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                        &rlnl::events::ingame::TeamBaseBoolean {
                        team: base_id,
                        value: 0,
                    },
                    rlnl::event_code::NetworkEvent::PlayerInsideBase,
                    literustlib::packet::Property::ReliableOrdered,
                    &conn.connection.connection,
                ).await);
            } else { return; }
            if is_friendly {
                let friendlies = base.friendlies.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if friendlies <= 1 {
                    // no longer contesting
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::TeamBaseContested,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::TeamBaseBoolean {
                            team: base_id,
                            value: 0,
                        },
                        true
                    ).await;
                    if base.enemies.load(std::sync::atomic::Ordering::SeqCst) > 0 {
                        generic.broadcast(
                            rlnl::event_code::NetworkEvent::TeamBaseCaptureStart,
                            literustlib::packet::Property::ReliableOrdered,
                            &rlnl::events::ingame::TeamBaseState {
                                base_team_or_mining_point_index: base_id,
                                current_progress: rlnl::types::ByteFloat::from(base.capture.load(std::sync::atomic::Ordering::SeqCst)),
                                max_progress: rlnl::types::ByteFloat::from(4.0),
                            },
                            true
                        ).await;
                    }
                }
            } else {
                let enemies = base.enemies.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if enemies <= 1 {
                    // no longer capturing
                    let last_state = base.capture.load(std::sync::atomic::Ordering::SeqCst);
                    let rounded_state = last_state.floor();
                    base.capture.store(rounded_state, std::sync::atomic::Ordering::SeqCst);
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::TeamBaseCaptureStop,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::TeamBaseState {
                            base_team_or_mining_point_index: base_id,
                            current_progress: rlnl::types::ByteFloat::from(last_state),
                            max_progress: rlnl::types::ByteFloat::from(4.0),
                        },
                        true
                    ).await;
                    /*generic.broadcast(
                        rlnl::event_code::NetworkEvent::TeamBaseCaptureReset,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::TeamBaseState {
                            base_team_or_mining_point_index: base_id,
                            current_progress: rlnl::types::ByteFloat::from(rounded_state),
                            max_progress: rlnl::types::ByteFloat::from(4.0),
                        },
                        true
                    ).await;*/
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::TeamBaseState,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::TeamBaseState {
                            base_team_or_mining_point_index: base_id,
                            current_progress: rlnl::types::ByteFloat::from(rounded_state + f32::EPSILON),
                            max_progress: rlnl::types::ByteFloat::from(4.0),
                        },
                        true
                    ).await;
                }
            }
        }
    }

    async fn tick(&self, generic: &crate::matches::GenericGamemodeEngine<EliminationLogic>) {
        let delta = self.ticker.tick();
        if delta == 0 { return; }
        for (team, base) in self.bases.iter() {
            if base.friendlies.load(std::sync::atomic::Ordering::SeqCst) > 0 { continue; }
            if base.enemies.load(std::sync::atomic::Ordering::SeqCst) <= 0 { continue; }
            let to_add = (delta as f32) * (Self::TICK_MS as f32) * base.percent_per_second * 4.0 / (100.0 * 1000.0);
            let pre_add = base.capture.fetch_add(to_add, std::sync::atomic::Ordering::SeqCst);
            let post_add = pre_add + to_add;
            if post_add >= 4.0 {
                log::info!("Team {}'s base was captured in game {}", team, generic.game_guid());
                base.capture.store(4.0, std::sync::atomic::Ordering::SeqCst);
                let data = rlnl::events::ingame::TeamBaseState {
                    base_team_or_mining_point_index: *team,
                    current_progress: rlnl::types::ByteFloat::from(4.0),
                    max_progress: rlnl::types::ByteFloat::from(4.0),
                };
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::TeamBaseFinalSectionComplete,
                    literustlib::packet::Property::ReliableOrdered,
                    &data,
                    true
                ).await;
                let winning_team = if *team == 0 { 1u8 } else { 0u8 };
                let winning_team_i32 = winning_team as i32;
                let win_data = rlnl::events::ingame::GameLoseWin {
                    winning_team,
                    end_reason: rlnl::types::GameEndReason::BaseCaptured,
                };
                for conn in generic.users.read().await.values() {
                    let event = if conn.descriptor.team == winning_team_i32 {
                        rlnl::event_code::NetworkEvent::GameWon
                    } else {
                        rlnl::event_code::NetworkEvent::GameLost
                    };
                    crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                        &win_data,
                        event,
                        literustlib::packet::Property::ReliableOrdered,
                        &conn.connection.connection
                    ).await);
                }
                generic.game_done();
                break;
            } else {
                let data = rlnl::events::ingame::TeamBaseState {
                    base_team_or_mining_point_index: *team,
                    current_progress: rlnl::types::ByteFloat::from(post_add),
                    max_progress: rlnl::types::ByteFloat::from(4.0),
                };
                let is_section_complete = pre_add.floor() < post_add.floor();
                generic.broadcast(
                    if is_section_complete { rlnl::event_code::NetworkEvent::TeamBaseSectionComplete } else { rlnl::event_code::NetworkEvent::TeamBaseState },
                    literustlib::packet::Property::ReliableOrdered,
                    &data,
                    true
                ).await;
            }

        }
    }

    fn teams(&self) -> std::collections::HashSet<u8> {
        self.bases.keys().copied().collect()
    }
}

pub struct EliminationLogic {
    tracked: PlayerTracker,
    bases: BaseTracker,
    respawn_full_heal_duration: f32,
    respawn_heal_duration: f32,
    timer_task: tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl EliminationLogic {
    pub fn new(config: &oj_rc_core::data::game_mode::GameModeConfig, map: &oj_rc_core::persist::config::MapConfig) -> Self {
        Self {
            tracked: PlayerTracker {
                alive: tokio::sync::Mutex::new(std::collections::HashMap::new()),
                in_base: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            },
            bases: BaseTracker {
                bases: map.bases.iter().map(|(team, base)| (*team, BaseCounters::new(base.1))).collect(),
                ticker: super::trackers::TickTracker::new(),
                is_baseless: map.bases.is_empty(),
            },
            respawn_full_heal_duration: config.respawn_full_heal_duration,
            respawn_heal_duration: config.respawn_heal_duration,
            timer_task: tokio::sync::Mutex::new(None),
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

    async fn on_last_player_gone(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, conn: &crate::matches::generic::UserConnection) {
        log::debug!("Everyone is dead, so long and thanks for all the fish");
        generic.game_done();
        self.abort_timer_sync().await;
        let data = rlnl::events::ingame::GameLoseWin {
            winning_team: if conn.descriptor.team == 0 { 1 } else { 0 }, // always the other team
            end_reason: rlnl::types::GameEndReason::NoPlayersRemaining,
        };
        crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
            &data,
            rlnl::event_code::NetworkEvent::GameLost,
            literustlib::packet::Property::ReliableOrdered,
            &conn.connection.connection
        ).await);
    }

    async fn send_win_info(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, winning_team: u8) {
        generic.game_done();
        self.abort_timer_sync().await;
        let data = rlnl::events::ingame::GameLoseWin {
            winning_team,
            end_reason: rlnl::types::GameEndReason::OneTeamRemaining,
        };
        let winning_team_i32 = winning_team as i32;
        for conn in generic.users.read().await.values() {
            let event = if conn.descriptor.team == winning_team_i32 {
                rlnl::event_code::NetworkEvent::GameWon
            } else {
                rlnl::event_code::NetworkEvent::GameLost
            };
            crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                &data,
                event,
                literustlib::packet::Property::ReliableOrdered,
                &conn.connection.connection
            ).await);
        }
    }
}

#[async_trait::async_trait]
impl CustomGameLogic for EliminationLogic {
    async fn on_player_join(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection, _others: &[oj_rc_core::persist::user::PlayerDescriptor]) -> bool {
        self.tracked.track_vehicle(&player.descriptor).await;
        true
    }

    async fn on_player_end(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection) -> bool {
        if generic.is_game_done() {
            return true;
        }
        if chrono::Utc::now().timestamp() >= generic.game_end() {
            generic.game_done();
            self.abort_timer_sync().await;
            return true;
        }
        self.tracked.destroy_vehicle(&player.descriptor).await;
        if let Some(winning_team) = self.tracked.winner_team().await {
            log::info!("Team {} has won sudden death game {} because player {} left", winning_team, generic.game_guid(), player.descriptor.player_id);
            self.send_win_info(generic, winning_team).await;
        } else if self.tracked.alive_count().await.is_empty() {
            self.on_last_player_gone(generic, player).await;
        }
        true
    }

    async fn on_vehicle_destroyed(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _killer: u8, victim: u8) -> bool {
        if let Some(conn) = generic.users.read().await.get(&victim) {
            self.tracked.destroy_vehicle(&conn.descriptor).await;
            let final_score = rlnl::events::ingame::SetFinalGameScore {
                player_id: victim,
                score: 42,
            };
            generic.broadcast(
                rlnl::event_code::NetworkEvent::SetFinalGameScore,
                literustlib::packet::Property::ReliableOrdered,
                &final_score,
                true,
            ).await;
            if let Some(winning_team) = self.tracked.winner_team().await {
                log::info!("Team {} has won sudden death game {}", winning_team, generic.game_guid());
                self.send_win_info(generic, winning_team).await;
            } else {
                log::info!("Player {} has been destroyed in sudden death game {}", victim, generic.game_guid());
                if self.tracked.alive_count().await.is_empty() {
                    self.on_last_player_gone(generic, conn).await;
                }
            }
        }
        true
    }

    async fn on_vehicle_self_destruct(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, user: u8, is_classic: bool) -> bool {
        if is_classic {
            self.on_vehicle_destroyed(generic, u8::MAX, user).await
        } else {
            log::warn!("Received non-elimination self destruct in elimination game mode");
            false
        }
    }

    async fn on_kill_bonus(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _killer: u8, _victim: u8) -> bool {
        true
    }

    async fn extra_sync_events(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _player: &crate::matches::generic::UserConnection) -> Vec<crate::matches::RlnlPacket> {
        vec![
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::GameModeSettings,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::sync::UpdateGameModeSettings {
                    respawn_heal_duration: self.respawn_heal_duration,
                    respawn_full_heal_duration: self.respawn_full_heal_duration,
                }),
            },
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::CurrentGameTime,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::GameTime(generic.game_duration.as_millis() as f32 / 1000.0)),
            },
        ]
    }

    async fn on_countdown_start(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, game_start: chrono::DateTime<chrono::Utc>) -> bool {
        let read_lock = generic.users.read().await;
        let mut senders = Vec::with_capacity(read_lock.len());
        for conn in read_lock.values() {
            senders.push((conn.connection.clone(), conn.state.clone()));
        }
        drop(read_lock);
        let game_end = game_start + generic.game_duration;
        let teams = self.bases.teams();
        let extra_packets = teams.iter().map(|team| crate::matches::RlnlPacket {
            event: rlnl::event_code::NetworkEvent::TeamBaseInitialise,
            property: literustlib::packet::Property::ReliableOrdered,
            data: Box::new(rlnl::events::ingame::TeamBaseState {
                base_team_or_mining_point_index: *team,
                current_progress: rlnl::types::ByteFloat::from(0.0),
                max_progress: rlnl::types::ByteFloat::from(4.0),
            }),
        }).collect();
        let end_packets = vec![
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::EndGame,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::ingame::GameEnd {
                    reason: rlnl::types::GameEndReason::TimeOut,
                }),
            }
        ];
        let new_timer_task = crate::matches::timer::match_time_syncer(senders, game_start, game_end, extra_packets, end_packets);
        let mut timer_lock = self.timer_task.lock().await;
        if let Some(timer_t) = &*timer_lock { // this is quite unlikely (i.e. impossible), but I've done it for completeness
            log::warn!("Aborting an existing timer task for elimination mode suggests an assumption was wrong");
            timer_t.abort();
        }
        *timer_lock = Some(new_timer_task);
        true
    }

    async fn on_game_completed(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>) -> bool {
        self.abort_timer_sync().await;
        true
    }

    async fn on_broadcast(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event_out: rlnl::event_code::NetworkEvent, _event_in: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, _data: &Option<Box<dyn crate::Broadcastable>>, _skip_user: bool) -> bool {
        true
    }

    async fn on_motion(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, motion: &rlnl::machine_motion::MachineMotion, location: (f32, f32, f32)) -> bool {
        if generic.is_game_done() {
            self.abort_timer_sync().await;
            return true;
        }
        if generic.is_game_past_end_time() {
            generic.game_done();
            self.abort_timer_sync().await;
            return true;
        }
        if self.bases.is_baseless {
            return true; // don't bother trying to track whether players are in bases since there are no bases
        }
        // ignore dead or invalid players
        if let Some(player_team) = self.tracked.player_team(motion.player_id).await {
            let mut now_in_base = None;
            for (&team, base) in generic.map_config.bases.iter() {
                if crate::matches::GenericGamemodeEngine::<Self>::is_in(&location, &base.0) {
                    now_in_base = Some(team);
                    break;
                }
            }
            let was_in_base = self.tracked.swap_is_in_base(motion.player_id, now_in_base).await;
            if now_in_base != was_in_base {
                if let Some(was_in_base) = was_in_base {
                    self.bases.on_exit(generic, was_in_base, player_team == was_in_base, motion.player_id).await;
                }
                if let Some(now_in_base) = now_in_base {
                    self.bases.on_enter(generic, now_in_base, player_team == now_in_base, motion.player_id).await;
                }
            }
        }
        self.bases.tick(generic).await;
        true
    }

    async fn on_custom(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, _data: Box<dyn crate::Broadcastable>) {}

    async fn on_spot_vehicle(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _remote_player: u8) -> bool {
        true
    }
}

// spawn points (best guess)
// Mars 1: (16, 0, 19) and (355, 7, 372)
// Earth vanguard 2: (-248, 10, -251) and (267, 10, 258)
