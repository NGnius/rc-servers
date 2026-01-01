use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

const PARAM_KEY: u8 = 1;

pub(super) fn player_level_info_provider() -> SimpleFunc<3, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Int, // int
            val_ty: TypePrefix::Int, // int
            items: vec![
                // FIXME load this from config
                // these are interpolated by the client
                (Typed::Int(0), Typed::Int(0)),
                (Typed::Int(10), Typed::Int(10_000)),
                (Typed::Int(10_000), Typed::Int(i32::MAX / 2)),
            ] }));
        Ok(params.into())
    })
}
