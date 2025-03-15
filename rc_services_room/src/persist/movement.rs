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
}

impl std::convert::Into<crate::data::movement_list::MovementCategorySpecificData> for MovementCategorySpecificData {
    fn into(self) -> crate::data::movement_list::MovementCategorySpecificData {
        match self {
            Self::Wheel => crate::data::movement_list::MovementCategorySpecificData::Wheel,
            Self::Hover(x) => crate::data::movement_list::MovementCategorySpecificData::Hover(x.into()),
            Self::Wing => crate::data::movement_list::MovementCategorySpecificData::Wing,
            Self::Rudder => crate::data::movement_list::MovementCategorySpecificData::Rudder,
            Self::Thruster => crate::data::movement_list::MovementCategorySpecificData::Thruster,
            Self::Propeller => crate::data::movement_list::MovementCategorySpecificData::Propeller,
            Self::InsectLeg => crate::data::movement_list::MovementCategorySpecificData::InsectLeg,
            Self::MechLeg(x) => crate::data::movement_list::MovementCategorySpecificData::MechLeg(x.into()),
            Self::SprinterLeg(x) => crate::data::movement_list::MovementCategorySpecificData::SprinterLeg(x.into()),
            Self::TankTrack => crate::data::movement_list::MovementCategorySpecificData::TankTrack,
            Self::Rotor(x) => crate::data::movement_list::MovementCategorySpecificData::Rotor(x.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HoverCategoryData {
    pub height_tolerance: f32,
    pub force_y_offset: f32,
    pub turning_scale: f32,
    pub small_angle_turning_scale: f32,
    pub max_vertical_velocity: f32,
    pub hover_damping: f32,
    pub angular_damping: f32,
    pub deceleration_multiplier: f32,
}

impl std::convert::Into<crate::data::movement_list::HoverCategoryData> for HoverCategoryData {
    fn into(self) -> crate::data::movement_list::HoverCategoryData {
        crate::data::movement_list::HoverCategoryData {
            height_tolerance: self.height_tolerance,
            force_y_offset: self.force_y_offset,
            turning_scale: self.turning_scale,
            small_angle_turning_scale: self.small_angle_turning_scale,
            max_vertical_velocity: self.max_vertical_velocity,
            hover_damping: self.hover_damping,
            angular_damping: self.angular_damping,
            deceleration_multiplier: self.deceleration_multiplier,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MechLegCategoryData {
    pub deceleration_multiplier: f32,
}

impl std::convert::Into<crate::data::movement_list::MechLegCategoryData> for MechLegCategoryData {
    fn into(self) -> crate::data::movement_list::MechLegCategoryData {
        crate::data::movement_list::MechLegCategoryData {
            deceleration_multiplier: self.deceleration_multiplier,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RotorCategoryData {
    pub max_turn_rate: f32,
}

impl std::convert::Into<crate::data::movement_list::RotorCategoryData> for RotorCategoryData {
    fn into(self) -> crate::data::movement_list::RotorCategoryData {
        crate::data::movement_list::RotorCategoryData {
            max_turn_rate: self.max_turn_rate,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovementData {
    pub speed_boost: Option<f32>,
    pub max_carry_mass: Option<f32>,
    pub horizontal_top_speed: Option<f32>,
    pub vertical_top_speed: Option<f32>,
    pub specifics: MovementSpecificData,
}

impl std::convert::Into<crate::data::movement_list::MovementData> for MovementData {
    fn into(self) -> crate::data::movement_list::MovementData {
        crate::data::movement_list::MovementData {
            speed_boost: self.speed_boost,
            max_carry_mass: self.max_carry_mass,
            horizontal_top_speed: self.horizontal_top_speed,
            vertical_top_speed: self.vertical_top_speed,
            specifics: self.specifics.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
}

impl std::convert::Into<crate::data::movement_list::MovementSpecificData> for MovementSpecificData {
    fn into(self) -> crate::data::movement_list::MovementSpecificData {
        match self {
            Self::Wheel(x) => crate::data::movement_list::MovementSpecificData::Wheel(x.into()),
            Self::Hover(x) => crate::data::movement_list::MovementSpecificData::Hover(x.into()),
            Self::Wing(x) => crate::data::movement_list::MovementSpecificData::Wing(x.into()),
            Self::Rudder(x) => crate::data::movement_list::MovementSpecificData::Rudder(x.into()),
            Self::Thruster(x) => crate::data::movement_list::MovementSpecificData::Thruster(x.into()),
            Self::Propeller(x) => crate::data::movement_list::MovementSpecificData::Propeller(x.into()),
            Self::InsectLeg(x) => crate::data::movement_list::MovementSpecificData::InsectLeg(x.into()),
            Self::MechLeg(x) => crate::data::movement_list::MovementSpecificData::MechLeg(x.into()),
            Self::SprinterLeg(x) => crate::data::movement_list::MovementSpecificData::SprinterLeg(x.into()),
            Self::TankTrack(x) => crate::data::movement_list::MovementSpecificData::TankTrack(x.into()),
            Self::Rotor(x) => crate::data::movement_list::MovementSpecificData::Rotor(x.into()),
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

impl std::convert::Into<crate::data::movement_list::WheelData> for WheelData {
    fn into(self) -> crate::data::movement_list::WheelData {
        crate::data::movement_list::WheelData {
            steering_speed_light: self.steering_speed_light,
            steering_speed_heavy: self.steering_speed_heavy,
            steering_force_multiplier_light: self.steering_force_multiplier_light,
            steering_force_multiplier_heavy: self.steering_force_multiplier_heavy,
            lateral_acceleration_light: self.lateral_acceleration_light,
            lateral_acceleration_heavy: self.lateral_acceleration_heavy,
            time_to_max_acceleration_light: self.time_to_max_acceleration_light,
            time_to_max_acceleration_heavy: self.time_to_max_acceleration_heavy,
            brake_force_light: self.brake_force_light,
            brake_force_heavy: self.brake_force_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HoverData {
    pub max_hover_height_light: f32,
    pub max_hover_height_heaver: f32,
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

impl std::convert::Into<crate::data::movement_list::HoverData> for HoverData {
    fn into(self) -> crate::data::movement_list::HoverData {
        crate::data::movement_list::HoverData {
            max_hover_height_light: self.max_hover_height_light,
            max_hover_height_heaver: self.max_hover_height_heaver,
            height_change_speed_light: self.height_change_speed_light,
            height_change_speed_heavy: self.height_change_speed_heavy,
            turn_torque_light: self.turn_torque_light,
            turn_torque_heavy: self.turn_torque_heavy,
            acceleration_light: self.acceleration_light,
            acceleration_heavy: self.acceleration_heavy,
            max_angular_velocity_light: self.max_angular_velocity_light,
            max_angular_velocity_heavy: self.max_angular_velocity_heavy,
            lateral_damping_light: self.lateral_damping_light,
            lateral_damping_heavy: self.lateral_damping_heavy,
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

impl std::convert::Into<crate::data::movement_list::AerofoilData> for AerofoilData {
    fn into(self) -> crate::data::movement_list::AerofoilData {
        crate::data::movement_list::AerofoilData {
            barrel_speed_light: self.barrel_speed_light,
            barrel_speed_heavy: self.barrel_speed_heavy,
            bank_speed_light: self.bank_speed_light,
            bank_speed_heavy: self.bank_speed_heavy,
            elevation_speed_light: self.elevation_speed_light,
            elevation_speed_heavy: self.elevation_speed_heavy,
            rudder_speed_light: self.rudder_speed_light,
            rudder_speed_heavy: self.rudder_speed_heavy,
            thrust_light: self.thrust_light,
            thrust_heavy: self.thrust_heavy,
            vtol_velocity_light: self.vtol_velocity_light,
            vtol_velocity_heavy: self.vtol_velocity_heavy,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ThrusterData {
    pub acceleration_delay_light: f32,
    pub acceleration_delay_heavy: f32,
}

impl std::convert::Into<crate::data::movement_list::ThrusterData> for ThrusterData {
    fn into(self) -> crate::data::movement_list::ThrusterData {
        crate::data::movement_list::ThrusterData {
            acceleration_delay_light: self.acceleration_delay_light,
            acceleration_delay_heavy: self.acceleration_delay_heavy,
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

impl std::convert::Into<crate::data::movement_list::InsectLegData> for InsectLegData {
    fn into(self) -> crate::data::movement_list::InsectLegData {
        crate::data::movement_list::InsectLegData {
            ideal_height_light: self.ideal_height_light,
            ideal_height_heavy: self.ideal_height_heavy,
            ideal_crouching_height_light: self.ideal_crouching_height_light,
            ideal_crouching_height_heavy: self.ideal_crouching_height_heavy,
            ideal_height_range_light: self.ideal_height_range_light,
            ideal_height_range_heavy: self.ideal_height_range_heavy,
            jump_height_light: self.jump_height_light,
            jump_height_heavy: self.jump_height_heavy,
            max_upwards_force_light: self.max_upwards_force_light,
            max_upwards_force_heavy: self.max_upwards_force_heavy,
            max_lateral_force_light: self.max_lateral_force_light,
            max_lateral_force_heavy: self.max_lateral_force_heavy,
            max_turning_force_light: self.max_turning_force_light,
            max_turning_force_heavy: self.max_turning_force_heavy,
            max_damping_force_light: self.max_damping_force_light,
            max_damping_force_heavy: self.max_damping_force_heavy,
            max_stopped_force_light: self.max_stopped_force_light,
            max_stopped_force_heavy: self.max_stopped_force_heavy,
            max_new_stopped_force_light: self.max_new_stopped_force_light,
            max_new_stopped_force_heavy: self.max_new_stopped_force_heavy,
            upwards_damping_force_light: self.upwards_damping_force_light,
            upwards_damping_force_heavy: self.upwards_damping_force_heavy,
            lateral_damp_force_light: self.lateral_damp_force_light,
            lateral_damp_force_heavy: self.lateral_damp_force_heavy,
            swagger_force_light: self.swagger_force_light,
            swagger_force_heavy: self.swagger_force_heavy,
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
    pub long_jump_speec_scale_light: f32,
    pub long_jump_speec_scale_heavy: f32,
    pub max_lateral_force_light: f32,
    pub max_lateral_force_heavy: f32,
    pub max_damping_force_light: f32,
    pub max_damping_force_heavy: f32,
}

impl std::convert::Into<crate::data::movement_list::MechLegData> for MechLegData {
    fn into(self) -> crate::data::movement_list::MechLegData {
        crate::data::movement_list::MechLegData {
            time_grounded_after_jump_light: self.time_grounded_after_jump_light,
            time_grounded_after_jump_heavy: self.time_grounded_after_jump_heavy,
            jump_height_light: self.jump_height_light,
            jump_height_heavy: self.jump_height_heavy,
            turn_acceleration_light: self.turn_acceleration_light,
            turn_acceleration_heavy: self.turn_acceleration_heavy,
            legacy_turn_acceleration_light: self.legacy_turn_acceleration_light,
            legacy_turn_acceleration_heavy: self.legacy_turn_acceleration_heavy,
            long_jump_speec_scale_light: self.long_jump_speec_scale_light,
            long_jump_speec_scale_heavy: self.long_jump_speec_scale_heavy,
            max_lateral_force_light: self.max_lateral_force_light,
            max_lateral_force_heavy: self.max_lateral_force_heavy,
            max_damping_force_light: self.max_damping_force_light,
            max_damping_force_heavy: self.max_damping_force_heavy,
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

impl std::convert::Into<crate::data::movement_list::TankTrackData> for TankTrackData {
    fn into(self) -> crate::data::movement_list::TankTrackData {
        crate::data::movement_list::TankTrackData {
            max_turn_rate_moving_light: self.max_turn_rate_moving_light,
            max_turn_rate_moving_heavy: self.max_turn_rate_moving_heavy,
            max_turn_rate_stopped_light: self.max_turn_rate_stopped_light,
            max_turn_rate_stopped_heavy: self.max_turn_rate_stopped_heavy,
            turn_acceleration_light: self.turn_acceleration_light,
            turn_acceleration_heavy: self.turn_acceleration_heavy,
            lateral_acceleration_light: self.lateral_acceleration_light,
            lateral_acceleration_heavy: self.lateral_acceleration_heavy,
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

impl std::convert::Into<crate::data::movement_list::RotorData> for RotorData {
    fn into(self) -> crate::data::movement_list::RotorData {
        crate::data::movement_list::RotorData {
            height_acceleration_light: self.height_acceleration_light,
            height_acceleration_heavy: self.height_acceleration_heavy,
            strafe_acceleration_light: self.strafe_acceleration_light,
            strafe_acceleration_heavy: self.strafe_acceleration_heavy,
            turn_acceleration_light: self.turn_acceleration_light,
            turn_acceleration_heavy: self.turn_acceleration_heavy,
            height_max_change_speed_light: self.height_max_change_speed_light,
            height_max_change_speed_heavy: self.height_max_change_speed_heavy,
            level_acceleration_light: self.level_acceleration_light,
            level_acceleration_heavy: self.level_acceleration_heavy,
        }
    }
}
