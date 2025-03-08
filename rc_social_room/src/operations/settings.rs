use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

const PARAM_KEY: u8 = 30;

pub(super) fn settings_provider<C: Send + Sync>() -> SimpleFunc<24, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: polariton::serdes::TypePrefix::Str,
            val_ty: polariton::serdes::TypePrefix::Any,
            items: Vec::default(),
        }));
        Ok(params.into())
    })
}
