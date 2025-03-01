use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

use crate::data::cube_list::ItemTier;

const PARAM_KEY: u8 = 1;

pub(super) fn weapon_xp_provider() -> SimpleFunc<129, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
            (Typed::Str("maxPower".into()), Typed::Int(2)),
            (Typed::Str("powerLevelsPerTier".into()), Typed::Dict(Dict {
                key_ty: 105, // int
                val_ty: 122, // obj arr
                items: vec![
                    (Typed::Int(ItemTier::T0 as _), Typed::ObjArr(vec![
                        Typed::Dict(Dict {
                            key_ty: 115, // str
                            val_ty: 42, // obj
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(1_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(1_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.1)),
                            ],
                        }),
                        Typed::Dict(Dict {
                            key_ty: 115, // str
                            val_ty: 42, // obj
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(2_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(2_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.2)),
                            ],
                        }),
                    ].into())),
                ],
            })),
        ].into()));
        Ok(params.into())
    })
}
