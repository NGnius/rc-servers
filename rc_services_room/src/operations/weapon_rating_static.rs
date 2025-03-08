use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

const PARAM_KEY: u8 = 1;

pub(super) fn weapon_rating_provider() -> SimpleFunc<127, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
            (Typed::Str("subRankCount".into()), Typed::Int(2)),
            (Typed::Str("subRankInterval".into()), Typed::Int(10)),
            (Typed::Str("gainsPerRank".into()), Typed::ObjArr(vec![
                Typed::Dict(Dict {
                    key_ty: TypePrefix::Str,
                    val_ty: TypePrefix::Any,
                    items: vec![
                        (Typed::Str("win".into()), Typed::Int(7)),
                        (Typed::Str("loss".into()), Typed::Int(3)),
                    ],
                }),
                Typed::Dict(Dict {
                    key_ty: TypePrefix::Str,
                    val_ty: TypePrefix::Any,
                    items: vec![
                        (Typed::Str("win".into()), Typed::Int(11)),
                        (Typed::Str("loss".into()), Typed::Int(3)),
                    ],
                })
            ].into())),
        ].into()));
        Ok(params.into())
    })
}
