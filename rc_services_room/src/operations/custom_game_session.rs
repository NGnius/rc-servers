use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const RESPONSE_CODE_PARAM_KEY: u8 = 168;
//const CUSTOM_GAME_DATA_PARAM_KEY: u8 = 169;

pub(super) fn get_custom_session_provider() -> SimpleFunc<144, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(0 /* Not in any session */));
        Ok(params.into())
    })
}
