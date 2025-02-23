use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const MOD_PARAM_KEY: u8 = 10;
const DEV_PARAM_KEY: u8 = 11;
const ADM_PARAM_KEY: u8 = 12;

pub(super) fn user_rights_provider() -> SimpleFunc<14, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(MOD_PARAM_KEY, Typed::Bool(false.into()));
        params.insert(DEV_PARAM_KEY, Typed::Bool(false.into()));
        params.insert(ADM_PARAM_KEY, Typed::Bool(false.into()));
        Ok(params.into())
    })
}
