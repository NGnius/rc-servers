#[repr(u8)]
pub enum ValidationResultCode {
    Invalid = 0,
    Ok = 1,
    NoWeapon = 2,
    NoMovement = 3,
    Sanctioned = 4,
}

impl ValidationResultCode {
    pub(super) fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::Invalid,
            1 => Self::Ok,
            2 => Self::NoWeapon,
            3 => Self::NoMovement,
            4 => Self::Sanctioned,
            _ => Self::Invalid,
        }
    }
}

pub trait VehicleValidatorPlugin: crate::Plugin {
    fn validate(&self, cube_data: &[u8], colour_data: &[u8]) -> ValidationResultCode;
}
