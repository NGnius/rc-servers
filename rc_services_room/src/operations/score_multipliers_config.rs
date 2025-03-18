use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::score_multipliers::*;

const PARAM_KEY: u8 = 137;

pub(super) fn tdm_ai_score_config_provider(/*conf: &crate::persist::config::ConfigImpl*/) -> SimpleFunc<117, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    //let game_config = conf.game_mode_config();
    SimpleFunc::new(move |params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, ScoreMultipliersData::default().as_transmissible());
        Ok(params.into())
    })
}
