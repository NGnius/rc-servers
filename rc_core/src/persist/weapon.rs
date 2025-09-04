use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct WeaponData {
    pub damage_inflicted: Option<i32>,
    pub protonium_damage_scale: Option<f32>,
    pub projectile_speed: Option<f32>,
    pub projectile_range: Option<f32>,
    pub base_inaccuracy: Option<f32>,
    pub base_air_inaccuracy: Option<f32>,
    pub movement_inaccuracy: Option<f32>,
    #[serde(alias="movement_max_threshold_speed")]
    pub movement_max_speed: Option<f32>,
    #[serde(alias="movement_min_threshold_speed")]
    pub movement_min_speed: Option<f32>,
    #[serde(alias="gun_rotation_threshold_slow")]
    pub gun_rotation_slow: Option<f32>,
    #[serde(alias="movement_inaccuracy_decay_time")]
    pub movement_inaccuracy_decay: Option<f32>,
    #[serde(alias="slow_rotation_inaccuracy_decay_time")]
    pub slow_rotation_decay: Option<f32>,
    #[serde(alias="quick_rotation_inaccuracy_decay_time")]
    pub quick_rotation_decay: Option<f32>,
    #[serde(alias="movement_inaccuracy_recovery_time")]
    pub movement_inaccuracy_recovery: Option<f32>,
    pub repeat_fire_inaccuracy_total_degrees: Option<f32>,
    #[serde(alias="repeat_fire_inaccuracy_decay_time")]
    pub repeat_fire_inaccuracy_decay: Option<f32>,
    #[serde(alias="repeat_fire_inaccuracy_recovery_time")]
    pub repeat_fire_innaccuracy_recovery: Option<f32>,
    #[serde(alias="fire_instant_accuracy_decay_degrees")]
    pub fire_instant_accuracy_decay: Option<f32>, // degrees
    pub accuracy_non_recover_time: Option<f32>,
    #[serde(alias="accuracy_decay_time")]
    pub accuracy_decay: Option<f32>,
    pub damage_radius: Option<f32>,
    pub plasma_time_to_full_damage: Option<f32>,
    pub plasma_starting_radius_scale: Option<f32>,
    pub nano_dps: Option<f32>,
    pub nano_hps: Option<f32>,
    pub tesla_damage: Option<f32>,
    pub tesla_charges: Option<f32>,
    pub aeroflak_proximity_damage: Option<f32>,
    pub aeroflak_damage_radius: Option<f32>,
    pub aeroflak_explosion_radius: Option<f32>,
    pub aeroflak_ground_clearance: Option<f32>,
    #[serde(alias="aeroflak_buff_max_stacks")]
    pub aeroflak_max_stacks: Option<i32>,
    #[serde(alias="aeroflak_buff_damage_per_stack")]
    pub aeroflak_damage_per_stack: Option<i32>,
    #[serde(alias="aeroflak_buff_time_to_expire")]
    pub aeroflak_stack_expire: Option<f32>,
    #[serde(alias="cooldown_between_shots")]
    pub shot_cooldown: Option<f32>,
    pub smart_rotation_cooldown: Option<f32>,
    #[serde(alias="smart_rotation_extra_cooldown_time")]
    pub smart_rotation_cooldown_extra: Option<f32>,
    pub smart_rotation_max_stacks: Option<f32>,
    pub spin_up_time: Option<f32>,
    pub spin_down_time: Option<f32>,
    pub spin_initial_cooldown: Option<f32>,
    #[serde(default = "group_fire_scales_default")]
    pub group_fire_scales: Vec<f32>,
    pub mana_cost: Option<f32>,
    pub lock_time: Option<f32>,
    pub full_lock_release: Option<f32>,
    pub change_lock_time: Option<f32>,
    pub max_rotation_speed: Option<f32>,
    pub initial_rotation_speed: Option<f32>,
    pub rotation_acceleration: Option<f32>,
    pub nano_healing_priority_time: Option<f32>,
    pub module_range: Option<f32>,
    pub shield_lifetime: Option<f32>,
    pub teleport_time: Option<f32>,
    pub camera_time: Option<f32>,
    pub camera_delay: Option<f32>,
    pub to_invisible_speed: Option<f32>,
    pub to_invisible_duration: Option<f32>,
    pub to_visible_duration: Option<f32>,
    pub countdown_time: Option<f32>,
    pub stun_time: Option<f32>,
    pub stun_radius: Option<f32>,
    pub effect_duration: Option<f32>,
}

fn group_fire_scales_default() -> Vec<f32> {
    vec![1.0]
}

impl std::convert::From<WeaponData> for crate::data::weapon_list::WeaponData {
    fn from(val: WeaponData) -> Self {
        crate::data::weapon_list::WeaponData {
            damage_inflicted: val.damage_inflicted,
            protonium_damage_scale: val.protonium_damage_scale,
            projectile_speed: val.projectile_speed,
            projectile_range: val.projectile_range,
            base_inaccuracy: val.base_inaccuracy,
            base_air_inaccuracy: val.base_air_inaccuracy,
            movement_inaccuracy: val.movement_inaccuracy,
            movement_max_speed: val.movement_max_speed,
            movement_min_speed: val.movement_min_speed,
            gun_rotation_slow: val.gun_rotation_slow,
            movement_inaccuracy_decay: val.movement_inaccuracy_decay,
            slow_rotation_decay: val.slow_rotation_decay,
            quick_rotation_decay: val.quick_rotation_decay,
            movement_inaccuracy_recovery: val.movement_inaccuracy_recovery,
            repeat_fire_inaccuracy_total_degrees: val.repeat_fire_inaccuracy_total_degrees,
            repeat_fire_inaccuracy_decay: val.repeat_fire_inaccuracy_decay,
            repeat_fire_innaccuracy_recovery: val.repeat_fire_innaccuracy_recovery,
            fire_instant_accuracy_decay: val.fire_instant_accuracy_decay, // degrees
            accuracy_non_recover_time: val.accuracy_non_recover_time,
            accuracy_decay: val.accuracy_decay,
            damage_radius: val.damage_radius,
            plasma_time_to_full_damage: val.plasma_time_to_full_damage,
            plasma_starting_radius_scale: val.plasma_starting_radius_scale,
            nano_dps: val.nano_dps,
            nano_hps: val.nano_hps,
            tesla_damage: val.tesla_damage,
            tesla_charges: val.tesla_charges,
            aeroflak_proximity_damage: val.aeroflak_proximity_damage,
            aeroflak_damage_radius: val.aeroflak_damage_radius,
            aeroflak_explosion_radius: val.aeroflak_explosion_radius,
            aeroflak_ground_clearance: val.aeroflak_ground_clearance,
            aeroflak_max_stacks: val.aeroflak_max_stacks,
            aeroflak_damage_per_stack: val.aeroflak_damage_per_stack,
            aeroflak_stack_expire: val.aeroflak_stack_expire,
            shot_cooldown: val.shot_cooldown,
            smart_rotation_cooldown: val.smart_rotation_cooldown,
            smart_rotation_cooldown_extra: val.smart_rotation_cooldown_extra,
            smart_rotation_max_stacks: val.smart_rotation_max_stacks,
            spin_up_time: val.spin_up_time,
            spin_down_time: val.spin_down_time,
            spin_initial_cooldown: val.spin_initial_cooldown,
            group_fire_scales: val.group_fire_scales,
            mana_cost: val.mana_cost,
            lock_time: val.lock_time,
            full_lock_release: val.full_lock_release,
            change_lock_time: val.change_lock_time,
            max_rotation_speed: val.max_rotation_speed,
            initial_rotation_speed: val.initial_rotation_speed,
            rotation_acceleration: val.rotation_acceleration,
            nano_healing_priority_time: val.nano_healing_priority_time,
            module_range: val.module_range,
            shield_lifetime: val.shield_lifetime,
            teleport_time: val.teleport_time,
            camera_time: val.camera_time,
            camera_delay: val.camera_delay,
            to_invisible_speed: val.to_invisible_speed,
            to_invisible_duration: val.to_invisible_duration,
            to_visible_duration: val.to_visible_duration,
            countdown_time: val.countdown_time,
            stun_time: val.stun_time,
            stun_radius: val.stun_radius,
            effect_duration: val.effect_duration,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct WeaponUpgradeInfo {
    pub xp: f64,
    pub rating: i32,
    pub rank: i32,
    pub power: i32,
}

impl WeaponUpgradeInfo {
    pub fn into_data(self, tier: super::ItemTier, type_: super::ItemCategory) -> crate::data::weapon_upgrade::WeaponUpgradeInfo {
        crate::data::weapon_upgrade::WeaponUpgradeInfo {
            tier: tier.into(),
            type_: type_.into(),
            xp: self.xp,
            rating: self.rating,
            rank: self.rank,
            power: self.power,
        }
    }
}
