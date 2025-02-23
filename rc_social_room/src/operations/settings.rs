use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

const PARAM_KEY: u8 = 30;

pub(super) fn settings_provider() -> SimpleFunc<24, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115,
            val_ty: 42,
            items: Vec::default(),
        }));
        Ok(params.into())
    })
}
