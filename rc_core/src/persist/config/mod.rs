mod cubes_json;
pub use cubes_json::CubeConfig;

mod traits;
pub use traits::{ConfigProvider, CompleteCampaignProvider, DevMessageProvider};

pub type ConfigImpl = CubeConfig;

fn __must_impl<T: ConfigProvider<()>>() {}

fn __test_impl() {
    __must_impl::<ConfigImpl>();
}
