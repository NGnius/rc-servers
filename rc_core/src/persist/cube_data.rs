use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use polariton::operation::Typed;

use super::{WeaponData, WeaponUpgradeInfo, TechTreeData, MovementData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cube {
    pub id: u32,
    pub info: CubeInfo,
    pub weapon: Option<WeaponData>,
    pub weapon_upgrade: Option<WeaponUpgradeInfo>,
    pub movement: Option<MovementData>,
    pub tree: Option<TechTreeData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CubeInfo {
    #[serde(default = "default_1")]
    pub cpu: u32,
    #[serde(default = "default_1")]
    pub health: u32,
    #[serde(default)]
    pub health_boost: f32,
    #[serde(default)]
    pub grey_out_in_tutorial: bool,
    #[serde(default)]
    pub visibility: VisibilityMode,
    #[serde(default)]
    pub indestructible: bool,
    #[serde(default)]
    pub category: ItemCategory,
    #[serde(default = "default_63")]
    pub placements: u32,
    #[serde(default)]
    pub protonium: bool,
    #[serde(default)]
    pub unlocked_by_league: bool,
    #[serde(default)]
    pub league_unlock_index: i32,
    pub stats: HashMap<String, serde_json::Value>,
    pub description: String,
    pub size: ItemTier,
    #[serde(rename = "type", alias = "type_")]
    pub type_: ItemType,
    #[serde(default)]
    pub ranking: i32,
    #[serde(default)]
    pub cosmetic: bool,
    #[serde(default)]
    pub variant_of: u32, // cube id (in hex)
    #[serde(default = "default_true")]
    pub ignore_in_weapon_list: bool,
}

fn default_1() -> u32 {
    1
}

fn default_63() -> u32 {
    63
}

fn default_true() -> bool {
    true
}

impl <C: Clone> std::convert::Into<crate::data::cube_list::CubeInfo<C>> for CubeInfo {
    fn into(self) -> crate::data::cube_list::CubeInfo<C> {
        crate::data::cube_list::CubeInfo {
            cpu: self.cpu,
            health: self.health,
            health_boost: self.health_boost,
            grey_out_in_tutorial: self.grey_out_in_tutorial,
            visibility: self.visibility.into(),
            indestructible: self.indestructible,
            category: self.category.into(),
            placements: self.placements, // default 63
            protonium: self.protonium,
            unlocked_by_league: self.unlocked_by_league,
            league_unlock_index: self.league_unlock_index,
            stats: self.stats.into_iter().map(|(k, v)| {
                let new_v = match v {
                    serde_json::Value::Bool(b) => Typed::Bool(b),
                    serde_json::Value::Number(n) => if let Some(n_i64) = n.as_i64() {
                        Typed::Long(n_i64)
                    } else if let Some(n_f64) = n.as_f64() {
                        Typed::Double(n_f64)
                    } else {
                        panic!("Invalid json number")
                    },
                    serde_json::Value::String(s) => Typed::Str(s.into()),
                    _ => panic!("Unsupported stats type"), // TODO is support for Object/Array/Null necessary?
                };
                (k, new_v)
            }).collect(),
            description: self.description,
            size: self.size.into(),
            type_: self.type_.into(),
            ranking: self.ranking,
            cosmetic: self.cosmetic,
            variant_of: hex::encode(self.variant_of.to_be_bytes()).into(),
            ignore_in_weapon_list: self.ignore_in_weapon_list,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub enum VisibilityMode {
    #[default]
    Mothership,
    All,
    Tutorial,
    None,
}

impl std::convert::Into<crate::data::cube_list::VisibilityMode> for VisibilityMode {
    fn into(self) -> crate::data::cube_list::VisibilityMode {
        match self {
            Self::Mothership => crate::data::cube_list::VisibilityMode::Mothership,
            Self::All => crate::data::cube_list::VisibilityMode::All,
            Self::Tutorial => crate::data::cube_list::VisibilityMode::Tutorial,
            Self::None => crate::data::cube_list::VisibilityMode::None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ItemTier {
    #[default]
    NoTier = 0,
    T0 = 100,
    T1 = 200,
    T2 = 300,
    T3 = 400,
    T4 = 500,
    T5 = 600,
}

impl std::convert::Into<crate::data::cube_list::ItemTier> for ItemTier {
    fn into(self) -> crate::data::cube_list::ItemTier {
        match self {
            Self::NoTier => crate::data::cube_list::ItemTier::NoTier,
            Self::T0 => crate::data::cube_list::ItemTier::T0,
            Self::T1 => crate::data::cube_list::ItemTier::T1,
            Self::T2 => crate::data::cube_list::ItemTier::T2,
            Self::T3 => crate::data::cube_list::ItemTier::T3,
            Self::T4 => crate::data::cube_list::ItemTier::T4,
            Self::T5 => crate::data::cube_list::ItemTier::T5,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub enum ItemType {
    #[default]
    NotAFunctionalItem,
    Weapon,
    Module,
    Movement,
    Cosmetic,
}

impl std::convert::Into<crate::data::cube_list::ItemType> for ItemType {
    fn into(self) -> crate::data::cube_list::ItemType {
        match self {
            Self::NotAFunctionalItem => crate::data::cube_list::ItemType::NoFunction,
            Self::Weapon => crate::data::cube_list::ItemType::Weapon,
            Self::Module => crate::data::cube_list::ItemType::Module,
            Self::Movement => crate::data::cube_list::ItemType::Movement,
            Self::Cosmetic => crate::data::cube_list::ItemType::Cosmetic,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ItemCategory {
    #[default]
    NotAFunctionalItem = 0,
    Wheel = 1,
    Hover = 2,
    Wing = 3,
    Rudder = 4,
    Thruster = 5,
    InsectLeg = 6,
    MechLeg = 7,
    Ski = 8,
    TankTrack = 9,
    Rotor = 10,
    SprinterLeg = 11,
    Propeller = 12,
    Laser = 100,
    Plasma = 200,
    Mortar = 250,
    Rail = 300,
    Nano = 400,
    Tesla = 500,
    Aeroflak = 600,
    Ion = 650,
    Seeker = 701,
    Chaingun = 750,
    ShieldModule = 800,
    GhostModule = 801,
    BlinkModule = 802,
    EmpModule = 803,
    WindowmakerModule = 804,
    EnergyModule = 900,
}

impl std::convert::Into<crate::data::weapon_list::ItemCategory> for ItemCategory {
    fn into(self) -> crate::data::weapon_list::ItemCategory {
        match self {
            Self::NotAFunctionalItem => crate::data::weapon_list::ItemCategory::NoFunction,
            Self::Wheel => crate::data::weapon_list::ItemCategory::Wheel,
            Self::Hover => crate::data::weapon_list::ItemCategory::Hover,
            Self::Wing => crate::data::weapon_list::ItemCategory::Wing,
            Self::Rudder => crate::data::weapon_list::ItemCategory::Rudder,
            Self::Thruster => crate::data::weapon_list::ItemCategory::Thruster,
            Self::InsectLeg => crate::data::weapon_list::ItemCategory::InsectLeg,
            Self::MechLeg => crate::data::weapon_list::ItemCategory::MechLeg,
            Self::Ski => crate::data::weapon_list::ItemCategory::Ski,
            Self::TankTrack => crate::data::weapon_list::ItemCategory::TankTrack,
            Self::Rotor => crate::data::weapon_list::ItemCategory::Rotor,
            Self::SprinterLeg => crate::data::weapon_list::ItemCategory::SprinterLeg,
            Self::Propeller => crate::data::weapon_list::ItemCategory::Propeller,
            Self::Laser => crate::data::weapon_list::ItemCategory::Laser,
            Self::Plasma => crate::data::weapon_list::ItemCategory::Plasma,
            Self::Mortar => crate::data::weapon_list::ItemCategory::Mortar,
            Self::Rail => crate::data::weapon_list::ItemCategory::Rail,
            Self::Nano => crate::data::weapon_list::ItemCategory::Nano,
            Self::Tesla => crate::data::weapon_list::ItemCategory::Tesla,
            Self::Aeroflak => crate::data::weapon_list::ItemCategory::Aeroflak,
            Self::Ion => crate::data::weapon_list::ItemCategory::Ion,
            Self::Seeker => crate::data::weapon_list::ItemCategory::Seeker,
            Self::Chaingun => crate::data::weapon_list::ItemCategory::Chaingun,
            Self::ShieldModule => crate::data::weapon_list::ItemCategory::ShieldModule,
            Self::GhostModule => crate::data::weapon_list::ItemCategory::GhostModule,
            Self::BlinkModule => crate::data::weapon_list::ItemCategory::BlinkModule,
            Self::EmpModule => crate::data::weapon_list::ItemCategory::EmpModule,
            Self::WindowmakerModule => crate::data::weapon_list::ItemCategory::WindowmakerModule,
            Self::EnergyModule => crate::data::weapon_list::ItemCategory::EnergyModule,
        }
    }
}
