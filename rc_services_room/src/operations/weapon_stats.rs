use polariton_server::operations::Immediate;
use crate::persist::config::ConfigProvider;

const PARAM_KEY: u8 = 57;

pub(super) fn weapon_config_provider(cubes: &crate::persist::config::ConfigImpl) -> Immediate<47, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(PARAM_KEY, cubes.weapon_list());
        /*params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::HashMap, // hashtable
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
        }));*/
        params.into()
    })
}
