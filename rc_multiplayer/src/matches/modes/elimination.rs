use crate::matches::CustomGameLogic;

struct PlayerTracker {
    alive: tokio::sync::Mutex<std::collections::HashMap<u8, std::collections::HashSet<u8>>>, // team -> set of player_id
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

    async fn teams(&self) -> std::collections::HashSet<u8> {
        self.alive.lock().await.keys().map(|x| *x).collect()
    }
}

pub struct EliminationLogic {
    tracked: PlayerTracker,
    game_duration: std::time::Duration,
    respawn_full_heal_duration: f32,
    respawn_heal_duration: f32,
    game_end: std::sync::atomic::AtomicI64,
    timer_task: tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl EliminationLogic {
    pub fn new(config: &oj_rc_core::data::game_mode::GameModeConfig) -> Self {
        let dur = std::time::Duration::from_secs((config.game_time_minutes as u64) * 60);
        let fake_end = (chrono::Utc::now() + dur).timestamp();
        Self {
            tracked: PlayerTracker {
                alive: tokio::sync::Mutex::new(std::collections::HashMap::new()),
            },
            game_duration: dur,
            respawn_full_heal_duration: config.respawn_full_heal_duration,
            respawn_heal_duration: config.respawn_heal_duration,
            game_end: std::sync::atomic::AtomicI64::new(fake_end),
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
        if chrono::Utc::now().timestamp() >= self.game_end.load(std::sync::atomic::Ordering::Relaxed) {
            generic.game_done();
            self.abort_timer_sync().await;
            return true;
        }
        self.tracked.destroy_vehicle(&player.descriptor).await;
        if let Some(winning_team) = self.tracked.winner_team().await {
            log::info!("Team {} has won sudden death game {} because player {} left", winning_team, generic.game_guid(), player.descriptor.player_id);
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
            generic.game_done();
            self.abort_timer_sync().await;
        } else if self.tracked.alive_count().await.is_empty() {
            log::debug!("Everyone is dead, so long and thanks for all the fish");
            generic.game_done();
            self.abort_timer_sync().await;
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
                generic.game_done();
            } else {
                log::info!("Player {} has been destroyed in sudden death game {}", victim, generic.game_guid());
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
        } else if self.tracked.alive_count().await.is_empty() {
            log::debug!("Everyone is dead, this must be the West Seth was talking about!");
            generic.game_done();
            self.abort_timer_sync().await;
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

    async fn extra_sync_events(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _player: &crate::matches::generic::UserConnection) -> Vec<crate::matches::RlnlPacket> {
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
                data: Box::new(rlnl::events::GameTime(self.game_duration.as_millis() as f32 / 1000.0)),
            },
        ]
    }

    async fn on_countdown_start(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, game_start: chrono::DateTime<chrono::Utc>) -> bool {
        let mut senders = Vec::new();
        for conn in generic.users.read().await.values() {
            senders.push((conn.connection.clone(), conn.state.clone()));
        }
        let game_end = game_start + self.game_duration;
        let teams = self.tracked.teams().await;
        let extra_packets = teams.iter().map(|team| crate::matches::RlnlPacket {
            event: rlnl::event_code::NetworkEvent::TeamBaseInitialise,
            property: literustlib::packet::Property::ReliableOrdered,
            data: Box::new(rlnl::events::ingame::TeamBaseState {
                base_team_or_mining_point_index: *team,
                current_progress: rlnl::types::ByteFloat::from(0.0),
                max_progress: rlnl::types::ByteFloat::from(4.0),
            }),
        }).collect();
        let new_timer_task = crate::matches::timer::match_time_syncer(senders, game_start, game_end, extra_packets);
        let mut timer_lock = self.timer_task.lock().await;
        if let Some(timer_t) = &*timer_lock { // this is quite unlikely (i.e. impossible), but I've done it for completeness
            log::warn!("Aborting an existing timer task for elimination mode suggests an assumption was wrong");
            timer_t.abort();
        }
        *timer_lock = Some(new_timer_task);
        self.game_end.store(game_end.timestamp(), std::sync::atomic::Ordering::Relaxed);
        true
    }

    async fn on_game_completed(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>) -> bool {
        self.abort_timer_sync().await;
        true
    }

    async fn on_broadcast(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event_out: rlnl::event_code::NetworkEvent, _event_in: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, _data: &Option<Box<dyn crate::Broadcastable>>, _skip_user: bool) -> bool {
        true
    }

    async fn on_motion(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _motion: &rlnl::machine_motion::MachineMotion) -> bool {
        true
    }
}

// spawn points (best guess)
// Mars 1: (16, 0, 19) and (355, 7, 372)
// Earth vanguard 2: (-248, 10, -251) and (267, 10, 258)
