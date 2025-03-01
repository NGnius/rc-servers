use polariton::operation::{Typed, Dict};

use super::{cube_list::ItemTier, weapon_list::ItemCategory};

pub struct WeaponUpgradeInfo {
    pub tier: ItemTier,
    pub type_: ItemCategory,
    pub xp: f64,
    pub rating: i32,
    pub rank: i32,
    pub power: i32,
}

impl WeaponUpgradeInfo {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 42, // obj
            items: vec![
                (Typed::Str("weaponSize".into()), Typed::Int(self.tier as _)),
                (Typed::Str("weaponType".into()), Typed::Int(self.type_ as _)),
                (Typed::Str("weaponXp".into()), Typed::Double(self.xp)),
                (Typed::Str("weaponRating".into()), Typed::Int(self.rating)),
                (Typed::Str("weaponRank".into()), Typed::Int(self.rank)),
                (Typed::Str("weaponPower".into()), Typed::Int(self.power)),
            ],
        })
    }
}
