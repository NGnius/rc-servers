use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 8;

pub(super) fn get_user_xp_provider() -> SimpleFunc<83, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Int(31337));
        Ok(params.into())
    })
}
