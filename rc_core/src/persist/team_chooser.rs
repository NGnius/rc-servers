use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "choice")]
pub enum TeamChooser {
    Alternating,
    AllOnOne {
        team: u8,
    },
    #[serde(alias = "Pit")]
    OneOnAll,
    // TODO more built-in choosers
    Custom {
        #[serde(alias = "library")]
        path: String,
    },
}
