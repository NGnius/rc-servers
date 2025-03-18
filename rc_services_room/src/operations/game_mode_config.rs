use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::persist::config::ConfigProvider;

const PARAM_KEY: u8 = 1;

pub(super) fn game_mode_config_provider(conf: &crate::persist::config::ConfigImpl) -> SimpleFunc<113, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    let game_config = conf.game_mode_config();
    SimpleFunc::new(move |params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, game_config.clone());
        Ok(params.into())
    })
}
