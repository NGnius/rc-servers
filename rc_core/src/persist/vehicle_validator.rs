use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "validator")]
pub enum VehicleValidator {
    None,
    Cpu {
        min: u32,
        max: u32,
    },
    // TODO other validators
    All {
        all: Vec<Self>,
    },
    Any {
        any: Vec<Self>,
    },
    Custom {
        #[serde(alias = "library")]
        path: String,
    },
}
