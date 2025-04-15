use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatConfig {
    #[serde(default = "default_pub_channs")]
    pub public_channels: Vec<String>,
}

fn default_pub_channs() -> Vec<String> {
    vec![
        "main".to_owned(),
        "sys".to_owned(),
        "openjam_worship".to_owned(),
    ]
}
