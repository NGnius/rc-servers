use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

const PARAM_KEY: u8 = 1;

pub(super) fn league_battle_parameters_provider() -> SimpleFunc<57, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, //str
            val_ty: 42, // obj
            items: vec![
                (Typed::Str("playerLevelRequired".into()), Typed::Int(10)),
                (Typed::Str("minCpu".into()), Typed::Int(100)),
            ],
        }));
        Ok(params.into())
    })
}
