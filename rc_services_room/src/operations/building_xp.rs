use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PROGRESS_PARAM_KEY: u8 = 199;
const LEVEL_PARAM_KEY: u8 = 81;
const GAINED_XP_PARAM_KEY: u8 = 205;

pub(super) fn building_xp_save_provider() -> SimpleFunc<170, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PROGRESS_PARAM_KEY, Typed::Float(1.0));
        params.insert(LEVEL_PARAM_KEY, Typed::Int(31337));
        params.insert(GAINED_XP_PARAM_KEY, Typed::Int(0));
        Ok(params.into())
    })
}
