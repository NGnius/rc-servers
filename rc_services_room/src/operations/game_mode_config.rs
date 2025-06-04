use polariton_server::operations::Immediate;

use oj_rc_core::ConfigProvider;

const PARAM_KEY: u8 = 1;

pub(super) fn game_mode_config_provider(conf: &oj_rc_core::ConfigImpl) -> Immediate<113, crate::UserTy> {
    let game_config = conf.game_mode_config();
    Immediate::new(move || {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(PARAM_KEY, game_config.clone());
        params.into()
    })
}
