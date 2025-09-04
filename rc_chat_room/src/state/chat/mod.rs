#[allow(clippy::module_inception)]
mod chat;
pub use chat::{ChatSystem, ChatProvider};

mod room;
pub use room::ChatRoom;

mod user;
pub use user::UserHandle;

mod config;
pub use config::{ChatSystemConfig};

pub type ChatImpl = ChatProvider;
