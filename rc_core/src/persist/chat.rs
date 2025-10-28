use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatConfig {
    #[serde(default = "default_pub_channs")]
    pub public_channels: Vec<String>,
    #[serde(default = "default_command_chann")]
    pub command_channel: String,
    pub commands: Vec<ChatCommand>,
    #[serde(default = "default_selected_chann")]
    pub default_channel: String,
    #[serde(default = "default_true")]
    pub can_create_channels: bool,
}

impl super::config::SelfValidator for ChatConfig {
    type Context = crate::ConfigImpl;
    fn validate(&self, _info: &mut super::config::ValidationInfo, _ctx: &Self::Context) -> bool {
        // TODO
        true
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
    Intercom(IntercomChatOperation),
    OnlineUsers,
    TotalUsers,
    Version,
    Help,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "intercom")]
pub enum IntercomChatOperation {
    DevMessage,
}


fn default_pub_channs() -> Vec<String> {
    vec![
        "main".to_owned(),
        "sys".to_owned(),
        "jam_club".to_owned(),
    ]
}


fn default_command_chann() -> String {
    "sys".to_owned()
}

fn default_selected_chann() -> String {
    "jam_club".to_owned()
}

fn default_true() -> bool {
    true
}
