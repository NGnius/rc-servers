use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    #[serde(flatten)]
    pub client_details: libfj::robocraft::TokenPayload,
    pub federate: bool,
    pub auth_time: i64,
    pub qualified_name: String,
    pub login_method: LoginMethod,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum LoginMethod {
    Steam,
    DisplayName,
    Username,
    Email,
}
