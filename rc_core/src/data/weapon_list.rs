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
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        let mut out = Vec::new();

        if let Some(x) = self.damage_inflicted { out.push((Typed::Str("damageInflicted".into()), Typed::Int(x))) }
        if let Some(x) = self.protonium_damage_scale { out.push((Typed::Str("protoniumDamageScale".into()), Typed::Float(x))) }
        if let Some(x) = self.projectile_speed { out.push((Typed::Str("projectileSpeed".into()), Typed::Float(x))) }
        if let Some(x) = self.projectile_range { out.push((Typed::Str("projectileRange".into()), Typed::Float(x))) }
        if let Some(x) = self.base_inaccuracy { out.push((Typed::Str("baseInaccuracy".into()), Typed::Float(x))) }
        if let Some(x) = self.base_air_inaccuracy { out.push((Typed::Str("baseAirInaccuracy".into()), Typed::Float(x))) }
        if let Some(x) = self.movement_inaccuracy { out.push((Typed::Str("movementInaccuracy".into()), Typed::Float(x))) }
        if let Some(x) = self.movement_max_speed { out.push((Typed::Str("movementMaxThresholdSpeed".into()), Typed::Float(x))) }
        if let Some(x) = self.movement_min_speed { out.push((Typed::Str("movementMinThresholdSpeed".into()), Typed::Float(x))) }
        if let Some(x) = self.gun_rotation_slow { out.push((Typed::Str("gunRotationThresholdSlow".into()), Typed::Float(x))) }
        if let Some(x) = self.movement_inaccuracy_decay { out.push((Typed::Str("movementInaccuracyDecayTime".into()), Typed::Float(x))) }
        if let Some(x) = self.slow_rotation_decay { out.push((Typed::Str("slowRotationInaccuracyDecayTime".into()), Typed::Float(x))) }
        if let Some(x) = self.quick_rotation_decay { out.push((Typed::Str("quickRotationInaccuracyDecayTime".into()), Typed::Float(x))) }
        if let Some(x) = self.movement_inaccuracy_recovery { out.push((Typed::Str("movementInaccuracyRecoveryTime".into()), Typed::Float(x))) }
        if let Some(x) = self.repeat_fire_inaccuracy_total_degrees { out.push((Typed::Str("repeatFireInaccuracyTotalDegrees".into()), Typed::Float(x))) }
        if let Some(x) = self.repeat_fire_inaccuracy_decay { out.push((Typed::Str("repeatFireInaccuracyDecayTime".into()), Typed::Float(x))) }
        if let Some(x) = self.repeat_fire_innaccuracy_recovery { out.push((Typed::Str("repeatFireInaccuracyRecoveryTime".into()), Typed::Float(x))) }
        if let Some(x) = self.fire_instant_accuracy_decay { out.push((Typed::Str("fireInstantAccuracyDecayDegrees".into()), Typed::Float(x))) } // degrees
        if let Some(x) = self.accuracy_non_recover_time { out.push((Typed::Str("accuracyNonRecoverTime".into()), Typed::Float(x))) }
        if let Some(x) = self.accuracy_decay { out.push((Typed::Str("accuracyDecayTime".into()), Typed::Float(x))) }
        if let Some(x) = self.damage_radius { out.push((Typed::Str("damageRadius".into()), Typed::Float(x))) }
        if let Some(x) = self.plasma_time_to_full_damage { out.push((Typed::Str("plasmaTimeToFullDamage".into()), Typed::Float(x))) }
        if let Some(x) = self.plasma_starting_radius_scale { out.push((Typed::Str("plasmaStartingRadiusScale".into()), Typed::Float(x))) }
        if let Some(x) = self.nano_dps { out.push((Typed::Str("nanoDPS".into()), Typed::Float(x))) }
        if let Some(x) = self.nano_hps { out.push((Typed::Str("nanoHPS".into()), Typed::Float(x))) }
        if let Some(x) = self.tesla_damage { out.push((Typed::Str("teslaDamage".into()), Typed::Float(x))) }
        if let Some(x) = self.tesla_charges { out.push((Typed::Str("teslaCharges".into()), Typed::Float(x))) }
        if let Some(x) = self.aeroflak_proximity_damage { out.push((Typed::Str("aeroflakProximityDamage".into()), Typed::Float(x))) }
        if let Some(x) = self.aeroflak_damage_radius { out.push((Typed::Str("aeroflakDamageRadius".into()), Typed::Float(x))) }
        if let Some(x) = self.aeroflak_explosion_radius { out.push((Typed::Str("aeroflakExplosionRadius".into()), Typed::Float(x))) }
        if let Some(x) = self.aeroflak_ground_clearance { out.push((Typed::Str("aeroflakGroundClearance".into()), Typed::Float(x))) }
        if let Some(x) = self.aeroflak_max_stacks { out.push((Typed::Str("aeroflakBuffMaxStacks".into()), Typed::Int(x))) }
        if let Some(x) = self.aeroflak_damage_per_stack { out.push((Typed::Str("aeroflakBuffDamagePerStack".into()), Typed::Int(x))) }
        if let Some(x) = self.aeroflak_stack_expire { out.push((Typed::Str("aeroflakBuffTimeToExpire".into()), Typed::Float(x))) }
        if let Some(x) = self.shot_cooldown { out.push((Typed::Str("cooldownBetweenShots".into()), Typed::Float(x))) }
        if let Some(x) = self.smart_rotation_cooldown { out.push((Typed::Str("smartRotationCooldown".into()), Typed::Float(x))) }
        if let Some(x) = self.smart_rotation_cooldown_extra { out.push((Typed::Str("smartRotationExtraCooldownTime".into()), Typed::Float(x))) }
        if let Some(x) = self.smart_rotation_max_stacks { out.push((Typed::Str("smartRotationMaxStacks".into()), Typed::Float(x))) }
        if let Some(x) = self.spin_up_time { out.push((Typed::Str("spinUpTime".into()), Typed::Float(x))) }
        if let Some(x) = self.spin_down_time { out.push((Typed::Str("spinDownTime".into()), Typed::Float(x))) }
        if let Some(x) = self.spin_initial_cooldown { out.push((Typed::Str("spinInitialCooldown".into()), Typed::Float(x))) }

        if !self.group_fire_scales.is_empty() {
            let typed_arr: Vec<Typed<C>> = self.group_fire_scales.iter().map(|x| Typed::Float(*x)).collect();
            out.push((Typed::Str("groupFireScales".into()), Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Float,
                custom_ty: None,
                items: typed_arr,
            })));
        }

        if let Some(x) = self.mana_cost { out.push((Typed::Str("manaCost".into()), Typed::Float(x))) }
        if let Some(x) = self.lock_time { out.push((Typed::Str("lockTime".into()), Typed::Float(x))) }
        if let Some(x) = self.full_lock_release { out.push((Typed::Str("fullLockRelease".into()), Typed::Float(x))) }
        if let Some(x) = self.change_lock_time { out.push((Typed::Str("changeLockTime".into()), Typed::Float(x))) }
        if let Some(x) = self.max_rotation_speed { out.push((Typed::Str("maxRotationSpeed".into()), Typed::Float(x))) }
        if let Some(x) = self.initial_rotation_speed { out.push((Typed::Str("initialRotationSpeed".into()), Typed::Float(x))) }
        if let Some(x) = self.rotation_acceleration { out.push((Typed::Str("rotationAcceleration".into()), Typed::Float(x))) }
        if let Some(x) = self.nano_healing_priority_time { out.push((Typed::Str("nanoHealingPriorityTime".into()), Typed::Float(x))) }
        if let Some(x) = self.module_range { out.push((Typed::Str("moduleRange".into()), Typed::Float(x))) }
        if let Some(x) = self.shield_lifetime { out.push((Typed::Str("shieldLifetime".into()), Typed::Float(x))) }
        if let Some(x) = self.teleport_time { out.push((Typed::Str("teleportTime".into()), Typed::Float(x))) }
        if let Some(x) = self.camera_time { out.push((Typed::Str("cameraTime".into()), Typed::Float(x))) }
        if let Some(x) = self.camera_delay { out.push((Typed::Str("cameraDelay".into()), Typed::Float(x))) }
        if let Some(x) = self.to_invisible_speed { out.push((Typed::Str("toInvisibleSpeed".into()), Typed::Float(x))) }
        if let Some(x) = self.to_invisible_duration { out.push((Typed::Str("toInvisibleDuration".into()), Typed::Float(x))) }
        if let Some(x) = self.to_visible_duration { out.push((Typed::Str("toVisibleDuration".into()), Typed::Float(x))) }
        if let Some(x) = self.countdown_time { out.push((Typed::Str("countdownTime".into()), Typed::Float(x))) }
        if let Some(x) = self.stun_time { out.push((Typed::Str("stunTime".into()), Typed::Float(x))) }
        if let Some(x) = self.stun_radius { out.push((Typed::Str("stunRadius".into()), Typed::Float(x))) }
        if let Some(x) = self.effect_duration { out.push((Typed::Str("effectDuration".into()), Typed::Float(x))) }
        Typed::HashMap(out.into())
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ItemCategory {
    NoFunction = 0,
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
            ItemCategory::SprinterLeg => "SprinterLeg",
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

    pub fn but_bigger(&self) -> i32 {
        (*self as i32) * 100_000
    }

    pub fn from_bigger(num: i32) -> Option<Self> {
        Self::from_smaller(num / 100_000)
    }

    pub fn from_smaller(num: i32) -> Option<Self> {
        match num {
            0 => Some(Self::NoFunction),
            1 => Some(Self::Wheel),
            2 => Some(Self::Hover),
            3 => Some(Self::Wing),
            4 => Some(Self::Rudder),
            5 => Some(Self::Thruster),
            6 => Some(Self::InsectLeg),
            7 => Some(Self::MechLeg),
            8 => Some(Self::Ski),
            9 => Some(Self::TankTrack),
            10 => Some(Self::Rotor),
            11 => Some(Self::SprinterLeg),
            12 => Some(Self::Propeller),
            100 => Some(Self::Laser),
            200 => Some(Self::Plasma),
            250 => Some(Self::Mortar),
            300 => Some(Self::Rail),
            400 => Some(Self::Nano),
            500 => Some(Self::Tesla),
            600 => Some(Self::Aeroflak),
            650 => Some(Self::Ion),
            701 => Some(Self::Seeker),
            750 => Some(Self::Chaingun),
            800 => Some(Self::ShieldModule),
            801 => Some(Self::GhostModule),
            802 => Some(Self::BlinkModule),
            803 => Some(Self::EmpModule),
            804 => Some(Self::WindowmakerModule),
            900 => Some(Self::EnergyModule),
            _ => None,
        }
    }
}
