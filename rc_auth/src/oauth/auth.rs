use actix_web::{get, post, web::{Data, Form, Query, Redirect}};
use serde::{Serialize, Deserialize};

use oj_rc_core::persist::user::FederatedAuthenticator;

#[derive(Serialize, Deserialize, Clone)]
struct AuthQuery {
    pub response_type: Option<String>,
    pub client_id: String,
    pub redirect_uri: Option<String>,
    pub scope: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}

#[post("/authenticate/oauth2/auth")]
pub async fn post_oauth_auth(body: Form<oj_rc_core::persist::user::federation::FederatedAuthenticationPayload>, query: Query<AuthQuery>, config: Data<crate::robocraft::RcConfig>) -> impl actix_web::Responder {
    let access_token = match config.account_provider.remote_auth(&body, &query.code_challenge).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to OAuth authenticate {} from {}: {}", body.display_name, body.domain_source, e.message);
            return Redirect::to("/")
                .temporary()
        }
    };
    let redirect_root = query.redirect_uri.as_ref().map(|x| x.to_owned()).unwrap_or_else(|| "/authenticate/oauth2/auth".to_owned());
    let redirect_url = format!("{}?code={}&state={}", redirect_root, access_token, query.state);
    #[cfg(debug_assertions)]
    log::debug!("Redirecting to {}", redirect_url);
    Redirect::to(redirect_url)
        .temporary()
}

#[get("/authenticate/oauth2/auth")]
pub async fn get_oauth_auth() -> &'static str {
    "This is unimplemented and should not be used for standard OAuth flows"
}
