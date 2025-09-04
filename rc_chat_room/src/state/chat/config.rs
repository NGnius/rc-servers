pub struct ChatSystemConfig {
    command_channel: String,
    commands: Vec<ChatCommand>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct CommandContext<'a, 'b> {
    chat_system: &'a super::ChatSystem,
    user: &'b dyn oj_rc_core::persist::user::User<()>,
}

impl ChatSystemConfig {
    pub fn from_persist(config: oj_rc_core::persist::config::ChatSystemConfig) -> std::io::Result<Self> {
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
        })
    }

    pub fn perform_command(&self, text: &str, chat_system: &super::ChatSystem, user: &dyn oj_rc_core::persist::user::User<()>,) -> String {
        let ctx = CommandContext {
            chat_system,
            user,
        };
        for cmd in self.commands.iter() {
            if let Some(result) = cmd.perform_if_match(text, ctx) {
                return result;
            }
        }
        "Invalid command".to_owned()
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
    fn compile_command(command: oj_rc_core::persist::ChatCommand) -> Result<Self, regex::Error> {
        Ok(Self {
            regex: regex::RegexBuilder::new(&command.regex).build()?,
            op: ChatOperation::from_persist(command.op)
        })
    }

    fn perform_if_match(&self, text: &str, ctx: CommandContext) -> Option<String> {
        self.regex.captures(text).map(|cap| self.op.perform_command(cap, ctx))
    }
}

enum ChatOperation {
    BuiltIn(BuiltIn),
    Custom,
    Nop,
}

impl ChatOperation {
    fn from_persist(op: oj_rc_core::persist::ChatOperation) -> Self {
        match op {
            oj_rc_core::persist::ChatOperation::BuiltIn(b_in) => Self::BuiltIn(BuiltIn::from_persist(b_in)),
            oj_rc_core::persist::ChatOperation::Custom => Self::Custom,
            oj_rc_core::persist::ChatOperation::Nop => Self::Nop,
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
    fn from_persist(b_in: oj_rc_core::persist::BuiltInChatOperation) -> Self {
        match b_in {
            oj_rc_core::persist::BuiltInChatOperation::OnlineUsers => Self::OnlineUsers,
            oj_rc_core::persist::BuiltInChatOperation::TotalUsers => Self::TotalUsers,
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
                "User count is not supported".to_string()
            },
        }
    }
}
