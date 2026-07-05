use std::str::FromStr;

use actix_web::{post, web::{Data, Form, Json}};
use serde::{Serialize, Deserialize};

use oj_rc_core::persist::user::{FederatedAuthenticator, federation::TokenResponsePayload};

#[derive(Serialize, Deserialize, Clone)]
struct TokenQuery {
    pub code: String,
    pub client_id: String,
    pub code_verifier: String,
}

#[post("/authenticate/oauth2/token")]
pub async fn post_oauth_token(body: Form<TokenQuery>, config: Data<crate::robocraft::RcConfig>) -> Json<TokenResponsePayload> {
    // TODO make errors compliant with OAuth2 spec
    match config.account_provider.remote_token(&body.code, &body.code_verifier).await {
        Ok(login_info) => {
            let mut token_resp = TokenResponsePayload::new(
                openidconnect::AccessToken::new(login_info.response.token.clone()),
                openidconnect::core::CoreTokenType::Bearer,
                openidconnect::core::CoreIdTokenFields::new(
                    Some(openidconnect::IdToken::from_str(&login_info.response.token).expect("Bad token")),
                    openidconnect::EmptyExtraTokenFields {},
                ),
            );
            token_resp.set_refresh_token(Some(openidconnect::RefreshToken::new(login_info.response.refresh_token)));
            Json(token_resp)
        },
        Err(e) => {
            log::error!("Failed OAuth2 token auth: {}", e.message);
            Json(TokenResponsePayload::new(
                openidconnect::AccessToken::new(String::default()),
                openidconnect::core::CoreTokenType::Bearer,
                openidconnect::core::CoreIdTokenFields::new(None, openidconnect::EmptyExtraTokenFields {}),
            ))
        }
    }
}
