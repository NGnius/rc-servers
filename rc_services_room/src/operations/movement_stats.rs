//use polariton::serdes::TypePrefix;
use polariton_server::operations::Immediate;
//use polariton::operation::{ParameterTable, Typed, Dict};

//use crate::data::movement_list::*;
//use crate::data::cube_list::ItemTier;
//use crate::data::weapon_list::ItemCategory;

const PARAM_KEY: u8 = 1;

pub(super) fn movement_config_provider(cubes: &crate::persist::config::CubeConfig) -> Immediate<62, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(PARAM_KEY, cubes.movement_list());
        /*params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::HashMap, // hashtable
            items: vec![
                (Typed::Str("Global".into()), Typed::HashMap(vec![
                    (Typed::Str("lerpValue".into()), Typed::Float(10.0)),
                ].into())),
                (Typed::Str("Movements".into()), Typed::HashMap(vec![
                    (Typed::Str(ItemCategory::Wheel.as_str().into()), MovementCategoryData {
                        horizontal_top_speed: Some(1.0),
                        vertical_top_speed: Some(1.0),
                        specifics: MovementCategorySpecificData::Wheel,
                        stats: vec![
                            (ItemTier::T0, MovementData {
                                speed_boost: Some(1.0),
                                max_carry_mass: Some(1.0),
                                horizontal_top_speed: Some(1.0),
                                vertical_top_speed: Some(1.0),
                                specifics: MovementSpecificData::Wheel(WheelData {
                                    steering_speed_light: 1.0,
                                    steering_speed_heavy: 1.0,
                                    steering_force_multiplier_light: 1.0,
                                    steering_force_multiplier_heavy: 1.0,
                                    lateral_acceleration_light: 1.0,
                                    lateral_acceleration_heavy: 1.0,
                                    time_to_max_acceleration_light: 1.0,
                                    time_to_max_acceleration_heavy: 1.0,
                                    brake_force_light: 1.0,
                                    brake_force_heavy: 1.0,
                                }),
                            }),
                            (ItemTier::T1, MovementData {
                                speed_boost: Some(1.0),
                                max_carry_mass: Some(1.0),
                                horizontal_top_speed: Some(1.0),
                                vertical_top_speed: Some(1.0),
                                specifics: MovementSpecificData::Wheel(WheelData {
                                    steering_speed_light: 1.0,
                                    steering_speed_heavy: 1.0,
                                    steering_force_multiplier_light: 1.0,
                                    steering_force_multiplier_heavy: 1.0,
                                    lateral_acceleration_light: 1.0,
                                    lateral_acceleration_heavy: 1.0,
                                    time_to_max_acceleration_light: 1.0,
                                    time_to_max_acceleration_heavy: 1.0,
                                    brake_force_light: 1.0,
                                    brake_force_heavy: 1.0,
                                }),
                            }),
                            (ItemTier::T2, MovementData {
                                speed_boost: Some(1.0),
                                max_carry_mass: Some(1.0),
                                horizontal_top_speed: Some(1.0),
                                vertical_top_speed: Some(1.0),
                                specifics: MovementSpecificData::Wheel(WheelData {
                                    steering_speed_light: 1.0,
                                    steering_speed_heavy: 1.0,
                                    steering_force_multiplier_light: 1.0,
                                    steering_force_multiplier_heavy: 1.0,
                                    lateral_acceleration_light: 1.0,
                                    lateral_acceleration_heavy: 1.0,
                                    time_to_max_acceleration_light: 1.0,
                                    time_to_max_acceleration_heavy: 1.0,
                                    brake_force_light: 1.0,
                                    brake_force_heavy: 1.0,
                                }),
                            }),
                            (ItemTier::T3, MovementData {
                                speed_boost: Some(1.0),
                                max_carry_mass: Some(1.0),
                                horizontal_top_speed: Some(1.0),
                                vertical_top_speed: Some(1.0),
                                specifics: MovementSpecificData::Wheel(WheelData {
                                    steering_speed_light: 1.0,
                                    steering_speed_heavy: 1.0,
                                    steering_force_multiplier_light: 1.0,
                                    steering_force_multiplier_heavy: 1.0,
                                    lateral_acceleration_light: 1.0,
                                    lateral_acceleration_heavy: 1.0,
                                    time_to_max_acceleration_light: 1.0,
                                    time_to_max_acceleration_heavy: 1.0,
                                    brake_force_light: 1.0,
                                    brake_force_heavy: 1.0,
                                }),
                            }),
                            (ItemTier::T4, MovementData {
                                speed_boost: Some(1.0),
                                max_carry_mass: Some(1.0),
                                horizontal_top_speed: Some(1.0),
                                vertical_top_speed: Some(1.0),
                                specifics: MovementSpecificData::Wheel(WheelData {
                                    steering_speed_light: 1.0,
                                    steering_speed_heavy: 1.0,
                                    steering_force_multiplier_light: 1.0,
                                    steering_force_multiplier_heavy: 1.0,
                                    lateral_acceleration_light: 1.0,
                                    lateral_acceleration_heavy: 1.0,
                                    time_to_max_acceleration_light: 1.0,
                                    time_to_max_acceleration_heavy: 1.0,
                                    brake_force_light: 1.0,
                                    brake_force_heavy: 1.0,
                                }),
                            }),
                            (ItemTier::T5, MovementData {
                                speed_boost: Some(1.0),
                                max_carry_mass: Some(1.0),
                                horizontal_top_speed: Some(1.0),
                                vertical_top_speed: Some(1.0),
                                specifics: MovementSpecificData::Wheel(WheelData {
                                    steering_speed_light: 1.0,
                                    steering_speed_heavy: 1.0,
                                    steering_force_multiplier_light: 1.0,
                                    steering_force_multiplier_heavy: 1.0,
                                    lateral_acceleration_light: 1.0,
                                    lateral_acceleration_heavy: 1.0,
                                    time_to_max_acceleration_light: 1.0,
                                    time_to_max_acceleration_heavy: 1.0,
                                    brake_force_light: 1.0,
                                    brake_force_heavy: 1.0,
                                }),
                            }),
                        ],
                        ..Default::default()
                    }.as_transmissible()),
                ].into())),
            ].into(),
        }));*/
        params.into()
    })
}
