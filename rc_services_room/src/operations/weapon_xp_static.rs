use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

use crate::data::cube_list::ItemTier;

const PARAM_KEY: u8 = 1;

pub(super) fn weapon_xp_provider() -> SimpleFunc<129, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
            (Typed::Str("maxPower".into()), Typed::Int(2)),
            (Typed::Str("powerLevelsPerTier".into()), Typed::Dict(Dict {
                key_ty: TypePrefix::Int, // int
                val_ty: TypePrefix::ObjArr, // obj arr
                items: vec![
                    (Typed::Int(ItemTier::T0 as _), Typed::ObjArr(vec![
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(1_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(1_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.1)),
                            ],
                        }),
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(2_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(2_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.2)),
                            ],
                        }),
                    ].into())),
                    (Typed::Int(ItemTier::T1 as _), Typed::ObjArr(vec![
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(1_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(1_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.1)),
                            ],
                        }),
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(2_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(2_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.2)),
                            ],
                        }),
                    ].into())),
                    (Typed::Int(ItemTier::T2 as _), Typed::ObjArr(vec![
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(1_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(1_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.1)),
                            ],
                        }),
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(2_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(2_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.2)),
                            ],
                        }),
                    ].into())),
                    (Typed::Int(ItemTier::T3 as _), Typed::ObjArr(vec![
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(1_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(1_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.1)),
                            ],
                        }),
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(2_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(2_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.2)),
                            ],
                        }),
                    ].into())),
                    (Typed::Int(ItemTier::T4 as _), Typed::ObjArr(vec![
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(1_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(1_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.1)),
                            ],
                        }),
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(2_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(2_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.2)),
                            ],
                        }),
                    ].into())),
                    (Typed::Int(ItemTier::T5 as _), Typed::ObjArr(vec![
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
                            items: vec![
                                (Typed::Str("xp".into()), Typed::Int(1_000)),
                                (Typed::Str("costRobits".into()), Typed::Int(1_000)),
                                (Typed::Str("damageMultiplier".into()), Typed::Float(1.1)),
                            ],
                        }),
                        Typed::Dict(Dict {
                            key_ty: TypePrefix::Str,
                            val_ty: TypePrefix::Any,
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
