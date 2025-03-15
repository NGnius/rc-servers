use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnlockedParts {
    pub unlocked: Vec<u32>,
    #[serde(rename = "override", default)]
    pub override_: UnlockOverride,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum UnlockOverride {
    #[default]
    Normal,
    UnlockAll,
    UnlockNone,
}
