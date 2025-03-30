use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

use crate::data::damage_boost::*;

const PARAM_KEY: u8 = 192;

pub(super) fn damage_boost_provider() -> SimpleFunc<163, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::Any, // obj
            items: vec![
                (Typed::Str("damageBoost".into()), DamageBoostData {
                    damage_map: vec![
                        (0, 1.0),
                        (2000, 1.0),
                        (10_000, 1.0),
                    ],
                }.as_transmissible())
            ],
        }));
        Ok(params.into())
    })
}
