use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 71; // long

pub(super) fn user_signup_date_provider() -> SimpleFunc<63, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, user: &crate::UserTy| {
        let mut params = params.to_dict();
        let user = user.user()?;
        params.insert(PARAM_KEY, Typed::Long(user.signup_date()));
        Ok(params.into())
    })
}
