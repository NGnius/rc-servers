pub mod federation;

pub(super) fn init(config: &dyn oj_rc_core::ConfigProvider<()>) {
    federation::init(config);
}
