pub struct ChatSystemConfig {
    command_channel: String,
    commands: Vec<ChatCommand>,
    asset_root: std::path::PathBuf,
    data_root: std::path::PathBuf,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct CommandContext<'a, 'b, 'c> {
    chat_system: &'a super::ChatSystem,
    user: &'b dyn rc_core::persist::user::User<()>,
    asset_root: &'c std::path::PathBuf,
    data_root: &'c std::path::PathBuf,
}

impl ChatSystemConfig {
    pub fn from_persist(config: crate::persist::config::ChatSystemConfig, asset_root: std::path::PathBuf, data_root: std::path::PathBuf) -> std::io::Result<Self> {
        let mut compiled_commands = Vec::with_capacity(config.commands.len());
        for (i, cmd) in config.commands.into_iter().enumerate() {
            let compiled_command = ChatCommand::compile_command(cmd).map_err(|e| {
                log::error!("Failed to load command {}: {}", i, e);
                std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
            })?;
            compiled_commands.push(compiled_command);
        }
        Ok(Self {
            command_channel: config.command_channel,
            commands: compiled_commands,
            asset_root,
            data_root,
        })
    }

    pub fn perform_command(&self, text: &str, chat_system: &super::ChatSystem, user: &dyn rc_core::persist::user::User<()>,) -> String {
        let ctx = CommandContext {
            chat_system,
            user,
            asset_root: &self.asset_root,
            data_root: &self.data_root,
        };
        for cmd in self.commands.iter() {
            if let Some(result) = cmd.perform_if_match(text, ctx) {
                return result;
            }
        }
        return "Invalid command".to_owned()
    }

    pub fn is_command_channel(&self, channel: &str) -> bool {
        self.command_channel == channel
    }

    pub fn is_command_user(&self, username: &str) -> bool {
        self.command_channel == username
    }

    pub fn command_username(&self) -> &'_ str {
        &self.command_channel
    }
}

pub struct ChatCommand {
    regex: regex::Regex,
    op: ChatOperation,
}

impl ChatCommand {
    fn compile_command(command: crate::persist::config::ChatCommand) -> Result<Self, regex::Error> {
        Ok(Self {
            regex: regex::RegexBuilder::new(&command.regex).build()?,
            op: ChatOperation::from_persist(command.op)
        })
    }

    fn perform_if_match(&self, text: &str, ctx: CommandContext) -> Option<String> {
        if let Some(cap) = self.regex.captures(text) {
            Some(self.op.perform_command(cap, ctx))
        } else {
            None
        }
    }
}

enum ChatOperation {
    BuiltIn(BuiltIn),
    Custom,
    Nop,
}

impl ChatOperation {
    fn from_persist(op: crate::persist::config::ChatOperation) -> Self {
        match op {
            crate::persist::config::ChatOperation::BuiltIn(b_in) => Self::BuiltIn(BuiltIn::from_persist(b_in)),
            crate::persist::config::ChatOperation::Custom => Self::Custom,
            crate::persist::config::ChatOperation::Nop => Self::Nop,
        }
    }

    fn perform_command<'a>(&self, _captures: regex::Captures<'a>, ctx: CommandContext) -> String {
        match self {
            Self::BuiltIn(b_in) => b_in.do_command(ctx),
            Self::Custom => "{not implemented}".to_owned(),
            Self::Nop => "{no op}".to_owned(),
        }
    }
}

enum BuiltIn {
    OnlineUsers,
    TotalUsers,
}

impl BuiltIn {
    fn from_persist(b_in: crate::persist::config::BuiltInChatOperation) -> Self {
        match b_in {
            crate::persist::config::BuiltInChatOperation::OnlineUsers => Self::OnlineUsers,
            crate::persist::config::BuiltInChatOperation::TotalUsers => Self::TotalUsers,
        }
    }

    fn do_command(&self, ctx: CommandContext) -> String {
        match self {
            Self::OnlineUsers => {
                let online_count = ctx.chat_system.user_count();
                if online_count == 1 {
                    "1 user online".to_owned()
                } else {
                    format!("{} users online", online_count)
                }
            },
            Self::TotalUsers => {
                let user_path = ctx.data_root.join(rc_core::persist::user::USERS_DIR);
                let user_count = user_path.read_dir().map_or(0, |dir| dir.count()).clamp(1, usize::MAX) - 1;
                if user_count == 1 {
                    "1 user exists".to_owned()
                } else {
                    format!("{} users exist", user_count)
                }

            },
        }
    }
}
