use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

use crate::data::damage_boost::*;

const PARAM_KEY: u8 = 192;

pub(super) fn damage_boost_provider() -> SimpleFunc<163, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 42, // obj
            items: vec![
                (Typed::Str("damageBoost".into()), DamageBoostData {
                    damage_map: vec![
                        (100, 1000.0),
                        (1000, 100.0),
                        (2000, 1.0),
                    ],
                }.as_transmissible())
            ],
        }));
        Ok(params.into())
    })
}
