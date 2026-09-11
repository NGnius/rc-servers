pub mod garage;
pub mod config;
pub mod urls;
pub mod user;

pub fn init(config: &dyn oj_rc_core::ConfigProvider<()>) {
    config::init(config);
    urls::init(config);
    garage::init(config);
    user::init(config);
}
