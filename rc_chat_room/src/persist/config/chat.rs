use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatSystemConfig {
    #[serde(default = "default_command_chann")]
    pub command_channel: String,
    pub commands: Vec<ChatCommand>,
}

fn default_command_chann() -> String {
    "sys".to_owned()
}

impl ChatSystemConfig {
    pub fn load(asset_root: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let file = std::fs::File::open(asset_root.as_ref().join(super::CHAT_CONFIG_FILE))?;
        let buffered = std::io::BufReader::new(file);
        let config = serde_json::from_reader(buffered)?;
        Ok(config)
    }
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
