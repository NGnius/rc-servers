#![allow(dead_code)]

use polariton::operation::{Typed, Dict};
use polariton::serdes::TypePrefix;

use super::cube_list::ItemTier;

#[derive(Default)]
pub struct MovementCategoryData {
    pub horizontal_top_speed: Option<f32>,
    pub vertical_top_speed: Option<f32>,
    pub min_required_items: Option<i32>,
    pub min_item_modifier: Option<f32>,
    pub max_hover_height: Option<f32>,
    pub light_machine_mass: Option<f32>,
    pub heavy_machine_mass: Option<f32>,
    pub specifics: MovementCategorySpecificData,
    pub stats: Vec<(ItemTier, MovementData)>,
}

impl MovementCategoryData {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        let mut out = Vec::new();
        self.horizontal_top_speed.map(|x| out.push((Typed::Str("horizontalTopSpeed".into()), Typed::Float(x))));
        self.vertical_top_speed.map(|x| out.push((Typed::Str("verticalTopSpeed".into()), Typed::Float(x))));
        self.min_required_items.map(|x| out.push((Typed::Str("minRequiredItems".into()), Typed::Int(x))));
        self.min_item_modifier.map(|x| out.push((Typed::Str("minItemsModifier".into()), Typed::Float(x))));
        self.max_hover_height.map(|x| out.push((Typed::Str("maxHoverHeight".into()), Typed::Float(x))));
        self.light_machine_mass.map(|x| out.push((Typed::Str("lightMachineMass".into()), Typed::Float(x))));
        self.heavy_machine_mass.map(|x| out.push((Typed::Str("heavyMachineMass".into()), Typed::Float(x))));
        out.append(&mut self.specifics.as_transmissible());
        for (tier, mov_data) in self.stats.iter() {
            out.push((Typed::Str(tier.as_str().into()), mov_data.as_transmissible()));
        }
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items: out.into(),
        })
    }
}

#[derive(Default)]
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

impl MovementCategorySpecificData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        match self {
            Self::Wheel => Vec::default(),
            Self::Hover(x) => x.as_transmissible(),
            Self::Wing => Vec::default(),
            Self::Rudder => Vec::default(),
            Self::Thruster => Vec::default(),
            Self::Propeller => Vec::default(),
            Self::InsectLeg => Vec::default(),
            Self::MechLeg(x) => x.as_transmissible(),
            Self::SprinterLeg(x) => x.as_transmissible(),
            Self::TankTrack => Vec::default(),
            Self::Rotor(x) => x.as_transmissible(),
        }
    }
}

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

impl HoverCategoryData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("heightTolerance".into()), Typed::Float(self.height_tolerance)),
            (Typed::Str("forceYOffset".into()), Typed::Float(self.force_y_offset)),
            (Typed::Str("turningScale".into()), Typed::Float(self.turning_scale)),
            (Typed::Str("smallAngleTurningScale".into()), Typed::Float(self.small_angle_turning_scale)),
            (Typed::Str("verticalTopSpeed".into()), Typed::Float(self.max_vertical_velocity)),
            (Typed::Str("hoverDamping".into()), Typed::Float(self.hover_damping)),
            (Typed::Str("angularDamping".into()), Typed::Float(self.angular_damping)),
            (Typed::Str("decelerationMultiplier".into()), Typed::Float(self.deceleration_multiplier)),
        ]
    }
}

pub struct MechLegCategoryData {
    pub deceleration_multiplier: f32,
}

impl MechLegCategoryData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("decelerationMultiplier".into()), Typed::Float(self.deceleration_multiplier)),
        ]
    }
}

pub struct RotorCategoryData {
    pub max_turn_rate: f32,
}


impl RotorCategoryData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("maxTurnRate".into()), Typed::Float(self.max_turn_rate)),
        ]
    }
}

pub struct MovementData {
    pub speed_boost: Option<f32>,
    pub max_carry_mass: Option<f32>,
    pub horizontal_top_speed: Option<f32>,
    pub vertical_top_speed: Option<f32>,
    pub specifics: MovementSpecificData,
}

impl MovementData {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        let mut out = Vec::new();
        self.speed_boost.map(|x| out.push((Typed::Str("speedBoost".into()), Typed::Float(x))));
        self.max_carry_mass.map(|x| out.push((Typed::Str("maxCarryMass".into()), Typed::Float(x))));
        self.horizontal_top_speed.map(|x| out.push((Typed::Str("horizontalTopSpeed".into()), Typed::Float(x))));
        self.vertical_top_speed.map(|x| out.push((Typed::Str("verticalTopSpeed".into()), Typed::Float(x))));
        out.append(&mut self.specifics.as_transmissible());
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::Any, // any
            items: out.into(),
        })
    }
}

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

impl MovementSpecificData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        match self {
            Self::Wheel(x) => x.as_transmissible(),
            Self::Hover(x) => x.as_transmissible(),
            Self::Wing(x) => x.as_transmissible(),
            Self::Rudder(x) => x.as_transmissible(),
            Self::Thruster(x) => x.as_transmissible(),
            Self::Propeller(x) => x.as_transmissible(),
            Self::InsectLeg(x) => x.as_transmissible(),
            Self::MechLeg(x) => x.as_transmissible(),
            Self::SprinterLeg(x) => x.as_transmissible(),
            Self::TankTrack(x) => x.as_transmissible(),
            Self::Rotor(x) => x.as_transmissible(),
        }
    }
}

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

impl WheelData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("steeringSpeedLight".into()), Typed::Float(self.steering_speed_light)),
            (Typed::Str("steeringSpeedHeavy".into()), Typed::Float(self.steering_speed_heavy)),
            (Typed::Str("steeringForceMultiplierLight".into()), Typed::Float(self.steering_force_multiplier_light)),
            (Typed::Str("steeringForceMultiplierHeavy".into()), Typed::Float(self.steering_force_multiplier_heavy)),
            (Typed::Str("lateralAccelerationLight".into()), Typed::Float(self.lateral_acceleration_light)),
            (Typed::Str("lateralAccelerationHeavy".into()), Typed::Float(self.lateral_acceleration_heavy)),
            (Typed::Str("timeToMaxAccelerationLight".into()), Typed::Float(self.time_to_max_acceleration_light)),
            (Typed::Str("timeToMaxAccelerationHeavy".into()), Typed::Float(self.time_to_max_acceleration_heavy)),
            (Typed::Str("brakeForceLight".into()), Typed::Float(self.brake_force_light)),
            (Typed::Str("brakeForceHeavy".into()), Typed::Float(self.brake_force_heavy)),
        ]
    }
}

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

impl HoverData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("maxHoverHeightLight".into()), Typed::Float(self.max_hover_height_light)),
            (Typed::Str("maxHoverHeightHeavy".into()), Typed::Float(self.max_hover_height_heaver)),
            (Typed::Str("heightChangeSpeedLight".into()), Typed::Float(self.height_change_speed_light)),
            (Typed::Str("heightChangeSpeedHeavy".into()), Typed::Float(self.height_change_speed_heavy)),
            (Typed::Str("turnTorqueLight".into()), Typed::Float(self.turn_torque_light)),
            (Typed::Str("turnTorqueHeavy".into()), Typed::Float(self.turn_torque_heavy)),
            (Typed::Str("accelerationLight".into()), Typed::Float(self.acceleration_light)),
            (Typed::Str("accelerationHeavy".into()), Typed::Float(self.acceleration_heavy)),
            (Typed::Str("maxAngularVelocityLight".into()), Typed::Float(self.max_angular_velocity_light)),
            (Typed::Str("maxAngularVelocityHeavy".into()), Typed::Float(self.max_angular_velocity_heavy)),
            (Typed::Str("lateralDampingLight".into()), Typed::Float(self.lateral_damping_light)),
            (Typed::Str("lateralDampingHeavy".into()), Typed::Float(self.lateral_damping_heavy)),
        ]
    }
}

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

impl AerofoilData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("barrelSpeedLight".into()), Typed::Float(self.barrel_speed_light)),
            (Typed::Str("barrelSpeedHeavy".into()), Typed::Float(self.barrel_speed_heavy)),
            (Typed::Str("bankSpeedLight".into()), Typed::Float(self.bank_speed_light)),
            (Typed::Str("bankSpeedHeavy".into()), Typed::Float(self.bank_speed_heavy)),
            (Typed::Str("elevationSpeedLight".into()), Typed::Float(self.elevation_speed_light)),
            (Typed::Str("elevationSpeedHeavy".into()), Typed::Float(self.elevation_speed_heavy)),
            (Typed::Str("rudderSpeedLight".into()), Typed::Float(self.rudder_speed_light)),
            (Typed::Str("rudderSpeedHeavy".into()), Typed::Float(self.rudder_speed_heavy)),
            (Typed::Str("thrustLight".into()), Typed::Float(self.thrust_light)),
            (Typed::Str("thrustHeavy".into()), Typed::Float(self.thrust_heavy)),
            (Typed::Str("vtolVelocityLight".into()), Typed::Float(self.vtol_velocity_light)),
            (Typed::Str("vtolVelocityHeavy".into()), Typed::Float(self.vtol_velocity_heavy)),
        ]
    }
}

pub struct ThrusterData {
    pub acceleration_delay_light: f32,
    pub acceleration_delay_heavy: f32,
}

impl ThrusterData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("accelerationDelayLight".into()), Typed::Float(self.acceleration_delay_light)),
            (Typed::Str("accelerationDelayHeavy".into()), Typed::Float(self.acceleration_delay_heavy)),
        ]
    }
}

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

impl InsectLegData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("idealHeightLight".into()), Typed::Float(self.ideal_height_light)),
            (Typed::Str("idealHeightHeavy".into()), Typed::Float(self.ideal_height_heavy)),
            (Typed::Str("idealCrouchingHeightLight".into()), Typed::Float(self.ideal_crouching_height_light)),
            (Typed::Str("idealCrouchingHeightHeavy".into()), Typed::Float(self.ideal_crouching_height_heavy)),
            (Typed::Str("idealHeightRangeLight".into()), Typed::Float(self.ideal_height_range_light)),
            (Typed::Str("idealHeightRangeHeavy".into()), Typed::Float(self.ideal_height_range_heavy)),
            (Typed::Str("jumpHeightLight".into()), Typed::Float(self.jump_height_light)),
            (Typed::Str("jumpHeightHeavy".into()), Typed::Float(self.jump_height_heavy)),
            (Typed::Str("maxUpwardsForceLight".into()), Typed::Float(self.max_upwards_force_light)),
            (Typed::Str("maxUpwardsForceHeavy".into()), Typed::Float(self.max_upwards_force_heavy)),
            (Typed::Str("maxLateralForceLight".into()), Typed::Float(self.max_lateral_force_light)),
            (Typed::Str("maxLateralForceHeavy".into()), Typed::Float(self.max_lateral_force_heavy)),
            (Typed::Str("maxTurningForceLight".into()), Typed::Float(self.max_turning_force_light)),
            (Typed::Str("maxTurningForceHeavy".into()), Typed::Float(self.max_turning_force_heavy)),
            (Typed::Str("maxDampingForceLight".into()), Typed::Float(self.max_damping_force_light)),
            (Typed::Str("maxDampingForceHeavy".into()), Typed::Float(self.max_damping_force_heavy)),
            (Typed::Str("maxStoppedForceLight".into()), Typed::Float(self.max_stopped_force_light)),
            (Typed::Str("maxStoppedForceHeavy".into()), Typed::Float(self.max_stopped_force_heavy)),
            (Typed::Str("maxNewStoppedForceLight".into()), Typed::Float(self.max_new_stopped_force_light)),
            (Typed::Str("maxNewStoppedForceHeavy".into()), Typed::Float(self.max_new_stopped_force_heavy)),
            (Typed::Str("upwardsDampingForceLight".into()), Typed::Float(self.upwards_damping_force_light)),
            (Typed::Str("upwardsDampingForceHeavy".into()), Typed::Float(self.upwards_damping_force_heavy)),
            (Typed::Str("lateralDampForceLight".into()), Typed::Float(self.lateral_damp_force_light)),
            (Typed::Str("lateralDampForceHeavy".into()), Typed::Float(self.lateral_damp_force_heavy)),
            (Typed::Str("swaggerForceLight".into()), Typed::Float(self.swagger_force_light)),
            (Typed::Str("swaggerForceHeavy".into()), Typed::Float(self.swagger_force_heavy)),
        ]
    }
}

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

impl MechLegData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("timeGroundedAfterJumpLight".into()), Typed::Float(self.time_grounded_after_jump_light)),
            (Typed::Str("timeGroundedAfterJumpHeavy".into()), Typed::Float(self.time_grounded_after_jump_heavy)),
            (Typed::Str("jumpHeightLight".into()), Typed::Float(self.jump_height_light)),
            (Typed::Str("jumpHeightHeavy".into()), Typed::Float(self.jump_height_heavy)),
            (Typed::Str("turnAccelerationLight".into()), Typed::Float(self.turn_acceleration_light)),
            (Typed::Str("turnAccelerationHeavy".into()), Typed::Float(self.turn_acceleration_heavy)),
            (Typed::Str("legacyTurnAccelerationLight".into()), Typed::Float(self.legacy_turn_acceleration_light)),
            (Typed::Str("legacyTurnAccelerationHeavy".into()), Typed::Float(self.legacy_turn_acceleration_heavy)),
            (Typed::Str("longJumpSpeedScaleLight".into()), Typed::Float(self.long_jump_speec_scale_light)),
            (Typed::Str("longJumpSpeedScaleHeavy".into()), Typed::Float(self.long_jump_speec_scale_heavy)),
            (Typed::Str("maxLateralForceLight".into()), Typed::Float(self.max_lateral_force_light)),
            (Typed::Str("maxLateralForceHeavy".into()), Typed::Float(self.max_lateral_force_heavy)),
            (Typed::Str("maxDampingForceLight".into()), Typed::Float(self.max_damping_force_light)),
            (Typed::Str("maxDampingForceHeavy".into()), Typed::Float(self.max_damping_force_heavy)),
        ]
    }
}

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

impl TankTrackData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("maxTurnRateMovingLight".into()), Typed::Float(self.max_turn_rate_moving_light)),
            (Typed::Str("maxTurnRateMovingHeavy".into()), Typed::Float(self.max_turn_rate_moving_heavy)),
            (Typed::Str("maxTurnRateStoppedLight".into()), Typed::Float(self.max_turn_rate_stopped_light)),
            (Typed::Str("maxTurnRateStoppedHeavy".into()), Typed::Float(self.max_turn_rate_stopped_heavy)),
            (Typed::Str("turnAccelerationLight".into()), Typed::Float(self.turn_acceleration_light)),
            (Typed::Str("turnAccelerationHeavy".into()), Typed::Float(self.turn_acceleration_heavy)),
            (Typed::Str("lateralAccelerationLight".into()), Typed::Float(self.lateral_acceleration_light)),
            (Typed::Str("lateralAccelerationHeavy".into()), Typed::Float(self.lateral_acceleration_heavy)),
        ]
    }
}

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


impl RotorData {
    pub fn as_transmissible<C>(&self) -> Vec<(Typed<C>, Typed<C>)> {
        vec![
            (Typed::Str("heightAccelerationLight".into()), Typed::Float(self.height_acceleration_light)),
            (Typed::Str("heightAccelerationHeavy".into()), Typed::Float(self.height_acceleration_heavy)),
            (Typed::Str("strafeAccelerationLight".into()), Typed::Float(self.strafe_acceleration_light)),
            (Typed::Str("strafeAccelerationHeavy".into()), Typed::Float(self.strafe_acceleration_heavy)),
            (Typed::Str("turnAccelerationLight".into()), Typed::Float(self.turn_acceleration_light)),
            (Typed::Str("turnAccelerationHeavy".into()), Typed::Float(self.turn_acceleration_heavy)),
            (Typed::Str("heightMaxChangeSpeedLight".into()), Typed::Float(self.height_max_change_speed_light)),
            (Typed::Str("heightMaxChangeSpeedHeavy".into()), Typed::Float(self.height_max_change_speed_heavy)),
            (Typed::Str("levelAccelerationLight".into()), Typed::Float(self.level_acceleration_light)),
            (Typed::Str("levelAccelerationHeavy".into()), Typed::Float(self.level_acceleration_heavy)),
        ]
    }
}
