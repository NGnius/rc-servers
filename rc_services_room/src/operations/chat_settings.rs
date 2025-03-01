use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 17;

pub(super) fn chat_settings_provider() -> SimpleFunc<18, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
            (Typed::Str("chatEnabled".into()), Typed::Bool(true.into())),
        ].into()));
        Ok(params.into())
    })
}
