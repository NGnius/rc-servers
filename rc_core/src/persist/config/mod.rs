mod cubes_json;
pub use cubes_json::CubeConfig;

mod traits;
pub use traits::{ConfigProvider, CompleteCampaignProvider, DevMessageProvider, ServerConfig, GarageUpgrades, GarageUpgradeIncrement, ChatSystemConfig, GameEventSequence, GameEvents, GameRotationStrategy, GameEvent, GameMap, GameVisibility, GameType, SingleplayerConfig, VehicleInfo, VehicleDescriptor, QueueChangeMode, Point, Sphere, MapConfig, LinksConfig};

pub type ConfigImpl = CubeConfig;

fn __must_impl<T: ConfigProvider<()>>() {}

fn __test_impl() {
    __must_impl::<ConfigImpl>();
}
