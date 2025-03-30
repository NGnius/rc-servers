use polariton_server::operations::Immediate;

use rc_core::ConfigProvider;

const PARAM_KEY: u8 = 37; // bytes

pub(super) fn auto_regen_config_provider(conf: &rc_core::ConfigImpl) -> Immediate<35, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(PARAM_KEY, conf.regen_config());
        params.into()
    })
}
