use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use polariton::operation::Typed;

use super::{WeaponData, WeaponUpgradeInfo, TechTreeData, MovementData, CubeConversionData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cube {
    pub id: u32,
    pub info: CubeInfo,
    pub weapon: Option<WeaponData>,
    pub weapon_upgrade: Option<WeaponUpgradeInfo>,
    pub movement: Option<MovementData>,
    pub tree: Option<TechTreeData>,
    pub conversion: Option<CubeConversionData>,
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

impl <C: Clone> std::convert::From<CubeInfo> for crate::data::cube_list::CubeInfo<C> {
    fn from(val: CubeInfo) -> Self {
        crate::data::cube_list::CubeInfo {
            cpu: val.cpu,
            health: val.health,
            health_boost: val.health_boost,
            grey_out_in_tutorial: val.grey_out_in_tutorial,
            visibility: val.visibility.into(),
            indestructible: val.indestructible,
            category: val.category.into(),
            placements: val.placements, // default 63
            protonium: val.protonium,
            unlocked_by_league: val.unlocked_by_league,
            league_unlock_index: val.league_unlock_index,
            stats: val.stats.into_iter().map(|(k, v)| {
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
            description: val.description,
            size: val.size.into(),
            type_: val.type_.into(),
            ranking: val.ranking,
            cosmetic: val.cosmetic,
            variant_of: hex::encode(val.variant_of.to_be_bytes()),
            ignore_in_weapon_list: val.ignore_in_weapon_list,
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

impl std::convert::From<VisibilityMode> for crate::data::cube_list::VisibilityMode {
    fn from(val: VisibilityMode) -> Self {
        match val {
            VisibilityMode::Mothership => crate::data::cube_list::VisibilityMode::Mothership,
            VisibilityMode::All => crate::data::cube_list::VisibilityMode::All,
            VisibilityMode::Tutorial => crate::data::cube_list::VisibilityMode::Tutorial,
            VisibilityMode::None => crate::data::cube_list::VisibilityMode::None,
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

impl std::convert::From<ItemTier> for crate::data::cube_list::ItemTier {
    fn from(val: ItemTier) -> Self {
        match val {
            ItemTier::NoTier => crate::data::cube_list::ItemTier::NoTier,
            ItemTier::T0 => crate::data::cube_list::ItemTier::T0,
            ItemTier::T1 => crate::data::cube_list::ItemTier::T1,
            ItemTier::T2 => crate::data::cube_list::ItemTier::T2,
            ItemTier::T3 => crate::data::cube_list::ItemTier::T3,
            ItemTier::T4 => crate::data::cube_list::ItemTier::T4,
            ItemTier::T5 => crate::data::cube_list::ItemTier::T5,
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

impl std::convert::From<ItemType> for crate::data::cube_list::ItemType {
    fn from(val: ItemType) -> Self {
        match val {
            ItemType::NotAFunctionalItem => crate::data::cube_list::ItemType::NoFunction,
            ItemType::Weapon => crate::data::cube_list::ItemType::Weapon,
            ItemType::Module => crate::data::cube_list::ItemType::Module,
            ItemType::Movement => crate::data::cube_list::ItemType::Movement,
            ItemType::Cosmetic => crate::data::cube_list::ItemType::Cosmetic,
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

impl std::convert::From<ItemCategory> for crate::data::weapon_list::ItemCategory {
    fn from(val: ItemCategory) -> Self {
        match val {
            ItemCategory::NotAFunctionalItem => crate::data::weapon_list::ItemCategory::NoFunction,
            ItemCategory::Wheel => crate::data::weapon_list::ItemCategory::Wheel,
            ItemCategory::Hover => crate::data::weapon_list::ItemCategory::Hover,
            ItemCategory::Wing => crate::data::weapon_list::ItemCategory::Wing,
            ItemCategory::Rudder => crate::data::weapon_list::ItemCategory::Rudder,
            ItemCategory::Thruster => crate::data::weapon_list::ItemCategory::Thruster,
            ItemCategory::InsectLeg => crate::data::weapon_list::ItemCategory::InsectLeg,
            ItemCategory::MechLeg => crate::data::weapon_list::ItemCategory::MechLeg,
            ItemCategory::Ski => crate::data::weapon_list::ItemCategory::Ski,
            ItemCategory::TankTrack => crate::data::weapon_list::ItemCategory::TankTrack,
            ItemCategory::Rotor => crate::data::weapon_list::ItemCategory::Rotor,
            ItemCategory::SprinterLeg => crate::data::weapon_list::ItemCategory::SprinterLeg,
            ItemCategory::Propeller => crate::data::weapon_list::ItemCategory::Propeller,
            ItemCategory::Laser => crate::data::weapon_list::ItemCategory::Laser,
            ItemCategory::Plasma => crate::data::weapon_list::ItemCategory::Plasma,
            ItemCategory::Mortar => crate::data::weapon_list::ItemCategory::Mortar,
            ItemCategory::Rail => crate::data::weapon_list::ItemCategory::Rail,
            ItemCategory::Nano => crate::data::weapon_list::ItemCategory::Nano,
            ItemCategory::Tesla => crate::data::weapon_list::ItemCategory::Tesla,
            ItemCategory::Aeroflak => crate::data::weapon_list::ItemCategory::Aeroflak,
            ItemCategory::Ion => crate::data::weapon_list::ItemCategory::Ion,
            ItemCategory::Seeker => crate::data::weapon_list::ItemCategory::Seeker,
            ItemCategory::Chaingun => crate::data::weapon_list::ItemCategory::Chaingun,
            ItemCategory::ShieldModule => crate::data::weapon_list::ItemCategory::ShieldModule,
            ItemCategory::GhostModule => crate::data::weapon_list::ItemCategory::GhostModule,
            ItemCategory::BlinkModule => crate::data::weapon_list::ItemCategory::BlinkModule,
            ItemCategory::EmpModule => crate::data::weapon_list::ItemCategory::EmpModule,
            ItemCategory::WindowmakerModule => crate::data::weapon_list::ItemCategory::WindowmakerModule,
            ItemCategory::EnergyModule => crate::data::weapon_list::ItemCategory::EnergyModule,
        }
    }
}
