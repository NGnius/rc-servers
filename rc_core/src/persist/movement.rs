use serde::{Serialize, Deserialize};

use super::ItemTier;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MovementCategoryData {
    pub horizontal_top_speed: Option<f32>,
    pub vertical_top_speed: Option<f32>,
    pub min_required_items: Option<i32>,
    pub min_item_modifier: Option<f32>,
    pub max_hover_height: Option<f32>,
    pub light_machine_mass: Option<f32>,
    pub heavy_machine_mass: Option<f32>,
    #[serde(flatten)]
    pub specifics: MovementCategorySpecificData,
}

impl MovementCategoryData {
    pub fn into_data(self, stats: Vec<(ItemTier, MovementData)>) -> crate::data::movement_list::MovementCategoryData {
        crate::data::movement_list::MovementCategoryData {
            horizontal_top_speed: self.horizontal_top_speed,
            vertical_top_speed: self.vertical_top_speed,
            min_required_items: self.min_required_items,
            min_item_modifier: self.min_item_modifier,
            max_hover_height: self.max_hover_height,
            light_machine_mass: self.light_machine_mass,
            heavy_machine_mass: self.heavy_machine_mass,
            specifics: self.specifics.into(),
            stats: stats.into_iter().map(|(t, m)| (t.into(), m.into())).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(tag = "movement_enum_variant")]
pub enum MovementCategorySpecificData {
    #[default]
    Wheel,
    Hover(HoverCategoryData),
    Wing,
    Rudder, // same as wing
    Thruster,
    Propeller, // same as thruster
    InsectLeg,
    MechLeg(MechLegCategoryData),
    SprinterLeg(MechLegCategoryData), // same as mech leg
    TankTrack,
    Rotor(RotorCategoryData),
    Ski,
}

impl std::convert::From<MovementCategorySpecificData> for crate::data::movement_list::MovementCategorySpecificData {
    fn from(val: MovementCategorySpecificData) -> Self {
        match val {
            MovementCategorySpecificData::Wheel => crate::data::movement_list::MovementCategorySpecificData::Wheel,
            MovementCategorySpecificData::Hover(x) => crate::data::movement_list::MovementCategorySpecificData::Hover(x.into()),
            MovementCategorySpecificData::Wing => crate::data::movement_list::MovementCategorySpecificData::Wing,
            MovementCategorySpecificData::Rudder => crate::data::movement_list::MovementCategorySpecificData::Rudder,
            MovementCategorySpecificData::Thruster => crate::data::movement_list::MovementCategorySpecificData::Thruster,
            MovementCategorySpecificData::Propeller => crate::data::movement_list::MovementCategorySpecificData::Propeller,
            MovementCategorySpecificData::InsectLeg => crate::data::movement_list::MovementCategorySpecificData::InsectLeg,
            MovementCategorySpecificData::MechLeg(x) => crate::data::movement_list::MovementCategorySpecificData::MechLeg(x.into()),
            MovementCategorySpecificData::SprinterLeg(x) => crate::data::movement_list::MovementCategorySpecificData::SprinterLeg(x.into()),
            MovementCategorySpecificData::TankTrack => crate::data::movement_list::MovementCategorySpecificData::TankTrack,
            MovementCategorySpecificData::Rotor(x) => crate::data::movement_list::MovementCategorySpecificData::Rotor(x.into()),
            MovementCategorySpecificData::Ski => crate::data::movement_list::MovementCategorySpecificData::Ski,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HoverCategoryData {
    pub height_tolerance: f32,
    pub force_y_offset: f32,
    pub turning_scale: f32,
    pub small_angle_turning_scale: f32,
    pub hover_damping: f32,
    pub angular_damping: f32,
    pub deceleration_multiplier: f32,
}

impl std::convert::From<HoverCategoryData> for crate::data::movement_list::HoverCategoryData {
    fn from(val: HoverCategoryData) -> Self {
        crate::data::movement_list::HoverCategoryData {
            height_tolerance: val.height_tolerance,
            force_y_offset: val.force_y_offset,
            turning_scale: val.turning_scale,
            small_angle_turning_scale: val.small_angle_turning_scale,
            hover_damping: val.hover_damping,
            angular_damping: val.angular_damping,
            deceleration_multiplier: val.deceleration_multiplier,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MechLegCategoryData {
    pub deceleration_multiplier: f32,
}

impl std::convert::From<MechLegCategoryData> for crate::data::movement_list::MechLegCategoryData {
    fn from(val: MechLegCategoryData) -> Self {
        crate::data::movement_list::MechLegCategoryData {
            deceleration_multiplier: val.deceleration_multiplier,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RotorCategoryData {
    pub max_turn_rate: f32,
}

impl std::convert::From<RotorCategoryData> for crate::data::movement_list::RotorCategoryData {
    fn from(val: RotorCategoryData) -> Self {
        crate::data::movement_list::RotorCategoryData {
            max_turn_rate: val.max_turn_rate,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovementData {
    pub speed_boost: Option<f32>,
    pub max_carry_mass: Option<f32>,
    pub horizontal_top_speed: Option<f32>,
    pub vertical_top_speed: Option<f32>,
    #[serde(flatten)]
    pub specifics: MovementSpecificData,
}

impl std::convert::From<MovementData> for crate::data::movement_list::MovementData {
    fn from(val: MovementData) -> Self {
        crate::data::movement_list::MovementData {
            speed_boost: val.speed_boost,
            max_carry_mass: val.max_carry_mass,
            horizontal_top_speed: val.horizontal_top_speed,
            vertical_top_speed: val.vertical_top_speed,
            specifics: val.specifics.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "movement_enum_variant")]
pub enum MovementSpecificData {
    Wheel(WheelData),
    Hover(HoverData),
    Wing(AerofoilData),
    Rudder(AerofoilData), // same as wing
    Thruster(ThrusterData),
    Propeller(ThrusterData), // same as thruster
    InsectLeg(InsectLegData),
    MechLeg(MechLegData),
    SprinterLeg(MechLegData), // same as mech leg
    TankTrack(TankTrackData),
    Rotor(RotorData),
    Ski,
}

impl std::convert::From<MovementSpecificData> for crate::data::movement_list::MovementSpecificData {
    fn from(val: MovementSpecificData) -> Self {
        match val {
            MovementSpecificData::Wheel(x) => crate::data::movement_list::MovementSpecificData::Wheel(x.into()),
            MovementSpecificData::Hover(x) => crate::data::movement_list::MovementSpecificData::Hover(x.into()),
            MovementSpecificData::Wing(x) => crate::data::movement_list::MovementSpecificData::Wing(x.into()),
            MovementSpecificData::Rudder(x) => crate::data::movement_list::MovementSpecificData::Rudder(x.into()),
            MovementSpecificData::Thruster(x) => crate::data::movement_list::MovementSpecificData::Thruster(x.into()),
            MovementSpecificData::Propeller(x) => crate::data::movement_list::MovementSpecificData::Propeller(x.into()),
            MovementSpecificData::InsectLeg(x) => crate::data::movement_list::MovementSpecificData::InsectLeg(x.into()),
            MovementSpecificData::MechLeg(x) => crate::data::movement_list::MovementSpecificData::MechLeg(x.into()),
            MovementSpecificData::SprinterLeg(x) => crate::data::movement_list::MovementSpecificData::SprinterLeg(x.into()),
            MovementSpecificData::TankTrack(x) => crate::data::movement_list::MovementSpecificData::TankTrack(x.into()),
            MovementSpecificData::Rotor(x) => crate::data::movement_list::MovementSpecificData::Rotor(x.into()),
            MovementSpecificData::Ski=> crate::data::movement_list::MovementSpecificData::Ski,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct WheelData {
    pub steering_speed_light: f32,
    pub steering_speed_heavy: f32,
    pub steering_force_multiplier_light: f32,
    pub steering_force_multiplier_heavy: f32,
    pub lateral_acceleration_light: f32,
    pub lateral_acceleration_heavy: f32,
    pub time_to_max_acceleration_light: f32,
    pub time_to_max_acceleration_heavy: f32,
    pub brake_force_light: f32,
    pub brake_force_heavy: f32,
}

impl std::convert::From<WheelData> for crate::data::movement_list::WheelData {
    fn from(val: WheelData) -> Self {
        crate::data::movement_list::WheelData {
            steering_speed_light: val.steering_speed_light,
            steering_speed_heavy: val.steering_speed_heavy,
            steering_force_multiplier_light: val.steering_force_multiplier_light,
            steering_force_multiplier_heavy: val.steering_force_multiplier_heavy,
            lateral_acceleration_light: val.lateral_acceleration_light,
            lateral_acceleration_heavy: val.lateral_acceleration_heavy,
            time_to_max_acceleration_light: val.time_to_max_acceleration_light,
            time_to_max_acceleration_heavy: val.time_to_max_acceleration_heavy,
            brake_force_light: val.brake_force_light,
            brake_force_heavy: val.brake_force_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HoverData {
    pub max_hover_height_light: f32,
    pub max_hover_height_heavy: f32,
    pub height_change_speed_light: f32,
    pub height_change_speed_heavy: f32,
    pub turn_torque_light: f32,
    pub turn_torque_heavy: f32,
    pub acceleration_light: f32,
    pub acceleration_heavy: f32,
    pub max_angular_velocity_light: f32,
    pub max_angular_velocity_heavy: f32,
    pub lateral_damping_light: f32,
    pub lateral_damping_heavy: f32,
}

impl std::convert::From<HoverData> for crate::data::movement_list::HoverData {
    fn from(val: HoverData) -> Self {
        crate::data::movement_list::HoverData {
            max_hover_height_light: val.max_hover_height_light,
            max_hover_height_heavy: val.max_hover_height_heavy,
            height_change_speed_light: val.height_change_speed_light,
            height_change_speed_heavy: val.height_change_speed_heavy,
            turn_torque_light: val.turn_torque_light,
            turn_torque_heavy: val.turn_torque_heavy,
            acceleration_light: val.acceleration_light,
            acceleration_heavy: val.acceleration_heavy,
            max_angular_velocity_light: val.max_angular_velocity_light,
            max_angular_velocity_heavy: val.max_angular_velocity_heavy,
            lateral_damping_light: val.lateral_damping_light,
            lateral_damping_heavy: val.lateral_damping_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AerofoilData {
    pub barrel_speed_light: f32,
    pub barrel_speed_heavy: f32,
    pub bank_speed_light: f32,
    pub bank_speed_heavy: f32,
    pub elevation_speed_light: f32,
    pub elevation_speed_heavy: f32,
    pub rudder_speed_light: f32,
    pub rudder_speed_heavy: f32,
    pub thrust_light: f32,
    pub thrust_heavy: f32,
    pub vtol_velocity_light: f32,
    pub vtol_velocity_heavy: f32,
}

impl std::convert::From<AerofoilData> for crate::data::movement_list::AerofoilData {
    fn from(val: AerofoilData) -> Self {
        crate::data::movement_list::AerofoilData {
            barrel_speed_light: val.barrel_speed_light,
            barrel_speed_heavy: val.barrel_speed_heavy,
            bank_speed_light: val.bank_speed_light,
            bank_speed_heavy: val.bank_speed_heavy,
            elevation_speed_light: val.elevation_speed_light,
            elevation_speed_heavy: val.elevation_speed_heavy,
            rudder_speed_light: val.rudder_speed_light,
            rudder_speed_heavy: val.rudder_speed_heavy,
            thrust_light: val.thrust_light,
            thrust_heavy: val.thrust_heavy,
            vtol_velocity_light: val.vtol_velocity_light,
            vtol_velocity_heavy: val.vtol_velocity_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ThrusterData {
    pub acceleration_delay_light: f32,
    pub acceleration_delay_heavy: f32,
}

impl std::convert::From<ThrusterData> for crate::data::movement_list::ThrusterData {
    fn from(val: ThrusterData) -> Self {
        crate::data::movement_list::ThrusterData {
            acceleration_delay_light: val.acceleration_delay_light,
            acceleration_delay_heavy: val.acceleration_delay_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct InsectLegData {
    pub ideal_height_light: f32,
    pub ideal_height_heavy: f32,
    pub ideal_crouching_height_light: f32,
    pub ideal_crouching_height_heavy: f32,
    pub ideal_height_range_light: f32,
    pub ideal_height_range_heavy: f32,
    pub jump_height_light: f32,
    pub jump_height_heavy: f32,
    pub max_upwards_force_light: f32,
    pub max_upwards_force_heavy: f32,
    pub max_lateral_force_light: f32,
    pub max_lateral_force_heavy: f32,
    pub max_turning_force_light: f32,
    pub max_turning_force_heavy: f32,
    pub max_damping_force_light: f32,
    pub max_damping_force_heavy: f32,
    pub max_stopped_force_light: f32,
    pub max_stopped_force_heavy: f32,
    pub max_new_stopped_force_light: f32,
    pub max_new_stopped_force_heavy: f32,
    pub upwards_damping_force_light: f32,
    pub upwards_damping_force_heavy: f32,
    pub lateral_damp_force_light: f32,
    pub lateral_damp_force_heavy: f32,
    pub swagger_force_light: f32,
    pub swagger_force_heavy: f32,
}

impl std::convert::From<InsectLegData> for crate::data::movement_list::InsectLegData {
    fn from(val: InsectLegData) -> Self {
        crate::data::movement_list::InsectLegData {
            ideal_height_light: val.ideal_height_light,
            ideal_height_heavy: val.ideal_height_heavy,
            ideal_crouching_height_light: val.ideal_crouching_height_light,
            ideal_crouching_height_heavy: val.ideal_crouching_height_heavy,
            ideal_height_range_light: val.ideal_height_range_light,
            ideal_height_range_heavy: val.ideal_height_range_heavy,
            jump_height_light: val.jump_height_light,
            jump_height_heavy: val.jump_height_heavy,
            max_upwards_force_light: val.max_upwards_force_light,
            max_upwards_force_heavy: val.max_upwards_force_heavy,
            max_lateral_force_light: val.max_lateral_force_light,
            max_lateral_force_heavy: val.max_lateral_force_heavy,
            max_turning_force_light: val.max_turning_force_light,
            max_turning_force_heavy: val.max_turning_force_heavy,
            max_damping_force_light: val.max_damping_force_light,
            max_damping_force_heavy: val.max_damping_force_heavy,
            max_stopped_force_light: val.max_stopped_force_light,
            max_stopped_force_heavy: val.max_stopped_force_heavy,
            max_new_stopped_force_light: val.max_new_stopped_force_light,
            max_new_stopped_force_heavy: val.max_new_stopped_force_heavy,
            upwards_damping_force_light: val.upwards_damping_force_light,
            upwards_damping_force_heavy: val.upwards_damping_force_heavy,
            lateral_damp_force_light: val.lateral_damp_force_light,
            lateral_damp_force_heavy: val.lateral_damp_force_heavy,
            swagger_force_light: val.swagger_force_light,
            swagger_force_heavy: val.swagger_force_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MechLegData {
    pub time_grounded_after_jump_light: f32,
    pub time_grounded_after_jump_heavy: f32,
    pub jump_height_light: f32,
    pub jump_height_heavy: f32,
    pub turn_acceleration_light: f32,
    pub turn_acceleration_heavy: f32,
    pub legacy_turn_acceleration_light: f32,
    pub legacy_turn_acceleration_heavy: f32,
    pub long_jump_speed_scale_light: f32,
    pub long_jump_speed_scale_heavy: f32,
    pub max_lateral_force_light: f32,
    pub max_lateral_force_heavy: f32,
    pub max_damping_force_light: f32,
    pub max_damping_force_heavy: f32,
}

impl std::convert::From<MechLegData> for crate::data::movement_list::MechLegData {
    fn from(val: MechLegData) -> Self {
        crate::data::movement_list::MechLegData {
            time_grounded_after_jump_light: val.time_grounded_after_jump_light,
            time_grounded_after_jump_heavy: val.time_grounded_after_jump_heavy,
            jump_height_light: val.jump_height_light,
            jump_height_heavy: val.jump_height_heavy,
            turn_acceleration_light: val.turn_acceleration_light,
            turn_acceleration_heavy: val.turn_acceleration_heavy,
            legacy_turn_acceleration_light: val.legacy_turn_acceleration_light,
            legacy_turn_acceleration_heavy: val.legacy_turn_acceleration_heavy,
            long_jump_speed_scale_light: val.long_jump_speed_scale_light,
            long_jump_speed_scale_heavy: val.long_jump_speed_scale_heavy,
            max_lateral_force_light: val.max_lateral_force_light,
            max_lateral_force_heavy: val.max_lateral_force_heavy,
            max_damping_force_light: val.max_damping_force_light,
            max_damping_force_heavy: val.max_damping_force_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TankTrackData {
    pub max_turn_rate_moving_light: f32,
    pub max_turn_rate_moving_heavy: f32,
    pub max_turn_rate_stopped_light: f32,
    pub max_turn_rate_stopped_heavy: f32,
    pub turn_acceleration_light: f32,
    pub turn_acceleration_heavy: f32,
    pub lateral_acceleration_light: f32,
    pub lateral_acceleration_heavy: f32,
}

impl std::convert::From<TankTrackData> for crate::data::movement_list::TankTrackData {
    fn from(val: TankTrackData) -> Self {
        crate::data::movement_list::TankTrackData {
            max_turn_rate_moving_light: val.max_turn_rate_moving_light,
            max_turn_rate_moving_heavy: val.max_turn_rate_moving_heavy,
            max_turn_rate_stopped_light: val.max_turn_rate_stopped_light,
            max_turn_rate_stopped_heavy: val.max_turn_rate_stopped_heavy,
            turn_acceleration_light: val.turn_acceleration_light,
            turn_acceleration_heavy: val.turn_acceleration_heavy,
            lateral_acceleration_light: val.lateral_acceleration_light,
            lateral_acceleration_heavy: val.lateral_acceleration_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RotorData {
    pub height_acceleration_light: f32,
    pub height_acceleration_heavy: f32,
    pub strafe_acceleration_light: f32,
    pub strafe_acceleration_heavy: f32,
    pub turn_acceleration_light: f32,
    pub turn_acceleration_heavy: f32,
    pub height_max_change_speed_light: f32,
    pub height_max_change_speed_heavy: f32,
    pub level_acceleration_light: f32,
    pub level_acceleration_heavy: f32,
}

impl std::convert::From<RotorData> for crate::data::movement_list::RotorData {
    fn from(val: RotorData) -> Self {
        crate::data::movement_list::RotorData {
            height_acceleration_light: val.height_acceleration_light,
            height_acceleration_heavy: val.height_acceleration_heavy,
            strafe_acceleration_light: val.strafe_acceleration_light,
            strafe_acceleration_heavy: val.strafe_acceleration_heavy,
            turn_acceleration_light: val.turn_acceleration_light,
            turn_acceleration_heavy: val.turn_acceleration_heavy,
            height_max_change_speed_light: val.height_max_change_speed_light,
            height_max_change_speed_heavy: val.height_max_change_speed_heavy,
            level_acceleration_light: val.level_acceleration_light,
            level_acceleration_heavy: val.level_acceleration_heavy,
        }
    }
}
