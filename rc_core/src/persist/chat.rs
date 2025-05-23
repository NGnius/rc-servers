use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatConfig {
    #[serde(default = "default_pub_channs")]
    pub public_channels: Vec<String>,
    #[serde(default = "default_command_chann")]
    pub command_channel: String,
    pub commands: Vec<ChatCommand>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatCommand {
    pub regex: String,
    pub op: ChatOperation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ChatOperation {
    BuiltIn(BuiltInChatOperation),
    Custom,
    #[serde(alias = "No-op")]
    Nop,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "built_in")]
pub enum BuiltInChatOperation {
    OnlineUsers,
    TotalUsers,
}


fn default_pub_channs() -> Vec<String> {
    vec![
        "main".to_owned(),
        "sys".to_owned(),
        "openjam_worship".to_owned(),
    ]
}


fn default_command_chann() -> String {
    "sys".to_owned()
}
