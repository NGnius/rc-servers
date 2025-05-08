use rc_core::UserAuthenticator;
use rocket::{http::Status, post, routes, serde::json::Json, State};

#[post("/authenticate/steam/game", data = "<body>")]
pub async fn steam_auth(body: Json<libfj::robocraft::SteamAuthenticationPayload>, config: &State<crate::common::cli::Config>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, Status> {
    let steam_id = crate::common::steam_utils::authenticate_steam_ticket(&body.steam_ticket)
        .map_err(|_| Status { code: 401 })?;
    log::info!("Authenticating {} steam user {}", body.target, steam_id);
    let payload = libfj::robocraft::TokenPayload {
        public_id: steam_id.to_string(),
        display_name: steam_id.to_string(),
        robocraft_name: steam_id.to_string(),
        email_address: format!("{}.rc.steam@ngram.ca", steam_id),
        email_verified: true,
        flags: Vec::new(),
    };
    let user_info = rc_core::persist::user::UserInfo {
        payload,
        extra: rc_core::persist::user::ExtraUserInfo::Steam { id: steam_id },
    };
    let response = config.robocraft.account_provider.login(user_info).await
        .map_err(|e| {
            log::error!("Failed to authenticate {} steam user {}: {}", body.target, steam_id, e);
            Status { code: 401 }
        })?;
    Ok(Json(response.response))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Robocraft Steam", |rocket| async {
        rocket.mount("/", routes![steam_auth])
    })
}
