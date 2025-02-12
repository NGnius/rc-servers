#![allow(dead_code)]
use polariton::operation::Typed;

#[derive(Default)]
pub struct WeaponData {
    pub damage_inflicted: Option<i32>,
    pub protonium_damage_scale: Option<f32>,
    pub projectile_speed: Option<f32>,
    pub projectile_range: Option<f32>,
    pub base_inaccuracy: Option<f32>,
    pub base_air_inaccuracy: Option<f32>,
    pub movement_inaccuracy: Option<f32>,
    pub movement_max_speed: Option<f32>,
    pub movement_min_speed: Option<f32>,
    pub gun_rotation_slow: Option<f32>,
    pub movement_inaccuracy_decay: Option<f32>,
    pub slow_rotation_decay: Option<f32>,
    pub quick_rotation_decay: Option<f32>,
    pub movement_inaccuracy_recovery: Option<f32>,
    pub repeat_fire_inaccuracy_total_degrees: Option<f32>,
    pub repeat_fire_inaccuracy_decay: Option<f32>,
    pub repeat_fire_innaccuracy_recovery: Option<f32>,
    pub fire_instant_accuracy_decay: Option<f32>, // degrees
    pub accuracy_non_recover_time: Option<f32>,
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
    pub aeroflak_max_stacks: Option<i32>,
    pub aeroflak_damage_per_stack: Option<i32>,
    pub aeroflak_stack_expire: Option<f32>,
    pub shot_cooldown: Option<f32>,
    pub smart_rotation_cooldown: Option<f32>,
    pub smart_rotation_cooldown_extra: Option<f32>,
    pub smart_rotation_max_stacks: Option<f32>,
    pub spin_up_time: Option<f32>,
    pub spin_down_time: Option<f32>,
    pub spin_initial_cooldown: Option<f32>,
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

impl WeaponData {
    pub fn as_transmissible(&self) -> Typed {
        let mut out = Vec::new();

        self.damage_inflicted.map(|x| out.push((Typed::Str("damageInflicted".into()), Typed::Int(x))));
        self.protonium_damage_scale.map(|x| out.push((Typed::Str("protoniumDamageScale".into()), Typed::Float(x))));
        self.projectile_speed.map(|x| out.push((Typed::Str("projectileSpeed".into()), Typed::Float(x))));
        self.projectile_range.map(|x| out.push((Typed::Str("projectileRange".into()), Typed::Float(x))));
        self.base_inaccuracy.map(|x| out.push((Typed::Str("baseInaccuracy".into()), Typed::Float(x))));
        self.base_air_inaccuracy.map(|x| out.push((Typed::Str("baseAirInaccuracy".into()), Typed::Float(x))));
        self.movement_inaccuracy.map(|x| out.push((Typed::Str("movementInaccuracy".into()), Typed::Float(x))));
        self.movement_max_speed.map(|x| out.push((Typed::Str("movementMaxThresholdSpeed".into()), Typed::Float(x))));
        self.movement_min_speed.map(|x| out.push((Typed::Str("movementMinThresholdSpeed".into()), Typed::Float(x))));
        self.gun_rotation_slow.map(|x| out.push((Typed::Str("gunRotationThresholdSlow".into()), Typed::Float(x))));
        self.movement_inaccuracy_decay.map(|x| out.push((Typed::Str("movementInaccuracyDecayTime".into()), Typed::Float(x))));
        self.slow_rotation_decay.map(|x| out.push((Typed::Str("slowRotationInaccuracyDecayTime".into()), Typed::Float(x))));
        self.quick_rotation_decay.map(|x| out.push((Typed::Str("quickRotationInaccuracyDecayTime".into()), Typed::Float(x))));
        self.movement_inaccuracy_recovery.map(|x| out.push((Typed::Str("movementInaccuracyRecoveryTime".into()), Typed::Float(x))));
        self.repeat_fire_inaccuracy_total_degrees.map(|x| out.push((Typed::Str("repeatFireInaccuracyTotalDegrees".into()), Typed::Float(x))));
        self.repeat_fire_inaccuracy_decay.map(|x| out.push((Typed::Str("repeatFireInaccuracyDecayTime".into()), Typed::Float(x))));
        self.repeat_fire_innaccuracy_recovery.map(|x| out.push((Typed::Str("repeatFireInaccuracyRecoveryTime".into()), Typed::Float(x))));
        self.fire_instant_accuracy_decay.map(|x| out.push((Typed::Str("fireInstantAccuracyDecayDegrees".into()), Typed::Float(x)))); // degrees
        self.accuracy_non_recover_time.map(|x| out.push((Typed::Str("accuracyNonRecoverTime".into()), Typed::Float(x))));
        self.accuracy_decay.map(|x| out.push((Typed::Str("accuracyDecayTime".into()), Typed::Float(x))));
        self.damage_radius.map(|x| out.push((Typed::Str("damageRadius".into()), Typed::Float(x))));
        self.plasma_time_to_full_damage.map(|x| out.push((Typed::Str("plasmaTimeToFullDamage".into()), Typed::Float(x))));
        self.plasma_starting_radius_scale.map(|x| out.push((Typed::Str("plasmaStartingRadiusScale".into()), Typed::Float(x))));
        self.nano_dps.map(|x| out.push((Typed::Str("nanoDPS".into()), Typed::Float(x))));
        self.nano_hps.map(|x| out.push((Typed::Str("nanoHPS".into()), Typed::Float(x))));
        self.tesla_damage.map(|x| out.push((Typed::Str("teslaDamage".into()), Typed::Float(x))));
        self.tesla_charges.map(|x| out.push((Typed::Str("teslaCharges".into()), Typed::Float(x))));
        self.aeroflak_proximity_damage.map(|x| out.push((Typed::Str("aeroflakProximityDamage".into()), Typed::Float(x))));
        self.aeroflak_damage_radius.map(|x| out.push((Typed::Str("aeroflakDamageRadius".into()), Typed::Float(x))));
        self.aeroflak_explosion_radius.map(|x| out.push((Typed::Str("aeroflakExplosionRadius".into()), Typed::Float(x))));
        self.aeroflak_ground_clearance.map(|x| out.push((Typed::Str("aeroflakGroundClearance".into()), Typed::Float(x))));
        self.aeroflak_max_stacks.map(|x| out.push((Typed::Str("aeroflakBuffMaxStacks".into()), Typed::Int(x))));
        self.aeroflak_damage_per_stack.map(|x| out.push((Typed::Str("aeroflakBuffDamagePerStack".into()), Typed::Int(x))));
        self.aeroflak_stack_expire.map(|x| out.push((Typed::Str("aeroflakBuffTimeToExpire".into()), Typed::Float(x))));
        self.shot_cooldown.map(|x| out.push((Typed::Str("cooldownBetweenShots".into()), Typed::Float(x))));
        self.smart_rotation_cooldown.map(|x| out.push((Typed::Str("smartRotationCooldown".into()), Typed::Float(x))));
        self.smart_rotation_cooldown_extra.map(|x| out.push((Typed::Str("smartRotationExtraCooldownTime".into()), Typed::Float(x))));
        self.smart_rotation_max_stacks.map(|x| out.push((Typed::Str("smartRotationMaxStacks".into()), Typed::Float(x))));
        self.spin_up_time.map(|x| out.push((Typed::Str("spinUpTime".into()), Typed::Float(x))));
        self.spin_down_time.map(|x| out.push((Typed::Str("spinDownTime".into()), Typed::Float(x))));
        self.spin_initial_cooldown.map(|x| out.push((Typed::Str("spinInitialCooldown".into()), Typed::Float(x))));

        if !self.group_fire_scales.is_empty() {
            let typed_arr: Vec<Typed> = self.group_fire_scales.iter().map(|x| Typed::Float(*x)).collect();
            out.push((Typed::Str("groupFireScales".into()), Typed::ObjArr(typed_arr.into())));
        }

        self.mana_cost.map(|x| out.push((Typed::Str("manaCost".into()), Typed::Float(x))));
        self.lock_time.map(|x| out.push((Typed::Str("lockTime".into()), Typed::Float(x))));
        self.full_lock_release.map(|x| out.push((Typed::Str("fullLockRelease".into()), Typed::Float(x))));
        self.change_lock_time.map(|x| out.push((Typed::Str("changeLockTime".into()), Typed::Float(x))));
        self.max_rotation_speed.map(|x| out.push((Typed::Str("maxRotationSpeed".into()), Typed::Float(x))));
        self.initial_rotation_speed.map(|x| out.push((Typed::Str("initialRotationSpeed".into()), Typed::Float(x))));
        self.rotation_acceleration.map(|x| out.push((Typed::Str("rotationAcceleration".into()), Typed::Float(x))));
        self.nano_healing_priority_time.map(|x| out.push((Typed::Str("nanoHealingPriorityTime".into()), Typed::Float(x))));
        self.module_range.map(|x| out.push((Typed::Str("moduleRange".into()), Typed::Float(x))));
        self.shield_lifetime.map(|x| out.push((Typed::Str("shieldLifetime".into()), Typed::Float(x))));
        self.teleport_time.map(|x| out.push((Typed::Str("teleportTime".into()), Typed::Float(x))));
        self.camera_time.map(|x| out.push((Typed::Str("cameraTime".into()), Typed::Float(x))));
        self.camera_delay.map(|x| out.push((Typed::Str("cameraDelay".into()), Typed::Float(x))));
        self.to_invisible_speed.map(|x| out.push((Typed::Str("toInvisibleSpeed".into()), Typed::Float(x))));
        self.to_invisible_duration.map(|x| out.push((Typed::Str("toInvisibleDuration".into()), Typed::Float(x))));
        self.to_visible_duration.map(|x| out.push((Typed::Str("toVisibleDuration".into()), Typed::Float(x))));
        self.countdown_time.map(|x| out.push((Typed::Str("countdownTime".into()), Typed::Float(x))));
        self.stun_time.map(|x| out.push((Typed::Str("stunTime".into()), Typed::Float(x))));
        self.stun_radius.map(|x| out.push((Typed::Str("stunRadius".into()), Typed::Float(x))));
        self.effect_duration.map(|x| out.push((Typed::Str("effectDuration".into()), Typed::Float(x))));
        Typed::HashMap(out.into())
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ItemCategory {
    NoFunction,
    Wheel,
    Hover,
    Wing,
    Rudder,
    Thruster,
    InsectLeg,
    MechLeg,
    Ski,
    TankTrack,
    Rotor,
    SrpinterLeg,
    Propeller,
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
    GhostModule,
    BlinkModule,
    EmpModule,
    WindowmakerModule,
    EnergyModule = 900,
}

impl ItemCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemCategory::NoFunction => "NotAFunctionalItem",
            ItemCategory::Wheel => "Wheel",
            ItemCategory::Hover => "Hover",
            ItemCategory::Wing => "Wing",
            ItemCategory::Rudder => "Rudder",
            ItemCategory::Thruster => "Thruster",
            ItemCategory::InsectLeg => "InsectLeg",
            ItemCategory::MechLeg => "MechLeg",
            ItemCategory::Ski => "Ski",
            ItemCategory::TankTrack => "TankTrack",
            ItemCategory::Rotor => "Rotor",
            ItemCategory::SrpinterLeg => "SrpinterLeg",
            ItemCategory::Propeller => "Propeller",
            ItemCategory::Laser => "Laser",
            ItemCategory::Plasma => "Plasma",
            ItemCategory::Mortar => "Mortar",
            ItemCategory::Rail => "Rail",
            ItemCategory::Nano => "Nano",
            ItemCategory::Tesla => "Tesla",
            ItemCategory::Aeroflak => "Aeroflak",
            ItemCategory::Ion => "Ion",
            ItemCategory::Seeker => "Seeker",
            ItemCategory::Chaingun => "Chaingun",
            ItemCategory::ShieldModule => "ShieldModule",
            ItemCategory::GhostModule => "GhostModule",
            ItemCategory::BlinkModule => "BlinkModule",
            ItemCategory::EmpModule => "EmpModule",
            ItemCategory::WindowmakerModule => "WindowmakerModule",
            ItemCategory::EnergyModule => "EnergyModule",
        }
    }
}
