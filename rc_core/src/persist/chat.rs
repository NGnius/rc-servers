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
    fn validate(&self, info: &mut super::config::ValidationInfo, _ctx: &Self::Context) -> bool {
        let mut is_ok = true;
        if !self.public_channels.contains(&self.command_channel) {
            info.warn(super::config::ValidationMessage {
                path: vec!["public_channels".to_owned()],
                message: "Chat command channel is not a public channel".to_owned(),
            });
        }
        if !self.public_channels.contains(&self.default_channel) {
            info.warn(super::config::ValidationMessage {
                path: vec!["public_channels".to_owned()],
                message: "Chat default channel is not a public channel".to_owned(),
            });
        }
        if self.command_channel.is_empty() {
            info.error(super::config::ValidationMessage {
                path: vec!["command_channel".to_owned()],
                message: "Chat command channel should not be empty".to_owned(),
            });
            is_ok = false;
        }
        for (i, cmd) in self.commands.iter().enumerate() {
            is_ok &= cmd.validate_in(info, self, &format!("commands[{}]", i));
        }
        if self.default_channel.is_empty() {
            info.error(super::config::ValidationMessage {
                path: vec!["default_channel".to_owned()],
                message: "Chat default channel should not be empty".to_owned(),
            });
            is_ok = false;
        }
        is_ok
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatCommand {
    pub regex: String,
    pub op: ChatOperation,
    pub permission: ChatPermission,
    #[serde(default)]
    pub hidden: bool,
}

impl super::config::SelfValidator for ChatCommand {
    type Context = ChatConfig;
    fn validate(&self, info: &mut super::config::ValidationInfo, ctx: &Self::Context) -> bool {
        let mut is_ok = true;
        let regex_count = ctx.commands.iter().filter(|other| self.regex == other.regex).count();
        if regex_count != 1 {
            info.error(super::config::ValidationMessage {
                path: vec!["regex".to_owned()],
                message: format!("Only one chat command can use an identical regex pattern {}; found {}", self.regex, regex_count),
            });
            is_ok = false;
        }
        // TODO validate regex

        // recommended commands to only allow with elevated permissions
        if matches!(
            self.op,
            ChatOperation::BuiltIn(BuiltInChatOperation::Intercom(IntercomChatOperation::DevBroadcast))
            | ChatOperation::BuiltIn(BuiltInChatOperation::Intercom(IntercomChatOperation::Maintenance))
            | ChatOperation::BuiltIn(BuiltInChatOperation::System(SystemChatOperation::Permissions))
        ) {
            if !matches!(self.permission, ChatPermission::Administrator | ChatPermission::Developer | ChatPermission::Royal) {
                info.warn(crate::persist::config::ValidationMessage {
                    path: vec!["permission".to_owned()],
                    message: format!("Chat command {:?} is recommended to require Administrator, Developer, or Royal permissions; found {:?}", self.op, self.permission),
                });
            }
        }
        is_ok
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ChatPermission {
    Player,
    #[serde(alias = "Mod")]
    Moderator,
    #[serde(alias = "Admin")]
    Administrator,
    #[serde(alias = "Dev")]
    Developer,
    #[serde(alias = "Special")]
    Royal,
    None,
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
    System(SystemChatOperation),
    OnlineUsers,
    TotalUsers,
    Stats,
    Version,
    Help,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "intercom")]
pub enum IntercomChatOperation {
    DevMessage,
    DevBroadcast,
    Maintenance,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "system")]
pub enum SystemChatOperation {
    Permissions,
    CheckPermissions,
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
