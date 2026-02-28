// possible codes
#[allow(dead_code)]
#[repr(u8)]
pub enum ValidateMachineResult {
    Invalid = 0,
    Ok = 1,
    NoWeapon = 2,
    NoMovement = 3,
    Sanctioned = 4,
}

impl ValidateMachineResult {
    pub fn from_plugin(code: oj_rc_plugins::vehicle_validation::ValidationResultCode) -> Self {
        match code {
            oj_rc_plugins::vehicle_validation::ValidationResultCode::Invalid => Self::Invalid,
            oj_rc_plugins::vehicle_validation::ValidationResultCode::Ok => Self::Ok,
            oj_rc_plugins::vehicle_validation::ValidationResultCode::NoWeapon => Self::NoWeapon,
            oj_rc_plugins::vehicle_validation::ValidationResultCode::NoMovement => Self::NoMovement,
            oj_rc_plugins::vehicle_validation::ValidationResultCode::Sanctioned => Self::Sanctioned,
        }
    }
}
