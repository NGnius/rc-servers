use polariton::operation::Typed;

#[async_trait::async_trait]
pub trait ConfigProvider<C: Clone> {
    fn cube_list(&self) -> Typed<C>;
    fn movement_list(&self) -> Typed<C>;
    fn weapon_list(&self) -> Typed<C>;
    fn weapon_upgrade_list(&self) -> Typed<C>;
    fn weapon_keys(&self) -> Typed<C>;
    fn tech_tree_nodes(&self, unlocked_cubes: &std::collections::HashSet<u32>) -> Typed<C>;
    fn ids(&self) -> Vec<u32>;
    fn regen_config(&self) -> Typed<C>;
    fn after_battle_vote_config(&self) -> Typed<C>;
    fn game_mode_config(&self) -> Typed<C>;
    fn campaigns_parameters(&self) -> Typed<C>;
    fn campaign_waves(&self) -> Typed<C>;
    fn campaign_version(&self) -> Typed<C>;
    fn campaign_details(&self) -> CompleteCampaignProvider;
    fn client_config(&self) -> Typed<C>;
    fn login_messages(&self) -> DevMessageProvider<C>;
    fn public_channels(&self) -> Typed<C>;
    fn server_config(&self) -> ServerConfig;
    fn garage_upgrades(&self) -> GarageUpgrades;
    async fn factory(&self) -> Result<crate::factory::Factory, Box<dyn std::error::Error + 'static>>;
    fn cubes(&self) -> &'_ std::collections::HashMap<String, crate::persist::Cube>;
    fn chat_system_config(&self) -> ChatSystemConfig;
    fn gamemode_events(&self) -> GameEventSequence;
    fn gamemodes(&self) -> crate::data::game_mode::GameModeConfigs; // FIXME
    fn singleplayer_details(&self) -> SingleplayerConfig;
    fn players_per_game(&self) -> usize;
    fn is_multiplayer_enabled(&self) -> bool;
    // FIXME don't use serializable types in traits
    fn network_config(&self) -> crate::persist::NetworkConf;
    fn maps(&self) -> std::collections::HashMap<GameMap, MapConfig>;
}

pub struct CompleteCampaignProvider {
    map: std::collections::HashMap<String, std::collections::HashMap<i32, crate::data::campaign::CampaignWavesDifficultyData>>,
}

impl CompleteCampaignProvider {
    pub fn new(map: std::collections::HashMap<String, std::collections::HashMap<i32, crate::data::campaign::CampaignWavesDifficultyData>>) -> Self {
        Self { map }
    }

    pub fn get<C>(&self, id: &str, difficulty: &i32) -> Result<Typed<C>, i16> {
        if let Some(campaign) = self.map.get(id) {
            if let Some(details) = campaign.get(difficulty) {
                Ok(details.as_transmissible())
            } else {
                log::warn!("Couldn't find difficulty {} in campaign `{}`", difficulty, id);
                Err(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            }
        } else {
            log::warn!("Couldn't find campaign {} (ignoring difficulty {})", id, difficulty);
            Err(crate::data::error_codes::WebServicesError::DatabaseError as i16)
        }
    }
}

pub struct DevMessageProvider<C: Clone> {
    messages: Vec<TypedDevMessage<C>>,
}

impl <C: Clone> DevMessageProvider<C> {
    pub fn new(messages: Vec<(String, i32)>) -> Self {
        Self {
            messages: messages.into_iter().map(|(msg, time)| {
                let bytes: Vec<u8> = msg.as_bytes().into();
                TypedDevMessage {
                    message: Typed::Bytes(bytes.into()),
                    display_time: Typed::Int(time),
                }
            }
            ).collect(),
        }
    }

    pub fn get(&self, index: usize) -> TypedDevMessage<C> {
        // TODO maybe make this less obtuse -- it works for random, but isn't really obvious for anything else
        if self.messages.is_empty() {
            self.get_empty()
        } else if self.messages.len() == 1 {
            self.messages[0].clone()
        } else {
            let actual_index = index % self.messages.len(); // guarantees index is within allowed range of messages
            self.messages[actual_index].clone()
        }
    }

    pub fn get_empty(&self) -> TypedDevMessage<C> {
        TypedDevMessage {
            message: Typed::Bytes(Vec::default().into()),
            display_time: Typed::Int(-1),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypedDevMessage<C> {
    pub message: Typed<C>,
    pub display_time: Typed<C>,
}

pub struct ServerConfig {
    pub database: String,
    pub auto_signup: bool,
    pub queue_mode: QueueChangeMode,
}

pub enum QueueChangeMode {
    Upgrade, // move enqueued players into newer gamemode
    Notify, // send match change
    Ignore, // do nothing
}

impl QueueChangeMode {
    pub(super) fn from_persist(persist: crate::persist::settings::QueueMode) -> Self {
        match persist {
            crate::persist::settings::QueueMode::Upgrade => Self::Upgrade,
            crate::persist::settings::QueueMode::Notify => Self::Notify,
            crate::persist::settings::QueueMode::Ignore => Self::Ignore,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GarageUpgrades {
    pub increments: Vec<GarageUpgradeIncrement>,
}

#[derive(Clone, Debug)]
pub struct GarageUpgradeIncrement {
    pub cpu: u32,
    pub cost: u32,
}

impl GarageUpgrades {
    pub fn slot_upgrades<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("cpuIncreaseCost".into()), Typed::Dict(polariton::operation::Dict {
                key_ty: polariton::serdes::TypePrefix::Int, // int
                val_ty: polariton::serdes::TypePrefix::Int, // int
                // (CPU limit, upgrade cost)
                items: self.increments.iter().map(|inc| (Typed::Int(inc.cpu as _), Typed::Int(inc.cost as _))).collect(),
            }))
        ].into())
    }
}

#[derive(Clone, Debug)]
pub struct ChatSystemConfig {
    pub command_channel: String,
    pub commands: Vec<crate::persist::ChatCommand>,
}

#[derive(Clone, Debug)]
pub struct GameEventSequence {
    pub strategy: GameRotationStrategy,
    pub modes: Vec<GameEvents>,
    pub index: usize,
    pub started: i64,
    pub(crate) needs_to_be_saved: bool,
}

impl GameEventSequence {
    pub fn now(&mut self, updater: Box<dyn crate::persist::user::GameEventSetter>) -> GameEventTransmissible {
        let time_now = chrono::Utc::now().timestamp();
        let mut item_now = &self.modes[self.index];
        let needs_refresh = time_now >= (item_now.duration.as_secs() as i64) + self.started;
        if self.needs_to_be_saved || needs_refresh {
            if needs_refresh {
                self.index = self.strategy.next(self.index, self.modes.len());
                self.started = time_now;
            } else {
                self.needs_to_be_saved = false;
            }
            item_now = &self.modes[self.index];
            let mp = crate::persist::user::CurrentGameEvent {
                map: crate::data::game_mode::GameMap::from_persist(item_now.multiplayer.map).as_str().to_owned(),
                visibility: crate::data::game_mode::MapVisibility::from_persist(item_now.multiplayer.visibility),
                mode: crate::data::game_mode::GameMode::from_persist(item_now.multiplayer.mode),
                auto_heal: item_now.multiplayer.auto_heal,
                start: self.started,
                end: self.started + item_now.duration.as_secs() as i64,
            };
            let sp = crate::persist::user::CurrentGameEvent {
                map: crate::data::game_mode::GameMap::from_persist(item_now.singleplayer.map).as_str().to_owned(),
                visibility: crate::data::game_mode::MapVisibility::from_persist(item_now.singleplayer.visibility),
                mode: crate::data::game_mode::GameMode::from_persist(item_now.singleplayer.mode),
                auto_heal: item_now.singleplayer.auto_heal,
                start: self.started,
                end: self.started + item_now.duration.as_secs() as i64,
            };
            tokio::spawn(async move {
                updater.set_multiplayer(mp).await;
                updater.set_singleplayer(sp).await;
            });
        }
        let remaining_ticks = ((item_now.duration.as_secs() as i64) - (time_now - self.started)) * 10_000_000;
        GameEventTransmissible {
            maps: Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Str,
                items: vec![
                    Typed::Str(crate::data::game_mode::GameMap::from_persist(item_now.singleplayer.map).as_str().into()),
                    Typed::Str(crate::data::game_mode::GameMap::from_persist(item_now.multiplayer.map).as_str().into()),
                ],
            }),
            visibilities: Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Int,
                items: vec![
                    Typed::Int(crate::data::game_mode::MapVisibility::from_persist(item_now.singleplayer.visibility) as _),
                    Typed::Int(crate::data::game_mode::MapVisibility::from_persist(item_now.multiplayer.visibility) as _),
                ],
            }),
            modes: Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Int,
                items: vec![
                    Typed::Int(crate::data::game_mode::GameMode::from_persist(item_now.singleplayer.mode) as _),
                    Typed::Int(crate::data::game_mode::GameMode::from_persist(item_now.multiplayer.mode) as _),
                ],
            }),
            auto_heals: Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Bool,
                items: vec![
                    Typed::Bool(item_now.singleplayer.auto_heal),
                    Typed::Bool(item_now.multiplayer.auto_heal),
                ],
            }),
            remaining_ticks: Typed::Long(remaining_ticks),
        }
    }
}

pub struct GameEventTransmissible {
    pub maps: Typed,
    pub visibilities: Typed,
    pub modes: Typed,
    pub auto_heals: Typed,
    pub remaining_ticks: Typed,
}

#[derive(Clone, Debug)]
pub enum GameRotationStrategy {
    Sequence,
    Random,
}

impl GameRotationStrategy {
    pub(super) fn next(&self, last: usize, count: usize) -> usize {
        match self {
            Self::Sequence => (last+1) % count,
            Self::Random => {
                use rand::prelude::*;
                let mut rng = rand::rng();
                let num: u64 = rng.random();
                let num = num.clamp(0, usize::MAX as u64) as usize;
                num % count
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameEvents {
    pub singleplayer: GameEvent,
    pub multiplayer: GameEvent,
    pub duration: std::time::Duration,
}

#[derive(Clone, Debug)]
pub struct GameEvent {
    pub map: GameMap,
    pub visibility: GameVisibility,
    pub mode: GameType,
    pub auto_heal: bool,
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum GameMap {
    Mars1,
    Mars2,
    Mars3,
    Neptune1,
    Neptune2,
    Neptune3,
    Earth1,
    Earth2,
}

#[derive(Clone, Debug, Copy)]
pub enum GameVisibility {
    Good,
    Poor,
    Bad,
}


#[derive(Clone, Debug, Copy)]
pub enum GameType {
    BattleArena,
    SuddenDeath,
    Pit,
    TestMode,
    SinglePlayer,
    TeamDeathmatch,
    Campaign,
}

#[derive(Clone, Debug)]
pub struct SingleplayerConfig {
    pub max_teammates: u32,
    pub max_enemies: u32,
    pub vehicles: Vec<VehicleInfo>,
}

#[derive(Clone, Debug)]
pub struct VehicleInfo {
    pub name: Option<String>,
    pub username: String,
    pub id: VehicleDescriptor,
}

#[derive(Clone, Debug)]
pub enum VehicleDescriptor {
    Factory {
        factory: u32,
    },
    Database {
        garage: i32,
    },
    Raw {
        cube_data: Vec<u8>,
        colour_data: Vec<u8>,
    }
    // TODO File
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub radius: f32,
    pub center: Point,
}

#[derive(Clone, Debug)]
pub struct MapConfig {
    pub spawns: std::collections::HashMap<u8, Vec<Point>>, // team -> points
    pub bases: std::collections::HashMap<u8, (Sphere, f32)>, // team -> base
}
