use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

const PARAM_KEY: u8 = 1;

pub(super) fn league_battle_parameters_provider() -> SimpleFunc<57, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, //str
            val_ty: TypePrefix::Any, // obj
            items: vec![
                (Typed::Str("playerLevelRequired".into()), Typed::Int(10)),
                (Typed::Str("minCpu".into()), Typed::Int(100)),
            ],
        }));
        Ok(params.into())
    })
}
