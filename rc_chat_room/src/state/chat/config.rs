pub struct ChatSystemConfig {
    command_channel: String,
    commands: Vec<ChatCommand>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct CommandContext<'a, 'b> {
    chat_system: &'a super::ChatSystem,
    user: &'b dyn oj_rc_core::persist::user::ChatUser,
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

    pub async fn perform_command(&self, text: &str, chat_system: &super::ChatSystem, user: &dyn oj_rc_core::persist::user::ChatUser,) -> String {
        let ctx = CommandContext {
            chat_system,
            user,
        };
        for cmd in self.commands.iter() {
            if let Some(result) = cmd.perform_if_match(text, ctx).await {
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

    async fn perform_if_match<'b, 'c>(&self, text: &str, ctx: CommandContext<'b, 'c>) -> Option<String> {
        if let Some(cap) = self.regex.captures(text) {
            Some(self.op.perform_command(cap, ctx).await)
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
    fn from_persist(op: oj_rc_core::persist::ChatOperation) -> Self {
        match op {
            oj_rc_core::persist::ChatOperation::BuiltIn(b_in) => Self::BuiltIn(BuiltIn::from_persist(b_in)),
            oj_rc_core::persist::ChatOperation::Custom => Self::Custom,
            oj_rc_core::persist::ChatOperation::Nop => Self::Nop,
        }
    }

    async fn perform_command<'a, 'b, 'c>(&self, _captures: regex::Captures<'a>, ctx: CommandContext<'b, 'c>) -> String {
        match self {
            Self::BuiltIn(b_in) => b_in.do_command(ctx).await,
            Self::Custom => "{not implemented}".to_owned(),
            Self::Nop => "{no op}".to_owned(),
        }
    }

    fn help_str(&self) -> String {
        match self {
            Self::BuiltIn(b_in) => b_in.do_help(),
            Self::Custom => "{not implemented}".to_owned(),
            Self::Nop => "does nothing".to_owned(),
        }
    }
}

enum BuiltIn {
    OnlineUsers,
    TotalUsers,
    Version,
    Help,
}

impl BuiltIn {
    fn from_persist(b_in: oj_rc_core::persist::BuiltInChatOperation) -> Self {
        match b_in {
            oj_rc_core::persist::BuiltInChatOperation::OnlineUsers => Self::OnlineUsers,
            oj_rc_core::persist::BuiltInChatOperation::TotalUsers => Self::TotalUsers,
            oj_rc_core::persist::BuiltInChatOperation::Version => Self::Version,
            oj_rc_core::persist::BuiltInChatOperation::Help => Self::Help,
        }
    }

    fn prettify_re(regex: &str) -> &str {
        regex.trim_start_matches("\\")
    }

    async fn do_command<'b, 'c>(&self, ctx: CommandContext<'b, 'c>) -> String {
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
                match ctx.user.get_total_registered_users().await {
                    Ok(count) => if count == 1 {
                        "1 user registered".to_owned()
                    } else {
                        format!("{} users registered", count)
                    },
                    Err(e) => e.error_msg().map(|x| x.to_owned()).unwrap_or_else(|| "Failed to retrieve registered users".to_owned()),
                }
            },
            Self::Version => {
                let name = env!("CARGO_PKG_NAME");
                let version = env!("CARGO_PKG_VERSION");
                let git_version = git_version::git_version!(args = ["--always", "--dirty=+"]);
                let authors = env!("CARGO_PKG_AUTHORS");
                let license = env!("CARGO_PKG_LICENSE");
                let repo = env!("CARGO_PKG_REPOSITORY");
                format!("{} {}:{}\n[{}]\n{} {}", name, version, git_version, authors, license, repo)
            }
            Self::Help => {
                use core::fmt::Write;
                let mut msg = String::new();
                for command in ctx.chat_system.chat_config().commands.iter() {
                    let raw_re = command.regex.to_string();
                    let pretty_name = Self::prettify_re(&raw_re);
                    if let Err(e) = write!(msg, "\n{}: {}", pretty_name, command.op.help_str()) {
                        log::warn!("Failed to construct help for command `{}`: {}", pretty_name, e);
                    }
                }
                msg
            }
        }
    }

    fn do_help(&self) -> String {
        match self {
            Self::OnlineUsers => "Show total users online".to_owned(),
            Self::TotalUsers => "Show total users registered".to_owned(),
            Self::Version => "Show chat server version information".to_owned(),
            Self::Help => "Display this message".to_owned(),
        }
    }
}
