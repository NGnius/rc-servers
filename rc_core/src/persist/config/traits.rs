use polariton::operation::Typed;

#[async_trait::async_trait]
pub trait ConfigProvider<C: Clone> {
    fn cube_list(&self) -> Typed<C>;
    fn movement_list(&self) -> Typed<C>;
    fn weapon_list(&self) -> Typed<C>;
    fn weapon_upgrade_list(&self) -> Typed<C>;
    fn weapon_keys(&self) -> Typed<C>;
    fn tech_tree_nodes(&self) -> super::TechTreeNodeProvider;
    fn tech_tree_costs(&self) -> std::collections::HashMap<String, u32>; // cube id (hex) -> tech point cost
    fn ids(&self) -> Vec<u32>;
    fn regen_config(&self) -> Typed<C>;
    fn after_battle_vote_config(&self) -> Typed<C>;
    fn game_mode_config(&self) -> Typed<C>;
    fn campaign_details(&self) -> super::CompleteCampaignProvider;
    fn campaigns(&self) -> super::CampaignResolver;
    fn client_config(&self) -> Typed<C>;
    fn login_messages(&self) -> DevMessageProvider<C>;
    fn public_channels(&self) -> Typed<C>;
    fn server_config(&self) -> ServerConfig;
    fn garage_upgrades(&self) -> GarageUpgrades;
    async fn factory(&self, builtin_factory_provider: &(dyn (Fn() -> oj_rc_database::FactoryDatabase) + Sync)) -> Result<crate::factory::Factory, Box<dyn std::error::Error + 'static>>;
    fn cubes(&self) -> &'_ indexmap::IndexMap<String, crate::persist::Cube>;
    fn chat_system_config(&self) -> ChatSystemConfig;
    fn gamemode_events(&self) -> GameEventSequence;
    fn gamemodes(&self) -> crate::data::game_mode::GameModeConfigs; // FIXME
    fn singleplayer_details(&self) -> SingleplayerConfig;
    fn players_per_game(&self) -> usize;
    fn multiplayer_settings(&self) -> MultiplayerSettings;
    // FIXME don't use serializable types in traits
    fn network_config(&self) -> crate::persist::NetworkConf;
    fn maps(&self) -> std::collections::HashMap<GameMap, MapConfig>;
    fn url_links(&self) -> LinksConfig;
    fn fake_players(&self) -> Vec<FakePlayer>;
    fn filler_players(&self) -> Vec<FakePlayer>;
    fn energy(&self) -> EnergyConfig;
    fn ba_settings(&self) -> BattleArenaResolver;
    fn pit_settings(&self) -> PitSettings;
    fn tdm_settings(&self) -> TeamDeathMatchSettings;
    fn shop_entries(&self) -> ShopEntriesResolver;
    fn promo_codes(&self) -> std::collections::HashMap<String, PromoCode>;
    fn vehicle_validation(&self) -> VehicleValidators;
    fn garage_slot_limit(&self) -> i32;
    fn team_choosers(&self) -> TeamChoosers;
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
    pub allow_signup: bool,
    pub queue_mode: QueueChangeMode,
    pub domain: String,
    pub cdn_url: String,
    pub auth_url: String,
    pub intercom_url: String,
    pub minimum_version: i32,
    pub dos_protect: bool,
    pub maintenance_message: Option<String>,
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
    pub default_channel: String,
    pub can_create_channels: bool,
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
                custom_ty: None,
                items: vec![
                    Typed::Str(crate::data::game_mode::GameMap::from_persist(item_now.singleplayer.map).as_str().into()),
                    Typed::Str(crate::data::game_mode::GameMap::from_persist(item_now.multiplayer.map).as_str().into()),
                ],
            }),
            visibilities: Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Int,
                custom_ty: None,
                items: vec![
                    Typed::Int(crate::data::game_mode::MapVisibility::from_persist(item_now.singleplayer.visibility) as _),
                    Typed::Int(crate::data::game_mode::MapVisibility::from_persist(item_now.multiplayer.visibility) as _),
                ],
            }),
            modes: Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Int,
                custom_ty: None,
                items: vec![
                    Typed::Int(crate::data::game_mode::GameMode::from_persist(item_now.singleplayer.mode) as _),
                    Typed::Int(crate::data::game_mode::GameMode::from_persist(item_now.multiplayer.mode) as _),
                ],
            }),
            auto_heals: Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Bool,
                custom_ty: None,
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
        factory: i32,
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
    pub pit_spawns: Vec<Point>,
    pub bases: std::collections::HashMap<u8, (Sphere, f32)>, // team -> (base, capture speed)
    pub capture_points: Vec<(Sphere, f32)>, // (capture point, capture speed)
    pub equalizer: Point,
}

#[derive(Clone, Debug)]
pub struct LinksConfig {
    pub feedback_url: String,
    pub support_url: String,
    pub wiki_url: String,
}

#[derive(Clone, Debug)]
pub struct FakePlayer {
    pub team: Option<u8>,
    pub vehicle: VehicleInfo,
    pub implementation: ClientEmulator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClientEmulator {
    Experiment,
    ClientAI,
}

#[derive(Clone, Debug)]
pub struct EnergyConfig {
    pub refill_rate: f32,
    pub total: u32,
}

pub struct BattleArenaResolver {
    pub(super) data: crate::persist::multiplayer::BattleArenaConfig,
}

impl BattleArenaResolver {
    pub async fn resolve(&self, user: &dyn crate::persist::user::CommonUser, factory: &crate::factory::Factory, weapon_list: &crate::cubes::WeaponListParser, cpu_counter: &crate::cubes::CpuListParser) -> Result<crate::data::battle_arena_config::BattleArenaData, polariton_server::operations::SimpleOpError> {
        let equalizer_data = user.resolve_config_vehicle(&self.data.equalizer.clone().into_conf(), factory, weapon_list, cpu_counter).await?;
        let base_data = user.resolve_config_vehicle(&self.data.base.clone().into_conf(), factory, weapon_list, cpu_counter).await?;
        Ok(crate::data::battle_arena_config::BattleArenaData {
            protonium_health: self.data.crystal_health as i64,
            respawn_time_seconds: self.data.respawn_time_s as i64,
            heal_over_time_per_tower: vec![10, 10, 10, 10], // Unused?
            base_machine_map: base_data.robot_map,
            equalizer_model: equalizer_data.robot_map,
            equalizer_health: self.data.equalizer_health as i64,
            equalizer_trigger_time_seconds: if let Some(trigger_time) = self.data.equalizer_trigger_time_s {
                vec![trigger_time; 5]
            } else {
                Vec::default()
            },
            equalizer_warning_seconds: self.data.equalizer_warning_s as i64, // TODO
            equalizer_duration_seconds: vec![self.data.equalizer_duration_s; 5],
            capture_time_seconds_per_player: vec![30, 20, 10, 5, 1], // TODO
            num_segments: self.data.num_segments as i32,
            heal_escalation_time_seconds: 5, // Unused?
        })
    }

    pub async fn resolve_typed<C>(&self, user: &dyn crate::persist::user::CommonUser, factory: &crate::factory::Factory, weapon_list: &crate::cubes::WeaponListParser, cpu_counter: &crate::cubes::CpuListParser) -> Result<Typed<C>, polariton_server::operations::SimpleOpError> {
        Ok(Typed::Dict(polariton::operation::Dict {
            key_ty: polariton::serdes::TypePrefix::Str, // str
            val_ty: polariton::serdes::TypePrefix::HashMap, // hashmap
            items: vec![
                (Typed::Str("BattleArenaSettings".into()), self.resolve(user, factory, weapon_list, cpu_counter).await?.as_transmissible())
            ],
        }))
    }

    pub fn resolve_base_machine_immediate_early(&self) -> Option<Vec<u8>> {
        match &self.data.base.id {
            crate::persist::PrefabId::Raw { cube_data, .. } => {
                Some(cube_data.to_owned())
            },
            _ => None,
        }
    }

    pub fn crystal_sort_params(&self) -> BattleArenaCrystalParams {
        BattleArenaCrystalParams {
            max_iterations: self.data.max_base_iterations as usize,
            max_random_iterations: self.data.max_base_random_iterations as usize,
        }
    }
}

#[derive(Debug)]
pub struct BattleArenaCrystalParams {
    pub max_iterations: usize,
    pub max_random_iterations: usize,
}

#[derive(Debug)]
pub struct PitSettings {
    pub wins: Vec<PitWinCondition>,
    pub respawn_time_seconds: u64,
}

#[derive(Debug)]
pub enum PitWinCondition {
    StreakKills(u32),
    TotalKills(u32),
    Score(u32),
    Damage(u32),
    Time,
}

#[derive(Debug)]
pub struct TeamDeathMatchSettings {
    pub respawn_time_seconds: u64,
    pub self_destruct_is_kill: bool,
}

pub struct ShopEntriesResolver {
    items: Vec<crate::persist::item_shop::ItemBundle>,
}

pub(super) fn item_shop_sku(i: usize) -> String {
    format!("item-shop-bundle-{}", i)
}

impl ShopEntriesResolver {
    pub async fn resolve_entries<C>(&self, user: &dyn crate::persist::user::User<C>) -> Typed<C> {
        let unlocked_cubes = user.unlocked_parts().await;
        crate::data::item_shop_bundle::ItemShopBundle::as_transmissible_vec(
            self.items.iter().enumerate()
                .map(|(i, entry)| entry.as_data(item_shop_sku(i), &unlocked_cubes))
                .collect()
        )
    }

    // sku -> purchase details
    pub async fn resolve_transactions(&self) -> std::collections::HashMap<String, super::ShopAction> {
        let mut map = std::collections::HashMap::with_capacity(self.items.len());
        let now = chrono::Utc::now().timestamp();
        for (i, entry) in self.items.iter().enumerate() {
            let sku = super::traits::item_shop_sku(i);
            let actual_price = if now > entry.discount_until { entry.discount_price } else { entry.price };
            map.insert(sku, super::ShopAction {
                cost_free: if matches!(entry.currency, crate::persist::item_shop::Currency::Robits) { actual_price } else { 0 },
                cost_paid: if matches!(entry.currency, crate::persist::item_shop::Currency::CosmeticCredits) { actual_price } else { 0 },
                gives: entry.gives.clone().into_iter().map(|x| x.into()).collect(),
            });
        }
        map
    }

    pub(super) fn new(items: Vec<crate::persist::item_shop::ItemBundle>) -> Self {
        Self {
            items,
        }
    }
}

#[derive(Debug)]
pub struct ShopAction {
    pub cost_free: i32,
    pub cost_paid: i32,
    pub gives: Vec<ShopGain>,
}

#[derive(Debug)]
pub enum ShopGain {
    Cube(u32),
    Experience(i64),
    FreeCurrency(i64),
    PaidCurrency(i64),
    TechPoints(i32)
}

#[derive(Debug)]
pub struct PromoCode {
    pub message: Option<String>,
    pub bundle_id: String,
    pub promo_id: String,
    pub is_serial: bool,
    pub is_repeatable: bool,
    pub value: f32,
    pub transaction: ShopAction,
}

#[derive(Debug)]
pub struct MultiplayerSettings {
    pub is_enabled: bool,
    pub lobby_autostart_after: Option<std::time::Duration>,
    pub loading_autostart_after: Option<std::time::Duration>,
}

pub struct VehicleValidators {
    // FIXME don't use serializable types in traits
    pub multiplayer: crate::persist::VehicleValidator,
    pub custom_game: crate::persist::VehicleValidator,
    pub singleplayer: crate::persist::VehicleValidator,
    pub campaigns: std::collections::HashMap<String, crate::persist::VehicleValidator>,
}

pub struct TeamChoosers {
    pub battle_arena: crate::persist::TeamChooser,
    pub elimination: crate::persist::TeamChooser,
    pub pit: crate::persist::TeamChooser,
    pub team_deathmatch: crate::persist::TeamChooser,
}
