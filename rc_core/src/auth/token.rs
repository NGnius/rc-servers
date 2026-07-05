use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    #[serde(flatten)]
    pub client_details: libfj::robocraft::TokenPayload,
    pub federate: bool,
    pub auth_time: i64,
    pub qualified_name: String,
    pub source_domain: String,
    pub login_method: LoginMethod,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
    pub aud: String,
    pub fedi_token: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum LoginMethod {
    Steam,
    DisplayName,
    Username,
    Email,
    OAuth,
}
