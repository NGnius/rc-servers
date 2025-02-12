use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::cpu_limits::CpuLimitsData;

const PARAM_KEY: u8 = 194;

pub(super) fn cpu_config_provider() -> SimpleFunc<75, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, CpuLimitsData {
            premium_for_life_cosmetic_gpu: 12,
            premium_cosmetic_cpu: 6,
            no_premium_cosmetic_cpu: 3,
            max_regular_health: 2_000_000,
            max_megabot_health: 200_000_000,
        }.as_transmissible());
        Ok(params.into())
    })
}
