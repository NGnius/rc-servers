use oj_rc_core::persist::user::FederatedAuthenticator;
use actix_web::{post, web::{Data, Json}};

#[post("/authenticate/displayname/game")]
pub async fn displaye_password_auth(body: Json<libfj::robocraft::EmailUserAuthenticationPayload>, config: Data<super::RcConfig>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, super::ErrorTy> {
    if body.display_name.is_none() {
        return Err(super::ErrorTy::from_err(oj_rc_core::persist::user::AuthError {
            message: "Missing display_name".to_owned(),
            code: oj_rc_core::data::error_codes::AuthErrorCode::BadCredentials,
        }));
    }
    let display_name = body.display_name.clone().unwrap();
    if let Some((display_name, domain)) = display_name.split_once('#') {
        log::info!("Authenticating {} user {} for domain {}", body.target, display_name, domain);
        let user_info = oj_rc_core::persist::user::FederatedAuthInfo {
            display_name: display_name.to_owned(),
            password: body.password.clone(),
            domain: domain.to_owned(),
        };
        let response = config.account_provider.local_login(user_info).await
            .map_err(|e| {
                log::error!("Failed to authenticate {} user {}#{}: {}", body.target, display_name, domain, e.message);
                super::ErrorTy::from_err(e)
            })?;
        Ok(Json(response.response))
    } else {
        Err(super::ErrorTy::from_err(oj_rc_core::persist::user::AuthError {
            message: "Missing display_name domain".to_owned(),
            code: oj_rc_core::data::error_codes::AuthErrorCode::InvalidDisplayName,
        }))
    }
}
