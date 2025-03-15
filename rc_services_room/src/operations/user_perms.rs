use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const MOD_PARAM_KEY: u8 = 10;
const DEV_PARAM_KEY: u8 = 11;
const ADM_PARAM_KEY: u8 = 12;

pub(super) fn user_rights_provider() -> SimpleFunc<14, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, user: &crate::UserTy| {
        let lock = user.read().unwrap();
        let user_info = lock.user()?;
        let mut params = params.to_dict();
        params.insert(MOD_PARAM_KEY, Typed::Bool(user_info.is_mod()));
        params.insert(DEV_PARAM_KEY, Typed::Bool(user_info.is_dev()));
        params.insert(ADM_PARAM_KEY, Typed::Bool(user_info.is_admin()));
        Ok(params.into())
    })
}
