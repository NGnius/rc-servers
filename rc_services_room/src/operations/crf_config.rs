use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::crf_config::*;

const PARAM_KEY: u8 = 110;

pub(super) fn crf_config_provider() -> SimpleFunc<92, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, RobotShopConfig {
            cpu_ranges: vec![100, 500, 1_000, 2_000],
            submission_mult: 1.0,
            earnings_mult: 1.0,
        }.as_transmissible());
        Ok(params.into())
    })
}
