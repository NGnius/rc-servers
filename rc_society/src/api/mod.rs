pub mod garage;
pub mod config;

pub fn init(config: &dyn oj_rc_core::ConfigProvider<()>) {
    config::init(config);
}
