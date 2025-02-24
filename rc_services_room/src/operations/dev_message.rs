use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const MESSAGE_PARAM_KEY: u8 = 2;
const DISPLAY_TIME_PARAM_KEY: u8 = 15;

pub(super) fn dev_message_provider() -> SimpleFunc<8, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(MESSAGE_PARAM_KEY, Typed::Bytes(Vec::from("No jam was harmed in the reverse-engineering of this game".as_bytes()).into()));
        params.insert(DISPLAY_TIME_PARAM_KEY, Typed::Int(60 /* seconds??? */));
        Ok(params.into())
    })
}
