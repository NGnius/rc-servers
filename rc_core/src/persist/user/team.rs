pub trait TeamChooser: Send + Sync {
    fn choose_team(&self, game: &str, index: usize, player: &super::PlayerLobbyDescriptor) -> i32;
}

pub enum StandardTeamChooser {
    /// Alternating between team 0 and team 1
    Alternating(TwoTeamPartyAware),
    /// All players will be put on the specified team
    AllOn(u8),
    /// Each player will be put on their own team (like in Pit mode)
    OnePer,
    Custom(Box<dyn TeamChooser>),
}

impl StandardTeamChooser {
    pub fn alternating() -> Self {
        Self::Alternating(TwoTeamPartyAware::new())
    }
}

impl TeamChooser for StandardTeamChooser {
    fn choose_team(&self, game: &str, index: usize, player: &super::PlayerLobbyDescriptor) -> i32 {
        match self {
            Self::Alternating(t) => t.choose_team(game, index, player),
            Self::AllOn(team) => *team as i32,
            Self::OnePer => index as i32,
            Self::Custom(t) => t.choose_team(game, index, player),
        }
    }
}

pub struct TwoTeamPartyAware {
    tracker: std::sync::Mutex<std::collections::HashMap<String, BalanceInfo>>,
}

struct BalanceInfo {
    overall: i64,
    /// platoon ID -> team
    platoons: std::collections::HashMap<String, i8>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl TwoTeamPartyAware {
    pub fn new() -> Self {
        Self {
            tracker: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// Manually remove stale team infos
    fn do_cleanup(tracker: &mut std::collections::HashMap<String, BalanceInfo>) {
        let max_ttl = chrono::TimeDelta::from_std(std::time::Duration::from_secs(10)).unwrap();
        let now = chrono::Utc::now();
        tracker.retain(|_, bal_info| (now - bal_info.created_at) < max_ttl);
    }
}

impl TeamChooser for TwoTeamPartyAware {
    fn choose_team(&self, game: &str, _index: usize, player: &super::PlayerLobbyDescriptor) -> i32 {
        let mut lock = self.tracker.lock().unwrap();
        // the algorithm is basically:
        // if player is in platoon AND platoon is known platoon:
        //   use platoon's team assignment
        // else if overall > 0:
        //   choose team 1 for player, remember team assignment for all future platoon members
        //   overall -= 1
        // else:
        //   choose team 0 for player, remember team assignment for all future platoon members
        //   overall += 1
        //
        // this roughly balances player count between the two teams
        // while ensuring players on the same platoon are put on the same team
        if let Some(bal_info) = lock.get_mut(game) {
            if let Some(platoon_id) = &player.group {
                if let Some(&platoon_team) = bal_info.platoons.get(platoon_id) {
                    if platoon_team == 0 {
                        bal_info.overall += 1;
                    } else {
                        bal_info.overall -= 1;
                    }
                    return platoon_team as i32;
                }
            }
            let chosen_team = if bal_info.overall > 0 {
                bal_info.overall -= 1;
                1
            } else {
                bal_info.overall += 1;
                0
            };
            if let Some(platoon_id) = &player.group {
                bal_info.platoons.insert(platoon_id.to_owned(), chosen_team);
            }
            chosen_team as i32
        } else {
            Self::do_cleanup(&mut lock);
            let init_info = if let Some(platoon) = &player.group {
                let mut platoon_teams = std::collections::HashMap::new();
                platoon_teams.insert(platoon.to_owned(), 0);
                BalanceInfo {
                    overall: 1,
                    platoons: platoon_teams,
                    created_at: chrono::Utc::now(),
                }
            } else {
                BalanceInfo {
                    overall: 1,
                    platoons: std::collections::HashMap::new(),
                    created_at: chrono::Utc::now(),
                }
            };
            lock.insert(game.to_owned(), init_info);
            0
        }
    }
}
