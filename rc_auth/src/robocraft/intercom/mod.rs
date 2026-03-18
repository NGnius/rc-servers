mod internal_auth;
pub use internal_auth::{IntercomAuth, IntercomOpError};

mod services;
pub use services::{services_ws, service_msg};

mod user_registry;
pub use user_registry::Users;

mod status;
pub use status::{status_set, status_get};

mod lobby;
pub use lobby::{lobby_state_ws, lobby_state_msg};

enum IntercomOp<M: serde::Serialize + serde::Deserialize<'static> + 'static> {
    Message(M),
    Info(IntercomInfo),
}

enum IntercomInfo {
    Close,
}

type WebServicesIntercomOp = IntercomOp<oj_rc_core::persist::user::intercom::IntercomWebServiceUserMessage>;
type LobbyStateIntercomOp = IntercomOp<oj_rc_core::persist::user::intercom::IntercomLobbyStateMessage>;
