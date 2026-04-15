mod cubes_json;
pub use cubes_json::CubeConfig;

mod traits;
pub use traits::{ConfigProvider, DevMessageProvider, ServerConfig, GarageUpgrades, GarageUpgradeIncrement, ChatSystemConfig, GameEventSequence, GameEvents, GameRotationStrategy, GameEvent, GameMap, GameVisibility, GameType, SingleplayerConfig, VehicleInfo, VehicleDescriptor, QueueChangeMode, Point, Sphere, MapConfig, LinksConfig, FakePlayer, ClientEmulator, EnergyConfig, BattleArenaResolver, PitSettings, PitWinCondition, TeamDeathMatchSettings, ShopEntriesResolver, ShopAction, ShopGain, PromoCode, MultiplayerSettings, BattleArenaCrystalParams, VehicleValidators, TeamChoosers, FactoryConfig, PlatformConfig};

mod validation;
pub use validation::{SelfValidator, ValidationInfo, ValidationMessage};

mod campaign;
pub use campaign::{CampaignResolver, CompleteCampaignProvider};

mod tech_tree;
pub use tech_tree::TechTreeNodeProvider;

pub type ConfigImpl = CubeConfig;

fn __must_impl<T: ConfigProvider<()>>() {}

fn __test_impl() {
    __must_impl::<ConfigImpl>();
}
