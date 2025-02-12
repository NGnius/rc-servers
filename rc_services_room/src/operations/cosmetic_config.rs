use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::cosmetic_limits::CosmeticLimitsData;

const PARAM_KEY: u8 = 196;

pub(super) fn cosmetic_limits_config_provider() -> SimpleFunc<72, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, CosmeticLimitsData {
            others_max_holo_and_trails: 16,
            others_max_headlamps: 8,
            others_max_cosmetic_items_with_particles: 12,
        }.as_transmissible());
        Ok(params.into())
    })
}
