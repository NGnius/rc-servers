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
        self.command_channel.to_lowercase() == channel.to_lowercase()
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
    perms: ExecutePermission,
}

impl ChatCommand {
    fn compile_command(command: oj_rc_core::persist::ChatCommand) -> Result<Self, regex::Error> {
        Ok(Self {
            regex: regex::RegexBuilder::new(&command.regex).build()?,
            op: ChatOperation::from_persist(command.op),
            perms: ExecutePermission::from_persist(command.permission),
        })
    }

    async fn perform_if_match<'b, 'c>(&self, text: &str, ctx: CommandContext<'b, 'c>) -> Option<String> {
        if let Some(cap) = self.regex.captures(text) {
            if self.perms.has_perms(ctx.user) {
                Some(self.op.perform_command(text, cap, ctx).await)
            } else {
                log::warn!("User {} tried to run command {} without sufficient permissions", ctx.user.public_id(), self.regex);
                None
            }
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

    async fn perform_command<'a, 'b, 'c>(&self, text: &str, _captures: regex::Captures<'a>, ctx: CommandContext<'b, 'c>) -> String {
        match self {
            Self::BuiltIn(b_in) => b_in.do_command(text, ctx).await,
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
    Intercom(Intercom),
    OnlineUsers,
    TotalUsers,
    Stats,
    Version,
    Help,
}

impl BuiltIn {
    fn from_persist(b_in: oj_rc_core::persist::BuiltInChatOperation) -> Self {
        match b_in {
            oj_rc_core::persist::BuiltInChatOperation::Intercom(com) => Self::Intercom(Intercom::from_persist(com)),
            oj_rc_core::persist::BuiltInChatOperation::OnlineUsers => Self::OnlineUsers,
            oj_rc_core::persist::BuiltInChatOperation::TotalUsers => Self::TotalUsers,
            oj_rc_core::persist::BuiltInChatOperation::Stats => Self::Stats,
            oj_rc_core::persist::BuiltInChatOperation::Version => Self::Version,
            oj_rc_core::persist::BuiltInChatOperation::Help => Self::Help,
        }
    }

    fn prettify_re(regex: &str) -> &str {
        regex.trim_start_matches("\\")
    }

    async fn do_command<'b, 'c>(&self, text: &str, ctx: CommandContext<'b, 'c>) -> String {
        match self {
            Self::Intercom(intercom) => intercom.do_command(text, ctx).await,
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
            Self::Stats => {
                let mut stats = Vec::new();
                for variant in text.trim().split(' ').skip(1) {
                    match variant {
                        "db" | "database" => {
                            let db_stats = format!("(chat db) {}", ctx.user.db_metrics().await);
                            stats.push(db_stats);
                        },
                        "perms" | "permission" | "permissions" => {
                            let com_stats = format!("(perms {}) mod:{} admin:{} dev:{} banned:{} royal:{}",
                                                    ctx.user.public_id(),
                                                    ctx.user.is_mod(),
                                                    ctx.user.is_admin(),
                                                    ctx.user.is_dev(),
                                                    ctx.user.is_banned(),
                                                    ctx.user.is_royal(),
                                                    );
                            stats.push(com_stats);
                        },
                        "up" | "uptime" => {
                            let now = chrono::Utc::now().timestamp();
                            let startup_timestamp = crate::START_TIMESTAMP_S.load(std::sync::atomic::Ordering::Relaxed);
                            let uptime_delta = now - startup_timestamp;
                            let uptime_str = if uptime_delta <= 0 {
                                "0?".to_owned()
                            } else if uptime_delta < 60 {
                                format!("{}s", uptime_delta)
                            } else if uptime_delta < 24 * 60 * 60 {
                                format!("{}:{:02}", uptime_delta / (60 * 60), (uptime_delta % (60 * 60)) / 60)
                            } else {
                                format!("{} days {}:{:02}", uptime_delta / (24 * 60 * 60), (uptime_delta % (24 * 60 * 60)) / (60 * 60), ((uptime_delta % (24 * 60 * 60)) % (60 * 60)) / 60)
                            };
                            let ready_ns = crate::READY_DURATION_NS.load(std::sync::atomic::Ordering::Relaxed);
                            let uptime_stats = format!("(uptime) {}, startup in {}ns", uptime_str, ready_ns);
                            stats.push(uptime_stats);
                        },
                        idk => {
                            let stat = format!("(unknown stat) {}", idk);
                            stats.push(stat);
                        }
                    }
                }
                if stats.is_empty() {
                    stats.push("TODO: general stats (try db)".to_owned());
                }
                stats.join("\n")
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
                let force_all = text.trim().split(' ').any(|word| word == "--all");
                let mut msg = String::new();
                for command in ctx.chat_system.chat_config().commands.iter() {
                    if command.perms.has_perms(ctx.user) || force_all {
                        let raw_re = command.regex.to_string();
                        let pretty_name = Self::prettify_re(&raw_re);
                        if let Err(e) = write!(msg, "\n{}: {}", pretty_name, command.op.help_str()) {
                            log::warn!("Failed to construct help for command `{}`: {}", pretty_name, e);
                        }
                    }
                }
                msg
            }
        }
    }

    fn do_help(&self) -> String {
        match self {
            Self::Intercom(i) => i.do_help(),
            Self::OnlineUsers => "Show total users online".to_owned(),
            Self::TotalUsers => "Show total users registered".to_owned(),
            Self::Stats => "Show server metrics (db|perms)".to_owned(),
            Self::Version => "Show chat server version information".to_owned(),
            Self::Help => "Display this message".to_owned(),
        }
    }
}

enum Intercom {
    DevMessage,
    DevBroadcast,
    Maintenance,
}

impl Intercom {
    fn from_persist(intercom: oj_rc_core::persist::IntercomChatOperation) -> Self {
        match intercom {
            oj_rc_core::persist::IntercomChatOperation::DevMessage => Self::DevMessage,
            oj_rc_core::persist::IntercomChatOperation::DevBroadcast => Self::DevBroadcast,
            oj_rc_core::persist::IntercomChatOperation::Maintenance => Self::Maintenance,
        }
    }

    async fn do_command<'b, 'c>(&self, text: &str, ctx: CommandContext<'b, 'c>) -> String {
        match self {
            Self::DevMessage => {
                let pub_id = ctx.user.public_id();
                ctx.user.show_dev_message(
                    oj_rc_core::persist::user::intercom::IntercomDevMessage {
                        message: text.trim().split_once(' ').map(|x| x.1.to_owned()).unwrap_or_else(|| "???".to_owned()),
                        duration: 10,
                    },
                    vec![pub_id.to_owned()],
                ).await;
                format!("Sent dev message to {}", pub_id)
            },
            Self::DevBroadcast => {
                if let Some(message) = text.trim().split_once(' ').map(|x| x.1.to_owned()) {
                    ctx.user.show_dev_message(
                        oj_rc_core::persist::user::intercom::IntercomDevMessage {
                            message,
                            duration: 60,
                        },
                        vec![],
                    ).await;
                    "Sent dev broadcast to everyone".to_owned()
                } else {
                    "Missing dev message, did not send".to_owned()
                }
            },
            Self::Maintenance => {
                if let Some(message) = text.trim().split_once(' ').map(|x| x.1.to_owned()) {
                    ctx.user.enter_maintenance(
                        oj_rc_core::persist::user::intercom::IntercomMaintenanceMessage { message },
                        vec![],
                    ).await;
                    "Sent maintenance message".to_owned()
                } else {
                    "Missing maintenance message, did not send".to_owned()
                }

            }
        }

    }

    fn do_help(&self) -> String {
        match self {
            Self::DevMessage => "Show dev message to yourself".to_owned(),
            Self::DevBroadcast => "Show dev message to everyone".to_owned(),
            Self::Maintenance => "Broadcast maintenance mode to everyone".to_owned(),
        }
    }
}

enum ExecutePermission {
    Player,
    Moderator,
    Administrator,
    Developer,
    Royal,
    None,
}

impl ExecutePermission {
    fn from_persist(perm: oj_rc_core::persist::ChatPermission) -> Self {
        match perm {
            oj_rc_core::persist::ChatPermission::Player => Self::Player,
            oj_rc_core::persist::ChatPermission::Moderator => Self::Moderator,
            oj_rc_core::persist::ChatPermission::Administrator => Self::Administrator,
            oj_rc_core::persist::ChatPermission::Developer => Self::Developer,
            oj_rc_core::persist::ChatPermission::Royal => Self::Royal,
            oj_rc_core::persist::ChatPermission::None => Self::None,
        }
    }

    fn has_perms(&self, user: &dyn oj_rc_core::persist::user::CommonUser) -> bool {
        match self {
            Self::Player => true,
            Self::Moderator => user.is_mod() || user.is_admin() || user.is_dev() || user.is_royal(),
            Self::Administrator => user.is_admin() || user.is_dev() || user.is_royal(),
            Self::Developer => user.is_dev() || user.is_royal(),
            Self::Royal => user.is_royal(),
            Self::None => false,
        }
    }
}
