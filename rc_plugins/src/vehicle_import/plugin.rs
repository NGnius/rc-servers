#[repr(u8)]
#[derive(Debug)]
pub enum VehicleImportErrorCode {
    Invalid,
    Unsupported,
    UnknownCube,
}

impl core::fmt::Display for VehicleImportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid/corrupt data"),
            Self::Unsupported => write!(f, "Unsupported/unrecognized data"),
            Self::UnknownCube => write!(f, "Unknown cube in data"),
        }
    }
}

impl core::error::Error for VehicleImportErrorCode {}

#[derive(Debug)]
pub struct VehicleImportData {
    pub cube_data: Vec<u8>,
    pub colour_data: Vec<u8>,
    pub vehicle_name: Option<String>,
    pub vehicle_author: Option<String>,
}

pub trait VehicleImportPlugin: crate::Plugin {
    //fn supports(&self) -> Vec<String>;
    /// Preferred file extension for the raw data
    fn file_ext(&self) -> &'static str;
    fn import(&self, upload: &[u8]) -> Result<VehicleImportData, VehicleImportErrorCode>;
    fn export(&self, data: &VehicleImportData) -> Result<Vec<u8>, VehicleImportErrorCode>;
}
