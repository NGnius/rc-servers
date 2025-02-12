use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::premium_config::*;

const PARAM_KEY: u8 = 1;

pub(super) fn premium_config_provider() -> SimpleFunc<5, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, PremiumEffects {
            factor: PremiumFactor {
                factor: 100,
                party_bonus: 100,
            },
            multiplayer: PremiumMultiplayer {
                tier_multiplier: 2.0,
            }
        }.as_transmissible());
        Ok(params.into())
    })
}
