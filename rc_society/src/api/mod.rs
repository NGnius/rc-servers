pub mod garage;
pub mod config;
pub mod urls;

pub fn init(config: &dyn oj_rc_core::ConfigProvider<()>) {
    config::init(config);
    urls::init(config);
}
