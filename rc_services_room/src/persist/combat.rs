use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BattleConfig {
    pub regen: AutoRegenHealth,
    pub votes: HashMap<Vote, Vec<VoteThreshold>>,
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
