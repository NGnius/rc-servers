use rc_core::UserAuthenticator;
use rocket::{post, routes, serde::json::Json, http::Status, State};

#[post("/authenticate/robocraft/game", data = "<body>")]
pub async fn email_password_auth(body: Json<libfj::robocraft::EmailUserAuthenticationPayload>, config: &State<crate::common::cli::Config>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, Status> {
    log::info!("Authenticating {} user {}", body.target, body.display_name);
    let payload = libfj::robocraft::TokenPayload {
        public_id: body.display_name.clone(),
        display_name: body.display_name.clone(),
        robocraft_name: body.display_name.clone(),
        email_address: body.email_address.clone(),
        email_verified: true,
        flags: Vec::new(),
    };
    let user_info = rc_core::persist::user::UserInfo {
        payload,
        extra: rc_core::persist::user::ExtraUserInfo::Standalone { password: body.password.clone() },
    };
    let response = config.robocraft.account_provider.login(user_info).await
        .map_err(|e| {
            log::error!("Failed to authenticate {} user {}: {}", body.target, body.display_name, e);
            Status { code: 401 }
        })?;
    Ok(Json(response.response))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Robocraft Email/Password", |rocket| async {
        rocket.mount("/", routes![email_password_auth])
    })
}
