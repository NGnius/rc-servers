mod chat_json;
pub use chat_json::ChatUserInfo;

mod traits;
pub use traits::ChatUser;

pub const CHAT_USER_FILE: &str = "chat.json";
pub type ChatUserImpl = ChatUserInfo;
