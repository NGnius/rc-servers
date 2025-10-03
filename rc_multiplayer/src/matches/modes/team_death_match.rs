use crate::matches::CustomGameLogic;

struct ScoreTracker {
    ticker: super::trackers::TickTracker<{Self::TICK_MS}>,
    scores: std::collections::HashMap<u8, std::sync::atomic::AtomicU32>, // team -> score
    teams: std::collections::HashMap<u8, u8>, // player_id -> team,
    self_destruct_is_kill: bool,
    is_sudden_death: std::sync::atomic::AtomicBool, // triggered if time runs out and both sides are tied
}

impl ScoreTracker {
    const TICK_MS: i64 = 50;

    fn new(players: &[oj_rc_core::persist::user::PlayerDescriptor], self_destruct_is_kill: bool) -> Self {
        Self {
            ticker: super::trackers::TickTracker::new(),
            scores: players.iter().map(|p| (p.player_id, std::sync::atomic::AtomicU32::new(0))).collect(),
            teams: players.iter().map(|p| (p.player_id, p.team as u8)).collect(),
            self_destruct_is_kill,
            is_sudden_death: std::sync::atomic::AtomicBool::new(false),
        }
    }

    fn on_destruction(&self, killer: u8, victim: u8) {
        if let Some(killer_team) = self.teams.get(&killer) {
            if killer == victim {
                if !self.self_destruct_is_kill { return; }
                let other_teams: Vec<u8> = self.scores.keys().copied().filter(|x| x != killer_team).collect();
                let team_to_award = if other_teams.is_empty() {
                    log::warn!("Only one team in team death match, cannot award self-destruct point to other team");
                    return;
                } else if other_teams.len() == 1 {
                    other_teams[0]
                } else {
                    use rand::Rng;
                    let index = rand::rng().random_range(0..other_teams.len());
                    other_teams[index]
                };
                self.scores[&team_to_award].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                log::debug!("Team {} awarded 1 point", team_to_award);
            } else {
                self.scores[killer_team].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                log::debug!("Team {} awarded 1 point", killer_team);
            }
        }
    }

    fn leading_team(&self) -> Option<u8> {
        let mut max = None;
        for (team, score) in self.scores.iter() {
            let score = score.load(std::sync::atomic::Ordering::Relaxed);
            if let Some((_existing_team, existing_score)) = max {
                if existing_score < score {
                    max = Some((*team, score));
                }
            } else {
                max = Some((*team, score));
            }
        }
        max.map(|(team, _score)| team)
    }

    async fn update_team_scores(&self, generic: &crate::matches::GenericGamemodeEngine<TeamDeathMatchLogic>) {
        let packet = rlnl::events::sync::UpdateTeamDeathMatch {
            num_teams: self.scores.len() as i32,
            team_scores: self.scores.iter().map(|(team, score)| rlnl::events::sync::TeamScore {
                team_id: *team as i32,
                score: score.load(std::sync::atomic::Ordering::Relaxed) as i32,
            }).collect(),
            time_expired: if generic.is_game_done() { 1 } else { 0 },
        };
        generic.broadcast(
            rlnl::event_code::NetworkEvent::TeamDeathMatchState,
            literustlib::packet::Property::ReliableOrdered,
            &packet,
            true,
        ).await;
    }

    async fn do_timeout_win(&self, generic: &crate::matches::GenericGamemodeEngine<TeamDeathMatchLogic>) {
        if let Some(winning_team) = self.leading_team() {
            generic.game_done();
            let was_sudden_death = self.is_sudden_death.load(std::sync::atomic::Ordering::Relaxed);
            let data = rlnl::events::ingame::GameLoseWin {
                winning_team,
                end_reason: if was_sudden_death { rlnl::types::GameEndReason::TeamDeathMatchTimeExpiredSuddenDeath } else { rlnl::types::GameEndReason::TeamDeathMatchTimeExpiredMostKills },
            };
            let winning_team_i32 = winning_team as i32;
            for (player_id, player_info) in generic.user_descriptors().iter() {
                let event = if player_info.descriptor.team == winning_team_i32 {
                    rlnl::event_code::NetworkEvent::GameWonBaseDestroyed
                } else {
                    rlnl::event_code::NetworkEvent::GameLostBaseDestroyed
                };
                generic.send_to_player(
                    *player_id,
                    event,
                    literustlib::packet::Property::ReliableOrdered,
                    &data,
                ).await;
            }
        } else {
            self.is_sudden_death.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }

    async fn do_objective_win(&self, generic: &crate::matches::GenericGamemodeEngine<TeamDeathMatchLogic>, winning_team: u8) {
        generic.game_done();
        let data = rlnl::events::ingame::GameLoseWin {
            winning_team,
            end_reason: rlnl::types::GameEndReason::TeamDeathMatchMaxKillsAchieved,
        };
        let winning_team_i32 = winning_team as i32;
        for (player_id, player_info) in generic.user_descriptors().iter() {
            let event = if player_info.descriptor.team == winning_team_i32 {
                rlnl::event_code::NetworkEvent::GameWonBaseDestroyed
            } else {
                rlnl::event_code::NetworkEvent::GameLostBaseDestroyed
            };
            generic.send_to_player(
                *player_id,
                event,
                literustlib::packet::Property::ReliableOrdered,
                &data,
            ).await;
        }
    }

    async fn check_win(&self, generic: &crate::matches::GenericGamemodeEngine<TeamDeathMatchLogic>, kill_limit: u32) {
        for (team, score) in self.scores.iter() {
            if score.load(std::sync::atomic::Ordering::Relaxed) >= kill_limit {
                self.do_objective_win(generic, *team).await;
                break;
            }
        }
    }

    async fn tick(&self, generic: &crate::matches::GenericGamemodeEngine<TeamDeathMatchLogic>, kill_limit: u32) {
        let delta = self.ticker.tick();
        if delta == 0 { return; }
        self.update_team_scores(generic).await;
        self.check_win(generic, kill_limit).await;
    }
}

struct PlayerTracker {
    respawns: std::collections::HashMap<u8, std::sync::atomic::AtomicI64>,
    teams: std::collections::HashMap<u8, u8>, // player_id -> team,
    team_members: std::collections::HashMap<u8, Vec<u8>>, // team -> set of player_id,
}

impl PlayerTracker {
    fn new(players: &[oj_rc_core::persist::user::PlayerDescriptor]) -> Self {
        Self {
            respawns: players.iter().map(|p| (p.player_id, std::sync::atomic::AtomicI64::new(i64::MIN))).collect(),
            teams: players.iter().map(|p| (p.player_id, p.team as u8)).collect(),
            team_members: Self::generate_team_members(players),
        }
    }

    fn generate_team_members(players: &[oj_rc_core::persist::user::PlayerDescriptor]) -> std::collections::HashMap<u8, Vec<u8>> {
        let mut team_map = std::collections::HashMap::<u8, Vec<u8>>::new();
        for user in players.iter() {
            if let Some(member_set) = team_map.get_mut(&(user.team as u8)) {
                member_set.push(user.player_id);
            } else {
                let member_set = vec![user.player_id];
                team_map.insert(user.team as u8, member_set);
            }
        }
        team_map
    }

    async fn single_remaining_team(generic: &crate::matches::GenericGamemodeEngine<TeamDeathMatchLogic>) -> Option<u8> {
        let mut first_remaining_team = None;
        for conn in generic.user_descriptors().values() {
            let mode = crate::matches::generic::ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
            if matches!(mode, crate::matches::generic::ConnectionMode::InGame) {
                if let Some(first_remaining_team) = first_remaining_team {
                    if first_remaining_team != (conn.descriptor.team as u8) {
                        return None;
                    }
                } else {
                    first_remaining_team = Some(conn.descriptor.team as u8);
                }
            }
        }
        first_remaining_team
    }
}

enum WinReason {
    OutOfPlayers,
    Surrender,
}

#[allow(dead_code)]
pub struct TeamDeathMatchLogic {
    respawn_heal_duration: f32,
    respawn_full_heal_duration: f32,
    kill_limit: u32,
    settings: std::sync::Arc<oj_rc_core::persist::config::TeamDeathMatchSettings>,
    timer_task: tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
    score_tracking: ScoreTracker,
    player_tracking: PlayerTracker,
    surrender_tracking: super::trackers::SurrenderGameTracker,
}

impl TeamDeathMatchLogic {
    pub fn new(config: &oj_rc_core::data::game_mode::GameModeConfig, _map: &oj_rc_core::persist::config::MapConfig, players: &[oj_rc_core::persist::user::PlayerDescriptor], tdm_settings: std::sync::Arc<oj_rc_core::persist::config::TeamDeathMatchSettings>) -> Self {
        let self_destruct_is_kill = tdm_settings.self_destruct_is_kill;
        TeamDeathMatchLogic {
            respawn_heal_duration: config.respawn_heal_duration,
            respawn_full_heal_duration: config.respawn_full_heal_duration,
            kill_limit: config.kill_limit as u32,
            settings: tdm_settings,
            timer_task: tokio::sync::Mutex::new(None),
            score_tracking: ScoreTracker::new(players, self_destruct_is_kill),
            player_tracking: PlayerTracker::new(players),
            surrender_tracking: super::trackers::SurrenderGameTracker::new(),
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

    async fn do_win(&self, reason: WinReason, generic: &crate::matches::GenericGamemodeEngine<TeamDeathMatchLogic>, winning_team: u8) {
        generic.game_done();
        let end_reason = match reason {
            WinReason::OutOfPlayers => rlnl::types::GameEndReason::OneTeamRemaining,
            WinReason::Surrender => rlnl::types::GameEndReason::Surrendered,
        };
        let data = rlnl::events::ingame::GameLoseWin {
            winning_team,
            end_reason,
        };
        let winning_team_i32 = winning_team as i32;
        for (player_id, player_info) in generic.user_descriptors().iter() {
            if player_info.descriptor.user_id.is_none() { continue; } // skip non-user players
            let event = if player_info.descriptor.team == winning_team_i32 {
                rlnl::event_code::NetworkEvent::GameWonBaseDestroyed
            } else {
                rlnl::event_code::NetworkEvent::GameLostBaseDestroyed
            };
            generic.send_to_player(
                *player_id,
                event,
                literustlib::packet::Property::ReliableOrdered,
                &data
            ).await;
        }
    }

    async fn do_respawn_tasks(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, player_id: u8) {
        log::info!("Handling respawn player {} in game {}", player_id, generic.game_guid());
        let respawn_time = std::time::Duration::from_secs(self.settings.respawn_time_seconds);
        let now = chrono::Utc::now();
        let respawn_timestamp = now + respawn_time;
        if let Some(player_respawn) = self.player_tracking.respawns.get(&player_id) {
            player_respawn.store(respawn_timestamp.timestamp(), std::sync::atomic::Ordering::Relaxed);
        }
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
        if let Some(user) = generic.user_descriptor(player_id) {
            let player_team = user.descriptor.team as u8;
            let spawn_point = if let Some(team_spawns) = generic.map_config.spawns.get(&player_team) {
                if team_spawns.is_empty() {
                    oj_rc_core::persist::config::Point {
                        x: 10.0 * (player_id as f32),
                        y: 100.0,
                        z: 10.0 * (player_team as f32) + 10.0,
                    }
                } else {
                    let index = (player_id as usize) % team_spawns.len();
                    team_spawns[index].clone()
                }
            } else {
                oj_rc_core::persist::config::Point {
                    x: 10.0 * (player_id as f32),
                    y: 100.0,
                    z: 10.0 * (player_team as f32) + 10.0,
                }
            };
            let connections = generic.users.read().await.values().map(|player_info| player_info.connection.clone()).collect();
            tokio::task::spawn(super::respawn_player_after(respawn_timestamp, connections, spawn_point, player_id));
        } else {
            log::error!("Player {} cannot respawn because they are not connected!?", player_id);
        }
    }
}

#[async_trait::async_trait]
impl CustomGameLogic for TeamDeathMatchLogic {
    async fn on_player_join(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _conn: &crate::matches::generic::UserConnection, _player: &crate::matches::generic::UserDescriptor) -> bool {
        true
    }

    async fn on_player_end(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _connection: &crate::matches::generic::UserConnection, _player: &crate::matches::generic::UserDescriptor) -> bool {
        if generic.is_game_done() {
            return true;
        }
        if let Some(winning_team) = PlayerTracker::single_remaining_team(generic).await {
            log::info!("All players except those on team {} have disconnected, ending game {} early", winning_team, generic.game_guid());
            self.do_win(WinReason::OutOfPlayers, generic, winning_team).await;
        }
        true
    }

    async fn on_vehicle_destroyed(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, killer: u8, victim: u8) -> bool {
        self.do_respawn_tasks(generic, victim).await;
        self.score_tracking.on_destruction(killer, victim);
        true
    }

    async fn on_vehicle_self_destruct(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, user: u8, _is_classic: bool) -> bool {
        self.do_respawn_tasks(generic, user).await;
        self.score_tracking.on_destruction(user, user);
        true
    }

    async fn on_kill_bonus(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _killer: u8, _victim: u8) -> bool {
        true
    }

    async fn extra_sync_events(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _connection: &crate::matches::generic::UserConnection, _player: &crate::matches::generic::UserDescriptor) -> Vec<crate::matches::RlnlPacket> {
        vec![
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::GameModeSettings,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::sync::UpdateTeamDeathmatchSettings {
                    settings: rlnl::types::GameModeSettings {
                        game_time_minutes: (generic.game_duration.as_secs() / 60) as i32,
                        kill_limit: self.kill_limit as i32,
                        respawn_heal_duration: self.respawn_heal_duration,
                        respawn_full_heal_duration: self.respawn_full_heal_duration,
                    }
                }),
            }
        ]
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
        /*let end_packets = vec![
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::EndGame,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::ingame::GameEnd {
                    reason: rlnl::types::GameEndReason::TimeOut,
                }),
            }
        ];*/
        let new_timer_task = crate::matches::timer::match_time_syncer(senders, game_start, game_end, Vec::default(), Vec::default());
        let mut timer_lock = self.timer_task.lock().await;
        if let Some(timer_t) = &*timer_lock { // this is quite unlikely (i.e. impossible), but I've done it for completeness
            log::warn!("Aborting an existing timer task for team death match mode suggests an assumption was wrong");
            timer_t.abort();
        }
        *timer_lock = Some(new_timer_task);
        generic.broadcast(
            rlnl::event_code::NetworkEvent::SetSurrenderTimes,
            literustlib::packet::Property::ReliableOrdered,
            &super::trackers::SurrenderGameTracker::surrender_times(),
            false,
        ).await;
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
            self.score_tracking.do_timeout_win(generic).await;
            self.abort_timer_sync().await;
            return true;
        }
        self.score_tracking.tick(generic, self.kill_limit).await;
        // handle surrender vote tick
        self.surrender_tracking.tick(generic).await;
        true
    }

    async fn on_custom(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, user_id: i32, event: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: Box<dyn crate::Broadcastable>) {
        match (event, property) {
            (rlnl::event_code::NetworkEvent::SurrenderRequest, literustlib::packet::Property::ReliableOrdered) => {
                let maybe_init_surr = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::InitiateSurrender>(data.as_ref());
                if let Some(init_surr) = maybe_init_surr {
                    if let Some(&team) = self.player_tracking.teams.get(&init_surr.player_id) {
                        let team_members = self.player_tracking.team_members[&team].clone();
                        let result = self.surrender_tracking.request_new(team, init_surr.player_id, team_members.into_iter(), generic).await;
                        if matches!(result, super::trackers::SurrenderVoteResult::Succeeded) {
                            let winning_team = if team == 0 { 1 } else { 0 };
                            self.do_win(WinReason::Surrender, generic, winning_team).await;
                        }
                    }
                } else {
                    log::warn!("Bad SurrenderRequest data");
                }
            },
            (rlnl::event_code::NetworkEvent::SurrenderVoteCast, literustlib::packet::Property::ReliableOrdered) => {
                if let Some(player_id) = generic.user_key_by_user_id(user_id) {
                    if let Some(&team) = self.player_tracking.teams.get(&player_id) {
                        let maybe_vote = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::SurrenderVoteCast>(data.as_ref());
                        if let Some(vote) = maybe_vote {
                            let result = self.surrender_tracking.vote(team, player_id, vote.surrender != 0, generic).await;
                            if matches!(result, super::trackers::SurrenderVoteResult::Succeeded) {
                                let winning_team = if team == 0 { 1 } else { 0 };
                                self.do_win(WinReason::Surrender, generic, winning_team).await;
                            }
                        } else {
                            log::warn!("Bad SurrenderVoteCast data");
                        }
                    }
                }
            },
            _ => {}
        }
    }

    async fn on_spot_vehicle(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _remote_player: u8) -> bool {
        true
    }
}
