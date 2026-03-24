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
    #[serde(default = "default_energy")]
    pub energy: EnergyConfig,
}

impl super::config::SelfValidator for BattleConfig {
    type Context = crate::ConfigImpl;
    fn validate(&self, info: &mut super::config::ValidationInfo, ctx: &Self::Context) -> bool {
        let mut is_ok = true;
        // TODO regen
        // TODO votes
        // TODO games
        is_ok &= self.singleplayer.validate_in(info, ctx, "singleplayer");
        is_ok &= self.rotation.validate_in(info, self, "rotation");
        // TODO multiplayer
        // TODO maps
        // TODO energy
        is_ok
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutoRegenHealth {
    pub wait_for_heal_s: f32,
    pub wait_full_heal_s: f32,
    pub sound_start_s: f32,
    pub auto_heal: bool,
}

impl std::convert::From<AutoRegenHealth> for crate::data::auto_regen::AutoRegenHealthConfig {
    fn from(val: AutoRegenHealth) -> Self {
        crate::data::auto_regen::AutoRegenHealthConfig {
            seconds_to_wait_for_heal: val.wait_for_heal_s,
            seconds_to_full_heal: val.wait_full_heal_s,
            threshold_to_start_sound: val.sound_start_s,
            enable_auto_heal: val.auto_heal,
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

impl std::convert::From<VoteThreshold> for crate::data::voting::VoteThresholdData {
    fn from(val: VoteThreshold) -> Self {
        crate::data::voting::VoteThresholdData {
            name: val.name,
            localised_name: val.localised_name,
            color: val.color,
            votes_required: val.votes_required,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Vote {
    BestPlayed,
    BestLooking,
}

impl std::convert::From<Vote> for crate::data::voting::Vote {
    fn from(val: Vote) -> Self {
        match val {
            Vote::BestPlayed => crate::data::voting::Vote::BestPlayed,
            Vote::BestLooking => crate::data::voting::Vote::BestLooking,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameMode {
    pub respawn_heal_duration: f32,
    pub respawn_full_heal_duration: f32,
    pub kill_limit: i32,
    pub game_time_m: i32,
    pub team_chooser: super::TeamChooser,
}

impl std::convert::From<GameMode> for crate::data::game_mode::GameModeConfig {
    fn from(val: GameMode) -> Self {
        crate::data::game_mode::GameModeConfig {
            respawn_heal_duration: val.respawn_heal_duration,
            respawn_full_heal_duration: val.respawn_full_heal_duration,
            kill_limit: val.kill_limit,
            game_time_minutes: val.game_time_m,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameModes {
    pub battle_arena: GameMode,
    pub elimination: GameMode,
    pub pit: GameMode,
    pub team_deathmatch: GameMode,
}

impl std::convert::From<GameModes> for crate::data::game_mode::GameModeConfigs {
    fn from(val: GameModes) -> Self {
        crate::data::game_mode::GameModeConfigs {
            battle_arena: val.battle_arena.into(),
            elimination: val.elimination.into(),
            the_pit: val.pit.into(),
            team_deathmatch: val.team_deathmatch.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameEventSequence {
    pub strategy: GameRotationStrategy,
    pub modes: Vec<GameEvents>,
}

impl super::config::SelfValidator for GameEventSequence {
    type Context = BattleConfig;
    fn validate(&self, info: &mut super::config::ValidationInfo, ctx: &Self::Context) -> bool {
        // TODO
        let mut is_ok = true;
        if self.modes.is_empty() {
            info.error(super::config::ValidationMessage {
                path: vec!["modes".to_owned()],
                message: "Game sequence must have at least one mode (event) in the rotation".to_owned(),
            });
        }
        for (i, mode) in self.modes.iter().enumerate() {
            is_ok &= mode.validate_in(info, ctx, &format!("modes[{}]", i));
        }
        is_ok
    }
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

impl super::config::SelfValidator for GameEvents {
    type Context = BattleConfig;
    fn validate(&self, info: &mut super::config::ValidationInfo, ctx: &Self::Context) -> bool {
        // TODO
        let mut is_ok = true;
        if !matches!(self.singleplayer.mode, GameType::SuddenDeath | GameType::SinglePlayer) {
            info.warn(crate::persist::config::ValidationMessage {
                path: vec!["singleplayer".to_owned(), "mode".to_owned()],
                message: format!("Singleplayer game mode {:?} will be overidden by the client", self.singleplayer.mode),
            });
        }
        if self.duration_s == 0 {
            info.error(crate::persist::config::ValidationMessage {
                path: vec!["duration_s".to_owned()],
                message: "Duration cannot be zero".to_owned(),
            });
            is_ok = false;
        }
        if ctx.multiplayer.enabled {
            if matches!(self.multiplayer.mode, GameType::Pit) {
                if ctx.multiplayer.fakes.iter().any(|f| f.team.is_some_and(|t| (t as usize) < ctx.multiplayer.players_per_game))
                    || ctx.multiplayer.fakes.iter().enumerate()
                        .any(|(i, f)| ctx.multiplayer.fakes.iter().enumerate()
                            .any(|(i2, f2)| i != i2 && f.team == f2.team)) {
                    info.warn(crate::persist::config::ValidationMessage {
                        path: vec!["multiplayer".to_owned(), "mode".to_owned()],
                        message: format!("Multiplayer game mode {:?} does not work well with more than one (fake) player per team", self.multiplayer.mode),
                    });
                }
                if ctx.multiplayer.fakes.iter().any(|f| matches!(f.implementation, super::multiplayer::ClientEmulation::ClientAI)) {
                    info.warn(crate::persist::config::ValidationMessage {
                        path: vec!["multiplayer".to_owned(), "mode".to_owned()],
                        message: format!("Multiplayer game mode {:?} does not work well with client-side AI", self.multiplayer.mode),
                    });
                }
            } else {
                if ctx.multiplayer.fakes.iter().any(|f| f.team.is_some_and(|t| t > 1)) {
                    info.warn(crate::persist::config::ValidationMessage {
                        path: vec!["multiplayer".to_owned(), "mode".to_owned()],
                        message: format!("Multiplayer game mode {:?} does not work well with (fake) players split between more than 2 teams", self.multiplayer.mode),
                    });
                }
                if !ctx.maps.map.contains_key(&self.multiplayer.map) {
                    info.warn(crate::persist::config::ValidationMessage {
                        path: vec!["multiplayer".to_owned(), "map".to_owned()],
                        message: format!("Multiplayer game map {:?} is not configured", self.multiplayer.map),
                    });
                }
            }
        }
        is_ok
    }
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
    #[serde(alias = "Hellion", alias = "HellionCrater")]
    Mars1,
    #[serde(alias = "Tihonium", alias = "TihoniumCanyon")]
    Mars2,
    #[serde(alias = "Tharsis", alias = "TharsisRift")]
    Mars3,
    #[serde(alias = "Gliese", alias = "GlieseLake")]
    Neptune1,
    #[serde(alias = "Ophiuchus", alias = "OphiuchusValley")]
    Neptune2,
    #[serde(alias = "Spitzer", alias = "SpitzerDam")]
    Neptune3,
    #[serde(alias = "Birmingham", alias = "BirminghamPowerStation")]
    Earth1,
    #[serde(alias = "Vanguard", alias = "VanguardsEnd")]
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

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct EnergyConfig {
    pub refill_rate_per_s: f32,
    pub total: u32,
}

fn default_game_modes() -> GameModes {
    GameModes {
        battle_arena: GameMode {
            respawn_heal_duration: 10.0,
            respawn_full_heal_duration: 0.5,
            kill_limit: 0,
            game_time_m: 20,
            team_chooser: super::TeamChooser::Alternating,
        },
        elimination: GameMode {
            respawn_heal_duration: 10.0,
            respawn_full_heal_duration: 0.5,
            kill_limit: 10,
            game_time_m: 10,
            team_chooser: super::TeamChooser::Alternating,
        },
        pit: GameMode {
            respawn_heal_duration: 20.0,
            respawn_full_heal_duration: 0.5,
            kill_limit: 0,
            game_time_m: 15,
            team_chooser: super::TeamChooser::OneOnAll,
        },
        team_deathmatch: GameMode {
            respawn_heal_duration: 10.0,
            respawn_full_heal_duration: 0.5,
            kill_limit: 10,
            game_time_m: 10,
            team_chooser: super::TeamChooser::Alternating,
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
                                vehicle: super::PrefabVehicle {
                                    name: Some("strCampaignAnimalName".to_owned()),
                                    username: "[ignored]".to_owned(),
                                    id: super::PrefabId::Raw {
                                        cube_data: super::VALID_ROBOT.into(),
                                        colour_data: super::VALID_COLOUR.into(),
                                    },
                                },
                                weapon: "strT5PlasmaGoldenName".to_owned(),
                                movement: "strT5SteeringWheelGoldenName".to_owned(),
                                rank: "strT0".to_owned(),
                                count: 5,
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
                        time_max: 60,
                    }
                ],
                vehicle_validator: super::VehicleValidator::None,
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
        vehicle_validator: super::VehicleValidator::None,
    }
}

fn default_rotation() -> GameEventSequence {
    GameEventSequence {
        strategy: GameRotationStrategy::Sequence,
        modes: vec![
            // dev mode sequence
            /*GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Earth1,
                    visibility: GameVisibility::Good,
                    mode: GameType::BattleArena,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Earth1,
                    visibility: GameVisibility::Good,
                    mode: GameType::BattleArena,
                    auto_heal: true,
                },
                duration_s: 30,
            },
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Earth1,
                    visibility: GameVisibility::Good,
                    mode: GameType::BattleArena,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Earth2,
                    visibility: GameVisibility::Good,
                    mode: GameType::BattleArena,
                    auto_heal: true,
                },
                duration_s: 30, // 30 seconds
            },*/
            // release sequence
            GameEvents {
                singleplayer: GameEvent {
                    map: GameMap::Neptune1,
                    visibility: GameVisibility::Good,
                    mode: GameType::SuddenDeath,
                    auto_heal: true,
                },
                multiplayer: GameEvent {
                    map: GameMap::Earth1,
                    visibility: GameVisibility::Poor,
                    mode: GameType::Pit,
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
                    map: GameMap::Earth2,
                    visibility: GameVisibility::Good,
                    mode: GameType::TeamDeathmatch,
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
                    map: GameMap::Mars1,
                    visibility: GameVisibility::Poor,
                    mode: GameType::BattleArena,
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
                    map: GameMap::Mars2,
                    visibility: GameVisibility::Good,
                    mode: GameType::TeamDeathmatch,
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
                    map: GameMap::Mars3,
                    visibility: GameVisibility::Poor,
                    mode: GameType::BattleArena,
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
                    map: GameMap::Neptune1,
                    visibility: GameVisibility::Good,
                    mode: GameType::TeamDeathmatch,
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
                    map: GameMap::Neptune2,
                    visibility: GameVisibility::Poor,
                    mode: GameType::BattleArena,
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
                    map: GameMap::Neptune3,
                    visibility: GameVisibility::Good,
                    mode: GameType::TeamDeathmatch,
                    auto_heal: true,
                },
                duration_s: 5*60,
            },
        ]
    }
}

fn default_multiplayer() -> super::MultiplayerConfig {
    super::MultiplayerConfig {
        players_per_game: 2,
        enabled: true,
        autostart_after_s: super::multiplayer::default_match_autostart_after_s(),
        continue_loading_after_s: super::multiplayer::default_continue_loading_after_s(),
        network: super::multiplayer::default_net_conf(),
        fakes: super::multiplayer::default_fake_users(),
        filler: super::multiplayer::default_filler_users(),
        battle_arena: super::multiplayer::default_ba_conf(),
        pit_config: super::multiplayer::default_pit_conf(),
        team_death_match: super::multiplayer::default_tdm_conf(),
        vehicle_validator: super::multiplayer::default_validator(),
    }
}

fn default_maps() -> super::MapsConfig {
    super::MapsConfig {
        map: super::maps::default_map(),
    }
}

fn default_energy() -> EnergyConfig {
    EnergyConfig {
        refill_rate_per_s: 0.1, // should take 10s to refill
        total: 12_550,
    }
}
