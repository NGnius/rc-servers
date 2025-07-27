use oj_rc_core::UserAuthenticator;
use actix_web::{post, web::{Data, Json}};

#[post("/authenticate/email/game")]
pub async fn email_password_auth(body: Json<libfj::robocraft::EmailUserAuthenticationPayload>, config: Data<super::RcConfig>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, super::ErrorTy> {
    log::info!("Authenticating {} user {}", body.target, body.display_name);
    let payload = libfj::robocraft::TokenPayload {
        public_id: body.display_name.clone(),
        display_name: body.display_name.clone(),
        robocraft_name: body.display_name.clone(),
        email_address: body.email_address.clone(),
        email_verified: true,
        flags: Vec::new(),
    };
    let user_info = oj_rc_core::persist::user::UserInfo {
        payload,
        extra: oj_rc_core::persist::user::ExtraUserInfo::Email { password: body.password.clone() },
    };
    let response = config.account_provider.login(user_info).await
        .map_err(|e| {
            log::error!("Failed to authenticate {} user {}: {}", body.target, body.display_name, e.message);
            super::ErrorTy::from_err(e)
        })?;
    Ok(Json(response.response))
}
