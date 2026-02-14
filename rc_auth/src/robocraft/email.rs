use oj_rc_core::UserAuthenticator;
use actix_web::{post, web::{Data, Json}};

#[post("/authenticate/email/game")]
pub async fn email_password_auth(body: Json<libfj::robocraft::EmailUserAuthenticationPayload>, config: Data<super::RcConfig>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, super::ErrorTy> {
    if body.email_address.is_empty() {
        return Err(super::ErrorTy::from_err(oj_rc_core::persist::user::AuthError {
            message: "Missing email_address".to_owned(),
            code: oj_rc_core::data::error_codes::AuthErrorCode::BadCredentials,
        }));
    }
    log::info!("Authenticating {} email user {}", body.target, body.email_address);
    let user_info = oj_rc_core::persist::user::UserAuthInfo::Email {
        email: body.email_address.clone(),
        password: body.password.clone()
    };
    let response = config.account_provider.login(user_info).await
        .map_err(|e| {
            log::error!("Failed to authenticate {} email user {}: {}", body.target, body.email_address, e.message);
            super::ErrorTy::from_err(e)
        })?;
    Ok(Json(response.response))
}
