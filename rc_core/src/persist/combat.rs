use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BattleConfig {
    pub regen: AutoRegenHealth,
    pub votes: HashMap<Vote, Vec<VoteThreshold>>,
    #[serde(default = "default_game_modes")]
    pub games: GameModes,
    #[serde(default = "default_campaigns")]
    pub singleplayer: super::SingleplayerConfig,
    #[serde(default = "default_rotation")]
    pub rotation: GameEventSequence,
    #[serde(default = "default_multiplayer")]
    pub multiplayer: super::MultiplayerConfig,
    #[serde(default = "default_maps")]
    pub maps: super::MapsConfig,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameEventSequence {
    pub strategy: GameRotationStrategy,
    pub modes: Vec<GameEvents>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum GameRotationStrategy {
    Sequence,
    Random,
}

impl GameRotationStrategy {
    pub fn into_conf(self) -> crate::persist::config::GameRotationStrategy {
        match self {
            Self::Sequence => crate::persist::config::GameRotationStrategy::Sequence,
            Self::Random => crate::persist::config::GameRotationStrategy::Random,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameEvents {
    pub singleplayer: GameEvent,
    pub multiplayer: GameEvent,
    pub duration_s: u64, // seconds
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameEvent {
    pub map: GameMap,
    pub visibility: GameVisibility,
    pub mode: GameType,
    pub auto_heal: bool,
}

impl GameEvent {
    pub(super) fn into_conf(self) -> crate::persist::config::GameEvent {
        crate::persist::config::GameEvent {
            map: self.map.into_conf(),
            visibility: self.visibility.into_conf(),
            mode: self.mode.into_conf(),
            auto_heal: self.auto_heal,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, Eq, PartialEq)]
pub enum GameMap {
    // TODO put some more obvious aliases on these
    Mars1,
    Mars2,
    Mars3,
    Neptune1,
    Neptune2,
    Neptune3,
    Earth1,
    Earth2,
}

impl GameMap {
    pub(super) fn into_conf(self) -> crate::persist::config::GameMap {
        match self {
            Self::Mars1 => crate::persist::config::GameMap::Mars1,
            Self::Mars2 => crate::persist::config::GameMap::Mars2,
            Self::Mars3 => crate::persist::config::GameMap::Mars3,
            Self::Neptune1 => crate::persist::config::GameMap::Neptune1,
            Self::Neptune2 => crate::persist::config::GameMap::Neptune2,
            Self::Neptune3 => crate::persist::config::GameMap::Neptune3,
            Self::Earth1 => crate::persist::config::GameMap::Earth1,
            Self::Earth2 => crate::persist::config::GameMap::Earth2,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum GameVisibility {
    Good,
    Poor,
    Bad,
}

impl GameVisibility {
    fn into_conf(self) -> crate::persist::config::GameVisibility {
        match self {
            Self::Good => crate::persist::config::GameVisibility::Good,
            Self::Poor => crate::persist::config::GameVisibility::Poor,
            Self::Bad => crate::persist::config::GameVisibility::Bad,
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum GameType {
    BattleArena,
    SuddenDeath,
    Pit,
    TestMode,
    SinglePlayer,
    TeamDeathmatch,
    Campaign,
}

impl GameType {
    fn into_conf(self) -> crate::persist::config::GameType {
        match self {
            Self::BattleArena => crate::persist::config::GameType::BattleArena,
            Self::SuddenDeath => crate::persist::config::GameType::SuddenDeath,
            Self::Pit => crate::persist::config::GameType::Pit,
            Self::TestMode => crate::persist::config::GameType::TestMode,
            Self::SinglePlayer => crate::persist::config::GameType::SinglePlayer,
            Self::TeamDeathmatch => crate::persist::config::GameType::TeamDeathmatch,
            Self::Campaign => crate::persist::config::GameType::Campaign,
        }
    }
}

fn default_game_modes() -> GameModes {
    GameModes {
        battle_arena: GameMode {
            respawn_heal_duration: 10.0,
            respawn_full_heal_duration: 0.5,
            kill_limit: 0,
            game_time_m: 20,
        },
        elimination: GameMode {
            respawn_heal_duration: 10.0,
            respawn_full_heal_duration: 0.5,
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
            respawn_full_heal_duration: 0.5,
            kill_limit: 10,
            game_time_m: 10,
        },
    }
}

pub(super) fn default_campaigns() -> super::SingleplayerConfig {
    super::SingleplayerConfig {
        campaigns: vec![
            super::Campaign {
                id: "strCampaignModeBattle".to_owned(),
                excluded_cubes: Vec::default(),
                categories: vec![super::ItemCategory::Wheel],
                min_cpu: 0,
                max_cpu: 2_000,
                name: "strCampaignModeBattle".to_owned(),
                description: "strCampaignsDesc".to_owned(),
                image: "RE_singleplayer_campaign_image_asset_TODO".to_owned(),
                rules: Vec::default(),
                parameters: Vec::default(),
                difficulties: vec![
                    super::CampaignDifficulty {
                        level: 0,
                        lives: 5,
                        auto_heal: true,
                        single_wave_bonus: 1_000,
                        initial_health_boost: 0.0,
                        health_boost_wave_increase: 0.0,
                        initial_damage_boost: 0.0,
                        damage_boost_wave_increase: 0.0,
                    }
                ],
                completed: vec![
                    super::CampaignCompletion {
                        wave: 0,
                        difficulty: false,
                    }
                ],
                map: "RC_Planet_Neptune_03_BA".to_owned(),
                campaign_type: super::CampaignType::Elimination,
                waves: vec![
                    super::Wave {
                        player_spawn_location: 0,
                        robots_in_wave: vec![
                            super::WaveRobot {
                                name: "strCampaignAnimalName".to_owned(),
                                weapon: "strT5PlasmaGoldenName".to_owned(),
                                movement: "strT5SteeringWheelGoldenName".to_owned(),
                                rank: "strT0".to_owned(),
                                count: 5,
                                robot_data: super::VALID_ROBOT.into(),
                                colour_data: super::VALID_COLOUR.into(),
                                time_to_spawn: 1,
                                kills_to_spawn: 0,
                                time_to_despawn: 60,
                                kills_to_despawn: 1,
                                initial_robot_amount: 0,
                                periodic_robot_amount: 3,
                                spawn_interval: 1,
                                min_robot_amount: 1,
                                max_robot_amount: 5,
                                is_boss: false,
                                is_kill_requirement: true,
                            }
                        ],
                        kill_target: 1,
                        time_min: 1,
                        time_max: 1 * 60,
                    }
                ],
            }
        ],
        vehicles: vec![
            super::PrefabVehicle {
                name: Some("Config your singleplayer!".to_owned()),
                username: "NGnius?".to_owned(),
                id: super::PrefabId::Database { garage: 1 },
            },
            super::PrefabVehicle {
                name: Some("Config your singleplayer!".to_owned()),
                username: "NGram!".to_owned(),
                id: super::PrefabId::Database { garage: 1 },
            },
            super::PrefabVehicle {
                name: Some("Config your singleplayer!".to_owned()),
                username: "NGniusness*".to_owned(),
                id: super::PrefabId::Database { garage: 1 },
            },
            super::PrefabVehicle {
                name: Some("Config your singleplayer!".to_owned()),
                username: "NG~".to_owned(),
                id: super::PrefabId::Database { garage: 1 },
            },
        ],
        max_teammates: 0,
        max_enemies: 5,
    }
}

fn default_rotation() -> GameEventSequence {
    GameEventSequence {
        strategy: GameRotationStrategy::Sequence,
        modes: vec![
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Neptune1,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Neptune3,
                    visibility: GameVisibility::Poor,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                duration_s: 5*60, // 5 minutes
            },
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Neptune2,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Mars1,
                    visibility: GameVisibility::Poor,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                duration_s: 5*60,
            },
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Neptune3,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Earth1,
                    visibility: GameVisibility::Poor,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                duration_s: 5*60,
            },
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Mars1,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Neptune3,
                    visibility: GameVisibility::Poor,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                duration_s: 5*60,
            },
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Mars2,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Mars1,
                    visibility: GameVisibility::Poor,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                duration_s: 5*60,
            },
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Mars3,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Mars1,
                    visibility: GameVisibility::Poor,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                duration_s: 5*60,
            },
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Earth1,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Earth2,
                    visibility: GameVisibility::Poor,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                duration_s: 5*60,
            },
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Earth2,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Mars1,
                    visibility: GameVisibility::Poor,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                duration_s: 5*60,
            },
        ]
    }
}

fn default_multiplayer() -> super::MultiplayerConfig {
    super::MultiplayerConfig {
        players_per_game: 1,
        enabled: true,
        network: super::multiplayer::default_net_conf(),
    }
}

fn default_maps() -> super::MapsConfig {
    super::MapsConfig {
        map: super::maps::default_map(),
    }
}
