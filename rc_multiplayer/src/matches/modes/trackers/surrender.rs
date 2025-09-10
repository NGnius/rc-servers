#[repr(u8)]
#[derive(Copy, Clone)]
enum VoteStatus {
    NoEntry = 0,
    Yes = 1,
    No = 2,
}

impl VoteStatus {
    fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::NoEntry,
            1 => Self::Yes,
            2 => Self::No,
            _ => panic!("Invalid vote status variant {}", num),
        }
    }

    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}

struct CurrentVoteInfo {
    current_votes: rlnl::events::ingame::CurrentSurrenderVotes,
    declined: Option<rlnl::events::ingame::SurrenderDeclined>,
    succeeded: bool,
}

struct SurrenderTracker {
    ends_at: chrono::DateTime<chrono::Utc>,
    players: std::collections::HashMap<u8, std::sync::atomic::AtomicU8>,
}

impl SurrenderTracker {
    const DURATION: std::time::Duration = std::time::Duration::from_secs(30);

    fn new(players: impl std::iter::Iterator<Item=u8>, initiator: u8) -> Self {
        let now = chrono::Utc::now();
        Self {
            ends_at: now + Self::DURATION,
            players: players
                .map(|player_id| (player_id, if initiator == player_id {
                    std::sync::atomic::AtomicU8::new(VoteStatus::Yes.to_u8())
                } else {
                    std::sync::atomic::AtomicU8::new(VoteStatus::NoEntry.to_u8())
                }))
                .collect(),
        }
    }

    fn current_votes(&self, player_id: Option<u8>, game_time_elapsed: f32) -> CurrentVoteInfo {
        let mut votes = Vec::with_capacity(self.players.len());
        let mut total_no_votes = 0;
        let mut total_yes_votes = 0;
        for voter in self.players.values() {
            let vote = VoteStatus::from_u8(voter.load(std::sync::atomic::Ordering::Relaxed));
            match vote {
                VoteStatus::NoEntry => {},
                VoteStatus::Yes => {
                    total_yes_votes += 1;
                    votes.push(1);
                },
                VoteStatus::No => {
                    total_no_votes += 1;
                    votes.push(0);
                },
            }
        }
        let threshold = self.players.len() / 2;
        let declined = if total_no_votes > threshold {
            Some(rlnl::events::ingame::SurrenderDeclined {
                surrendering_player_id: player_id.or_else(|| self.players.keys().next().copied()).unwrap_or(0) as i32,
                game_time_elapsed,
            })
        } else {
            None
        };
        CurrentVoteInfo {
            current_votes: rlnl::events::ingame::CurrentSurrenderVotes {
                players_on_team: self.players.len() as i32,
                num_votes: votes.len() as _,
                votes,
            },
            declined,
            succeeded: total_yes_votes > threshold,
        }
    }

    fn vote(&self, player_id: u8, is_surrender: bool, game_time_elapsed: f32) -> Option<CurrentVoteInfo> {
        if let Some(voter) = self.players.get(&player_id) {
            let vote = if is_surrender { VoteStatus::Yes } else { VoteStatus::No };
            voter.store(vote.to_u8(), std::sync::atomic::Ordering::Relaxed);
            Some(self.current_votes(Some(player_id), game_time_elapsed))
        } else {
            None
        }
    }

    fn tick(&self, game_time_elapsed: f32) -> Option<rlnl::events::ingame::SurrenderDeclined> {
        let now = chrono::Utc::now();
        if now > self.ends_at {
            Some(rlnl::events::ingame::SurrenderDeclined {
                surrendering_player_id: 0,
                game_time_elapsed,
            })
        } else {
            None
        }
    }
}

pub struct SurrenderGameTracker {
    surrenders: tokio::sync::RwLock<std::collections::HashMap<u8, SurrenderTracker>>,
    ticker: super::TickTracker<250>,
}

pub enum SurrenderVoteResult {
    Succeeded,
    Declined,
    NoChange,
}

impl SurrenderGameTracker {
    pub fn new() -> Self {
        Self {
            surrenders: tokio::sync::RwLock::new(std::collections::HashMap::with_capacity(2)),
            ticker: super::TickTracker::new(),
        }
    }

    pub async fn vote<L: crate::matches::CustomGameLogic>(&self, team: u8, player_id: u8, is_surrender: bool, generic: &crate::matches::GenericGamemodeEngine<L>) -> SurrenderVoteResult {
        let mut result = SurrenderVoteResult::NoChange;
        if let Some(tracker) = self.surrenders.read().await.get(&team) {
            if let Some(vote_info) = tracker.vote(player_id, is_surrender, generic.elapsed_game_time()) {
                if vote_info.succeeded {
                    result = SurrenderVoteResult::Succeeded;
                } else if vote_info.declined.is_some() {
                    result = SurrenderVoteResult::Declined;
                }
                Self::handle_vote_info(tracker, generic, vote_info, false).await;
            }
        }
        match result {
            SurrenderVoteResult::Succeeded | SurrenderVoteResult::Declined => {
                self.surrenders.write().await.remove(&team);
            },
            _ => {},
        }
        result
    }

    pub async fn request_new<L: crate::matches::CustomGameLogic>(&self, team: u8, initiator: u8, players: impl std::iter::Iterator<Item=u8>, generic: &crate::matches::GenericGamemodeEngine<L>) -> SurrenderVoteResult {
        log::info!("Surrender vote for team {} initiated by player {} in game {}", team, initiator, generic.game_guid());
        let new_tracker = SurrenderTracker::new(players, initiator);
        let vote_info = new_tracker.current_votes(None, generic.elapsed_game_time());
        let result = if vote_info.succeeded {
            SurrenderVoteResult::Succeeded
        } else {
            SurrenderVoteResult::NoChange
        };
        Self::handle_vote_info(&new_tracker, generic, vote_info, true).await;
        self.surrenders.write().await.insert(team, new_tracker);
        result
    }

    pub const fn surrender_times() -> rlnl::events::ingame::SurrenderTimes {
        // TODO make this configurable
        rlnl::events::ingame::SurrenderTimes {
            player_cooldown_seconds: 30,
            team_cooldown_seconds: 30,
            surrender_timeout_seconds: 10,
            initial_surrender_timeout_seconds: 60,
        }
    }

    pub async fn tick<L: crate::matches::CustomGameLogic>(&self, generic: &crate::matches::GenericGamemodeEngine<L>) {
        if self.ticker.tick() == 0 { return; }
        let mut keys_to_remove = Vec::default();
        let generic_user_lock = generic.users.read().await;
        for (team, tracker) in self.surrenders.read().await.iter() {
            if let Some(declined) = tracker.tick(generic.elapsed_game_time()) {
                log::info!("Surrender vote for team {} expired in game {}", team, generic.game_guid());
                for team_member in tracker.players.keys() {
                    if let Some(conn) = generic_user_lock.get(team_member) {
                        crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                            &declined,
                            rlnl::event_code::NetworkEvent::SurrenderDeclined,
                            literustlib::packet::Property::ReliableOrdered,
                            &conn.connection.connection,
                        ).await);
                    }
                }
                keys_to_remove.push(*team);
            }
        }
        if !keys_to_remove.is_empty() {
            let mut tracker_write_lock = self.surrenders.write().await;
            for to_remove in keys_to_remove {
                tracker_write_lock.remove(&to_remove);
            }
        }
    }

    async fn handle_vote_info<L: crate::matches::CustomGameLogic>(tracker: &SurrenderTracker, generic: &crate::matches::GenericGamemodeEngine<L>, vote_info: CurrentVoteInfo, is_new: bool) {
        let generic_user_lock = generic.users.read().await;
        for team_member in tracker.players.keys() {
            if let Some(conn) = generic_user_lock.get(team_member) {
                crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                    &vote_info.current_votes,
                    if is_new { rlnl::event_code::NetworkEvent::SurrenderVoteStarted } else { rlnl::event_code::NetworkEvent::CurrentSurrenderVotes },
                    literustlib::packet::Property::ReliableOrdered,
                    &conn.connection.connection,
                ).await);
            }
        }
        if let Some(declined) = vote_info.declined {
            for team_member in tracker.players.keys() {
                if let Some(conn) = generic_user_lock.get(team_member) {
                    crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                        &declined,
                        rlnl::event_code::NetworkEvent::SurrenderDeclined,
                        literustlib::packet::Property::ReliableOrdered,
                        &conn.connection.connection,
                    ).await);
                }
            }
        }
    }
}
