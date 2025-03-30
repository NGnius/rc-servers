use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SelectedGarage {
    pub uuid: (u32, u32),
    pub slot: u32,
}

impl SelectedGarage {
    pub fn uuid_str(&self) -> String {
        format!("{}_{}", self.uuid.0, self.uuid.1)
    }
}
