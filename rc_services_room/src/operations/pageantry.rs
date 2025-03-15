use polariton_server::operations::Immediate;

use crate::persist::config::ConfigProvider;

const PARAM_KEY: u8 = 203; // bytes

pub(super) fn after_battle_vote_thresholds_provider(conf: &crate::persist::config::ConfigImpl) -> Immediate<169, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(PARAM_KEY, conf.after_battle_vote_config());
        params.into()
    })
}
