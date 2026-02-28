//! The foreign function interface implementation for validation vehicles in different shared objects/libraries.
//use std::ffi::{CString, c_char, CStr};

const VALIDATE_VEHICLE_SYMBOL_NAME: &[u8] = b"oj_rc_validate_vehicle";
const VALIDATE_VEHICLE_SYMBOL_NAME_STR: &str = "oj_rc_validate_vehicle";

pub struct VehicleValidatorCPlugin {
    dll: libloading::Library,
    pretty_name: String,
}

impl VehicleValidatorCPlugin {
    pub fn new(file: impl AsRef<std::path::Path>) -> Result<Self, libloading::Error> {
        let dll = unsafe { libloading::Library::new(file.as_ref()) }?;
        Ok(Self {
            dll,
            pretty_name: file.as_ref().display().to_string(),
        })
    }
}

impl super::VehicleValidatorPlugin for VehicleValidatorCPlugin {
    fn validate(&self, cube_data: &[u8], colour_data: &[u8]) -> super::ValidationResultCode {
        let func: libloading::Symbol<unsafe extern "C" fn(u32, *const u8, u32, *const u8) -> u8> = match unsafe { self.dll.get(VALIDATE_VEHICLE_SYMBOL_NAME) } {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to find symbol {} in library {}: {}", VALIDATE_VEHICLE_SYMBOL_NAME_STR, self.pretty_name, e);
                return super::ValidationResultCode::Invalid;
            }
        };
        let cubes_len = cube_data.len() as u32;
        let cubes = cube_data.as_ptr();
        let colour_len = colour_data.len() as u32;
        let colours = colour_data.as_ptr();
        let code = unsafe {
            func(cubes_len, cubes, colour_len, colours)
        };
        super::ValidationResultCode::from_u8(code)
    }
}

impl crate::Plugin for VehicleValidatorCPlugin {
    fn self_check(&self) -> bool {
        unsafe {
            self.dll.get::<unsafe extern "C" fn(u32, *const u8, u32, *const u8) -> u8>(VALIDATE_VEHICLE_SYMBOL_NAME)
        }.is_ok()
    }
}
