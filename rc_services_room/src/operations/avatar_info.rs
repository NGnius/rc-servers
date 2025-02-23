use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const IS_CUSTOM_PARAM_KEY: u8 = 130;
const AVATAR_ID_PARAM_KEY: u8 = 129;

pub(super) fn get_avatar_provider() -> SimpleFunc<110, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(IS_CUSTOM_PARAM_KEY, Typed::Bool(false.into()));
        params.insert(AVATAR_ID_PARAM_KEY, Typed::Int(1));
        Ok(params.into())
    })
}
