use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

use crate::data::weapon_list::*;
use crate::data::cube_list::ItemTier;

const PARAM_KEY: u8 = 57;

pub(super) fn weapon_config_provider() -> SimpleFunc<47, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 104, // hashtable
            items: vec![
                // (Item category, map<tier, weapon stats>)
                (Typed::Str(ItemCategory::Laser.as_str().into()), Typed::HashMap(vec![
                    (Typed::Str(ItemTier::T0.as_str().into()), WeaponData {
                        damage_inflicted: Some(42),
                        ..Default::default()
                    }.as_transmissible()),
                    (Typed::Str(ItemTier::T1.as_str().into()), WeaponData {
                        damage_inflicted: Some(420),
                        ..Default::default()
                    }.as_transmissible()),
                    (Typed::Str(ItemTier::T2.as_str().into()), WeaponData {
                        damage_inflicted: Some(4200),
                        ..Default::default()
                    }.as_transmissible()),
                    (Typed::Str(ItemTier::T3.as_str().into()), WeaponData {
                        damage_inflicted: Some(42000),
                        ..Default::default()
                    }.as_transmissible()),
                    (Typed::Str(ItemTier::T4.as_str().into()), WeaponData {
                        damage_inflicted: Some(420000),
                        ..Default::default()
                    }.as_transmissible()),
                    (Typed::Str(ItemTier::T5.as_str().into()), WeaponData {
                        damage_inflicted: Some(4200000),
                        ..Default::default()
                    }.as_transmissible()),
                ].into()))
            ].into(),
        }));
        Ok(params.into())
    })
}
