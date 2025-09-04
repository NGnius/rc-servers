#![allow(dead_code)]
use std::collections::HashMap;
use polariton::operation::{Dict, Typed};

pub struct CubeInfo<C: Clone> {
    pub cpu: u32,
    pub health: u32,
    pub health_boost: f32,
    pub grey_out_in_tutorial: bool,
    pub visibility: VisibilityMode,
    pub indestructible: bool,
    pub category: super::weapon_list::ItemCategory,
    pub placements: u32, // default 63
    pub protonium: bool,
    pub unlocked_by_league: bool,
    pub league_unlock_index: i32,
    pub stats: HashMap<String, Typed<C>>,
    pub description: String,
    pub size: ItemTier,
    pub type_: ItemType,
    pub ranking: i32,
    pub cosmetic: bool,
    pub variant_of: String, // cube id (in hex)
    pub ignore_in_weapon_list: bool,
}

impl <C: Clone> CubeInfo<C> {
    pub fn as_transmissible(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("cpuRating".into()), Typed::Int(self.cpu as i32)),
            (Typed::Str("health".into()), Typed::Int(self.health as i32)),
            (Typed::Str("healthBoost".into()), Typed::Float(self.health_boost)),
            (Typed::Str("GreyOutInTutorial".into()), Typed::Bool(self.grey_out_in_tutorial)),
            (Typed::Str("buildVisibility".into()), Typed::Str(self.visibility.as_str().into())),
            (Typed::Str("isIndestructible".into()), Typed::Bool(self.indestructible)),
            (Typed::Str("ItemCategory".into()), Typed::Int(self.category as i32)),
            (Typed::Str("PlacementFaces".into()), Typed::Int(self.placements as i32)),
            (Typed::Str("protoniumCrystal".into()), Typed::Bool(self.protonium)),
            (Typed::Str("UnlockedByLeague".into()), Typed::Bool(self.unlocked_by_league)),
            (Typed::Str("LeagueUnlockIndex".into()), Typed::Int(self.league_unlock_index)),
            (Typed::Str("DisplayStats".into()), {
                let items: Vec<(Typed<C>, Typed<C>)> = self.stats.iter().map(|(key, val)| (Typed::<C>::Str(key.into()), val.to_owned())).collect();
                //Typed::HashMap(items.into())
                Typed::Dict(Dict {
                    key_ty: polariton::serdes::TypePrefix::Str,
                    val_ty: polariton::serdes::TypePrefix::Any,
                    items,
                })
            }),
            (Typed::Str("Description".into()), Typed::Str(self.description.clone().into())),
            (Typed::Str("ItemSize".into()), Typed::Int(self.size as i32)),
            (Typed::Str("ItemType".into()), Typed::Str(self.type_.as_str().into())),
            (Typed::Str("robotRanking".into()), Typed::Int(self.ranking)),
            (Typed::Str("isCosmetic".into()), Typed::Bool(self.cosmetic)),
            (Typed::Str("variantOf".into()), Typed::Str(self.variant_of.clone().into())),
            (Typed::Str("ignoreInWeaponsList".into()), Typed::Bool(self.ignore_in_weapon_list)), // optional
        ].into())
    }

    pub fn as_transmissible_key_val(&self, cube_id: u32) -> (Typed<C>, Typed<C>) {
        (Typed::Str(hex::encode(cube_id.to_be_bytes()).into()), self.as_transmissible())
    }
}

#[derive(Clone, Copy)]
pub enum VisibilityMode {
    Mothership,
    All,
    Tutorial,
    None,
}

impl VisibilityMode {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Mothership => "Mothership",
            Self::All => "All",
            Self::Tutorial => "Tutorial",
            Self::None => "None",
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ItemTier {
    NoTier = 0,
    T0 = 100,
    T1 = 200,
    T2 = 300,
    T3 = 400,
    T4 = 500,
    T5 = 600,
}

impl ItemTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemTier::NoTier => "NotAWeapon",
            ItemTier::T0 => "T0",
            ItemTier::T1 => "T1",
            ItemTier::T2 => "T2",
            ItemTier::T3 => "T3",
            ItemTier::T4 => "T4",
            ItemTier::T5 => "T5",
        }
    }

    pub fn from_u32(num: u32) -> Option<Self> {
        match num {
            0 => Some(Self::NoTier),
            100 => Some(Self::T0),
            200 => Some(Self::T1),
            300 => Some(Self::T2),
            400 => Some(Self::T3),
            500 => Some(Self::T4),
            600 => Some(Self::T5),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ItemType {
    NoFunction,
    Weapon,
    Module,
    Movement,
    Cosmetic,
}

impl ItemType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::NoFunction => "NotAFunctionalItem",
            Self::Weapon => "Weapon",
            Self::Module => "Module",
            Self::Movement => "Movement",
            Self::Cosmetic => "Cosmetic",
        }
    }
}

pub fn item_key(category: super::weapon_list::ItemCategory, tier: ItemTier) -> i32 {
    category.but_bigger() + (tier as i32)
}
