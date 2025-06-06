use serde::{Serialize, Deserialize};

use super::ItemCategory;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GarageSlot {
    #[serde(default)]
    pub slot: i32,
    pub name: String,
    #[serde(default)]
    pub cubes: u32,
    #[serde(default)]
    pub crf_id: i32, // 0 means not uploaded
    #[serde(default = "default_false")]
    pub was_rated: bool,
    #[serde(default)]
    pub movement_categories: Vec<ItemCategory>,
    #[serde(default)]
    pub uuid: (u32, u32),
    pub thumbnail_version: i32,
    #[serde(default)]
    pub total_robot_cpu: i32,
    #[serde(default)]
    pub total_cosmetic_cpu: i32,
    #[serde(default)]
    pub total_robot_ranking: i32,
    #[serde(default)]
    pub bay_cpu: i32,
    #[serde(default = "default_false")]
    pub tutorial_robot: bool,
    #[serde(default = "default_neg_1")]
    pub starter_robot_index: i32,
    #[serde(default)]
    pub control_type: ControlType,
    #[serde(default)]
    pub control_options: GarageControls,
    #[serde(default)]
    pub mastery_level: i32,
    #[serde(default)]
    pub bay_skin_id: String,
    #[serde(default)]
    pub weapon_order: Vec<i32>,
    #[serde(default = "default_robot_bytes")]
    pub robot_data: Vec<u8>,
    #[serde(default = "default_robot_bytes")]
    pub colour_data: Vec<u8>,
}

fn default_false() -> bool {
    false
}

fn default_neg_1() -> i32 {
    -1
}

fn default_robot_bytes() -> Vec<u8> {
    vec![0u8, 0u8, 0u8, 0u8]
}

impl GarageSlot {
    pub fn load(filepath: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let file = std::fs::File::open(filepath)?;
        let buffered = std::io::BufReader::new(file);
        let result = serde_json::from_reader(buffered)?;
        Ok(result)
    }

    pub fn save(&self, filepath: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let file = std::fs::File::create(filepath)?;
        let buffered = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(buffered, self)?;
        Ok(())
    }
}

impl std::convert::Into<crate::data::garage_bay::GarageSlotInfo> for GarageSlot {
    fn into(self) -> crate::data::garage_bay::GarageSlotInfo {
        crate::data::garage_bay::GarageSlotInfo {
            name: self.name,
            cubes: self.cubes,
            crf_id: self.crf_id as u32,
            was_rated: self.was_rated,
            movement_categories: self.movement_categories.into_iter().map(|x| x.into()).collect(),
            uuid: self.uuid,
            thumbnail_version: self.thumbnail_version as u32,
            total_robot_cpu: self.total_robot_cpu as u32,
            total_cosmetic_cpu: self.total_cosmetic_cpu as u32,
            total_robot_ranking: self.total_robot_ranking as u32,
            bay_cpu: self.bay_cpu as u32,
            tutorial_robot: self.tutorial_robot,
            starter_robot_index: self.starter_robot_index,
            control_type: self.control_type.into(),
            control_options: self.control_options.into(),
            mastery_level: self.mastery_level,
            bay_skin_id: self.bay_skin_id,
            weapon_order: self.weapon_order,
        }
    }
}

pub fn db_into_data(garage: oj_rc_database::schema::garage::Model) -> crate::data::garage_bay::GarageSlotInfo {
    let cube_count = garage.cube_count();
    crate::data::garage_bay::GarageSlotInfo {
        name: garage.name,
        cubes: cube_count,
        crf_id: garage.crf_id.unwrap_or(0) as u32,
        was_rated: garage.was_rated,
        movement_categories: movement_category_into_data(&garage.movement_categories),
        uuid: super::user::i64_split(garage.uuid),
        thumbnail_version: garage.thumbnail_version as u32,
        total_robot_cpu: garage.total_robot_cpu as u32,
        total_cosmetic_cpu: garage.total_cosmetic_cpu as u32,
        total_robot_ranking: garage.total_robot_ranking as u32,
        bay_cpu: garage.bay_cpu as u32,
        tutorial_robot: garage.tutorial_robot,
        starter_robot_index: garage.starter_robot_index.map(|x| x as i32).unwrap_or(-1),
        control_type: control_ty_into_data(garage.control_type),
        control_options: crate::data::garage_bay::ControlOptions {
            vertical_strafing: garage.vertical_strafing,
            sideways_driving: garage.sideways_driving,
            tracks_turn_on_spot: garage.tracks_turn_on_spot,
        },
        mastery_level: garage.mastery_level as i32,
        bay_skin_id: garage.bay_skin_id,
        weapon_order: oj_rc_database::schema::parse_int_csv(&garage.weapon_order).into_iter().map(|x| x as i32).collect(),
    }
}

fn movement_category_into_data(mov_cat: &str) -> Vec<crate::data::weapon_list::ItemCategory> {
    oj_rc_database::schema::parse_int_csv(mov_cat)
        .into_iter()
        .filter_map(|num| crate::data::weapon_list::ItemCategory::from_bigger(num as _))
        .collect()
}

pub fn control_ty_into_data(control_ty: oj_rc_database::schema::garage::ControlType) -> crate::data::garage_bay::ControlType {
    match control_ty {
        oj_rc_database::schema::garage::ControlType::Camera => crate::data::garage_bay::ControlType::Camera,
        oj_rc_database::schema::garage::ControlType::Keyboard => crate::data::garage_bay::ControlType::Keyboard,
        oj_rc_database::schema::garage::ControlType::Count => crate::data::garage_bay::ControlType::Count,
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub enum ControlType {
    #[default]
    Camera,
    Keyboard,
    Count,
}

impl std::convert::Into<crate::data::garage_bay::ControlType> for ControlType {
    fn into(self) -> crate::data::garage_bay::ControlType {
        match self {
            Self::Camera => crate::data::garage_bay::ControlType::Camera,
            Self::Keyboard => crate::data::garage_bay::ControlType::Keyboard,
            Self::Count => crate::data::garage_bay::ControlType::Count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GarageControls {
    pub vertical_strafing: bool,
    pub sideways_driving: bool,
    pub tracks_turn_on_spot: bool,
}

impl std::convert::Into<crate::data::garage_bay::ControlOptions> for GarageControls {
    fn into(self) -> crate::data::garage_bay::ControlOptions {
        crate::data::garage_bay::ControlOptions {
            vertical_strafing: self.vertical_strafing,
            sideways_driving: self.sideways_driving,
            tracks_turn_on_spot: self.tracks_turn_on_spot,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrefabVehicle {
    pub name: Option<String>,
    pub username: String,
    #[serde(flatten)]
    pub id: PrefabId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum PrefabId {
    Factory {
        #[serde(alias="crf")]
        factory: u32,
    },
    Database {
        garage: i32,
    },
    Raw {
        cube_data: Vec<u8>,
        colour_data: Vec<u8>,
    }
    // TODO File
}

impl std::convert::Into<crate::persist::config::VehicleDescriptor> for PrefabId {
    fn into(self) -> crate::persist::config::VehicleDescriptor {
        match self {
            Self::Factory { factory } => crate::persist::config::VehicleDescriptor::Factory { factory },
            Self::Database { garage } => crate::persist::config::VehicleDescriptor::Database { garage },
            Self::Raw { cube_data, colour_data } => crate::persist::config::VehicleDescriptor::Raw { cube_data , colour_data },
        }
    }
}
