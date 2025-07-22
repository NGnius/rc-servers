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
}

pub struct EliminationLogic {
    tracked: PlayerTracker
}

impl EliminationLogic {
    pub fn new() -> Self {
        Self {
            tracked: PlayerTracker {
                alive: tokio::sync::Mutex::new(std::collections::HashMap::new()),
            },
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
        self.tracked.destroy_vehicle(&player.descriptor).await;
        if let Some(winning_team) = self.tracked.winner_team().await {
            log::info!("Team {} has won sudden death game {} because player {} left", winning_team, generic.game_guid, player.descriptor.player_id);
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
                log::info!("Team {} has won sudden death game {}", winning_team, generic.game_guid);
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
                log::info!("Player {} has been destroyed in sudden death game {}", victim, generic.game_guid);
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

        }
        true
    }

    async fn extra_sync_events(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _player: &crate::matches::generic::UserConnection) -> Vec<crate::matches::RlnlPacket> {
        vec![
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::GameModeSettings,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::sync::UpdateGameModeSettings { // FIXME use value from config
                    respawn_heal_duration: 10.0,
                    respawn_full_heal_duration: 10.0,
                }),
            },
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::CurrentGameTime,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::GameTime(300.0)), // FIXME use value from config
            },
        ]
    }
}
