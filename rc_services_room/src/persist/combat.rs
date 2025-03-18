use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BattleConfig {
    pub regen: AutoRegenHealth,
    pub votes: HashMap<Vote, Vec<VoteThreshold>>,
    #[serde(default = "default_game_modes")]
    pub games: GameModes,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutoRegenHealth {
    pub wait_for_heal_s: f32,
    pub wait_full_heal_s: f32,
    pub sound_start_s: f32,
    pub auto_heal: bool,
}

impl std::convert::Into<crate::data::auto_regen::AutoRegenHealthConfig> for AutoRegenHealth {
    fn into(self) -> crate::data::auto_regen::AutoRegenHealthConfig {
        crate::data::auto_regen::AutoRegenHealthConfig {
            seconds_to_wait_for_heal: self.wait_for_heal_s,
            seconds_to_full_heal: self.wait_full_heal_s,
            threshold_to_start_sound: self.sound_start_s,
            enable_auto_heal: self.auto_heal,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoteThreshold {
    pub name: String,
    pub localised_name: String,
    pub color: String,
    pub votes_required: i32,
}

impl std::convert::Into<crate::data::voting::VoteThresholdData> for VoteThreshold {
    fn into(self) -> crate::data::voting::VoteThresholdData {
        crate::data::voting::VoteThresholdData {
            name: self.name,
            localised_name: self.localised_name,
            color: self.color,
            votes_required: self.votes_required,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Vote {
    BestPlayed,
    BestLooking,
}

impl std::convert::Into<crate::data::voting::Vote> for Vote {
    fn into(self) -> crate::data::voting::Vote {
        match self {
            Self::BestPlayed => crate::data::voting::Vote::BestPlayed,
            Self::BestLooking => crate::data::voting::Vote::BestLooking,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct GameMode {
    pub respawn_heal_duration: f32,
    pub respawn_full_heal_duration: f32,
    pub kill_limit: i32,
    pub game_time_m: i32,
}

impl std::convert::Into<crate::data::game_mode::GameModeConfig> for GameMode {
    fn into(self) -> crate::data::game_mode::GameModeConfig {
        crate::data::game_mode::GameModeConfig {
            respawn_heal_duration: self.respawn_heal_duration,
            respawn_full_heal_duration: self.respawn_full_heal_duration,
            kill_limit: self.kill_limit,
            game_time_minutes: self.game_time_m,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct GameModes {
    pub battle_arena: GameMode,
    pub elimination: GameMode,
    pub pit: GameMode,
    pub team_deathmatch: GameMode,
}

impl std::convert::Into<crate::data::game_mode::GameModeConfigs> for GameModes {
    fn into(self) -> crate::data::game_mode::GameModeConfigs {
        crate::data::game_mode::GameModeConfigs {
            battle_arena: self.battle_arena.into(),
            elimination: self.elimination.into(),
            the_pit: self.pit.into(),
            team_deathmatch: self.team_deathmatch.into(),
        }
    }
}

fn default_game_modes() -> GameModes {
    GameModes {
        battle_arena: GameMode {
            respawn_heal_duration: 10.0,
            respawn_full_heal_duration: 10.0,
            kill_limit: 0,
            game_time_m: 20,
        },
        elimination: GameMode {
            respawn_heal_duration: 10.0,
            respawn_full_heal_duration: 10.0,
            kill_limit: 10,
            game_time_m: 10,
        },
        pit: GameMode {
            respawn_heal_duration: 20.0,
            respawn_full_heal_duration: 20.0,
            kill_limit: 15,
            game_time_m: 15,
        },
        team_deathmatch: GameMode {
            respawn_heal_duration: 10.0,
            respawn_full_heal_duration: 10.0,
            kill_limit: 10,
            game_time_m: 10,
        },
    }
}
