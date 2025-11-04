mod internal_auth;
pub use internal_auth::{IntercomAuth, IntercomOpError};

mod services;
pub use services::{services_ws, service_msg};

mod user_registry;
pub use user_registry::Users;

mod status;
pub use status::{status_set, status_get};

enum IntercomOp {
    Message(oj_rc_core::persist::user::intercom::IntercomWebServiceUserMessage),
    Info(IntercomInfo),
}

enum IntercomInfo {
    Close,
}
