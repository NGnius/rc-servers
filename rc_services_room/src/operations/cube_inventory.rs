use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

const PARAM_KEY: u8 = 16;

pub(super) fn cube_inv_provider() -> SimpleFunc<16, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 105, // int
            val_ty: 105, // int
            items: vec![
                (Typed::Int(0), Typed::Int(99)),
            ] }));
        Ok(params.into())
    })
}
