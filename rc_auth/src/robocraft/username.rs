use oj_rc_core::UserAuthenticator;
use actix_web::{post, web::{Data, Json}};

#[post("/authenticate/robocraft/game")]
pub async fn user_password_auth(body: Json<libfj::robocraft::EmailUserAuthenticationPayload>, config: Data<super::RcConfig>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, super::ErrorTy> {
    if body.display_name.is_none() {
        return Err(super::ErrorTy::from_err(oj_rc_core::persist::user::AuthError {
            message: "Missing display_name".to_owned(),
            code: oj_rc_core::data::error_codes::AuthErrorCode::BadCredentials,
        }));
    }
    let display_name = body.display_name.clone().unwrap();
    log::info!("Authenticating {} user {}", body.target, display_name);
    let user_info = oj_rc_core::persist::user::UserAuthInfo::Username {
        username: display_name.clone(),
        password: body.password.clone(),
    };
    let response = config.account_provider.login(user_info).await
        .map_err(|e| {
            log::error!("Failed to authenticate {} user {}: {}", body.target, display_name, e.message);
            super::ErrorTy::from_err(e)
        })?;
    Ok(Json(response.response))
}
