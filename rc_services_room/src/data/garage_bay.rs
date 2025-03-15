use polariton::{operation::{Arr, Typed}, serdes::TypePrefix};

use super::weapon_list::ItemCategory;

pub struct GarageSlotInfo {
    pub name: String,
    pub cubes: u32,
    pub crf_id: u32, // 0 means not uploaded
    pub was_rated: bool, // ignored when not on CRF
    pub movement_categories: Vec<ItemCategory>,
    pub uuid: (u32, u32),
    pub thumbnail_version: u32,
    pub total_robot_cpu: u32,
    pub total_cosmetic_cpu: u32,
    pub total_robot_ranking: u32,
    pub bay_cpu: u32,
    pub tutorial_robot: bool, // assumed to be false (when omitted)
    pub starter_robot_index: i32, // assumed to be -1 (whem omitted)
    pub control_type: ControlType,
    pub control_options: ControlOptions,
    pub mastery_level: i32,
    pub bay_skin_id: String,
    pub weapon_order: Vec<i32>,
}

impl GarageSlotInfo {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("name".into()), Typed::Str(self.name.clone().into())),
            (Typed::Str("numberCubes".into()), Typed::Int(self.cubes as i32)),
            (Typed::Str("crfId".into()), Typed::Int(self.crf_id as i32)),
            (Typed::Str("wasRated".into()), Typed::Bool(self.was_rated.into())),
            (Typed::Str("movementCategories".into()), Typed::Arr(Arr {
                ty: TypePrefix::Int, // int
                items: self.movement_categories.iter().map(|x| Typed::Int(x.but_bigger() as i32)).collect(),
            })),
            (Typed::Str("uniqueId1".into()), Typed::Int(self.uuid.0 as i32)),
            (Typed::Str("uniqueId2".into()), Typed::Int(self.uuid.1 as i32)),
            (Typed::Str("thumbnailVersion".into()), Typed::Int(self.thumbnail_version as i32)),
            (Typed::Str("totalRobotCPU".into()), Typed::Int(self.total_robot_cpu as i32)),
            (Typed::Str("totalCosmeticCPU".into()), Typed::Int(self.total_cosmetic_cpu as i32)),
            (Typed::Str("totalRobotRanking".into()), Typed::Int(self.total_robot_ranking as i32)),
            (Typed::Str("bayCpu".into()), Typed::Int(self.bay_cpu as i32)),
            (Typed::Str("tutorialRobot".into()), Typed::Bool(self.tutorial_robot.into())),
            (Typed::Str("starterRobotIndex".into()), Typed::Int(self.starter_robot_index)),
            (Typed::Str("controlType".into()), Typed::Int(self.control_type as i32)),
            (Typed::Str("controlOptions".into()), self.control_options.as_transmissible()),
            (Typed::Str("masteryLevel".into()), Typed::Int(self.mastery_level)),
            (Typed::Str("baySkinId".into()), Typed::Str(self.bay_skin_id.clone().into())),
            (Typed::Str("weaponOrder".into()), Typed::Arr(Arr {
                ty: TypePrefix::Int, // int
                items: self.weapon_order.iter().map(|x| Typed::Int(*x)).collect(),
            })),
        ].into())
    }
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum ControlType {
    Camera = 0,
    Keyboard = 1,
    Count = 2,
}

pub struct ControlOptions {
    pub vertical_strafing: bool,
    pub sideways_driving: bool,
    pub tracks_turn_on_spot: bool,
}

impl ControlOptions {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::Arr(Arr {
            ty: TypePrefix::Bool, // bool
            items: vec![
                Typed::Bool(self.vertical_strafing.into()),
                Typed::Bool(self.sideways_driving.into()),
                Typed::Bool(self.tracks_turn_on_spot.into()),
            ],
        })
    }
}


