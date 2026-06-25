use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Federation {
    pub enabled: bool,
    pub defederated: Vec<String>,
}

impl std::default::Default for Federation {
    fn default() -> Self {
        Self {
            enabled: true,
            defederated: Vec::default(),
        }
    }
}
