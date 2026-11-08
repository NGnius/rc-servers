use actix_web::{Responder, get, post, web::{Data, Form, Path}};
use oj_rc_core::{UserProvider, types::RawType};
use serde::{Serialize, Deserialize};

const FORM_NAME: &str = "user_password_reset";

#[derive(Deserialize)]
#[serde(tag = "action")]
struct FormData {
    password: String,
    password_c: String,
}

#[derive(Serialize, Deserialize)]
struct RenderData {
    display_name: String,
    public_id: String,
    password: String,
    message: Option<String>,
}

#[get("/user/{id}/password_reset/{secret}")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, path: Path<(i32, String)>) -> Result<impl Responder, actix_web::error::Error> {
    let account_id = oj_rc_core::types::AccountId::UserId(oj_rc_core::types::UserId::from_raw(path.0));
    match auth.web_password_secret_authenticate(account_id, path.1.clone()).await {
        Ok(user) => {
            Ok(crate::web::render_ok(
                RenderData {
                    display_name: user.display_name().as_raw(),
                    public_id: user.public_id().as_raw(),
                    password: String::default(),
                    message: None,
                },
                handlebars_ref.as_ref(),
                FORM_NAME,
            ))
        },
        Err(e) => {
            // TODO render an error page
            log::error!("Failed to validate password reset key for {}: {}", path.0, e.message);
            Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, e.message).into())
        }
    }
}

#[post("/user/{id}/password_reset/{secret}")]
pub async fn post(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, form: Form<FormData>, path: Path<(i32, String)>) -> Result<impl Responder, actix_web::error::Error> {
    let account_id = oj_rc_core::types::AccountId::UserId(oj_rc_core::types::UserId::from_raw(path.0));
    match auth.web_password_secret_authenticate(account_id, path.1.clone()).await {
        Ok(user) => {
            if form.password != form.password_c {
                return Ok(crate::web::render_err(
                    RenderData {
                        display_name: user.display_name().as_raw(),
                        public_id: user.public_id().as_raw(),
                        password: form.password.clone(),
                        message: None,
                    },
                    "Passwords do not match".to_owned(),
                    handlebars_ref.as_ref(),
                    FORM_NAME,
                ))
            }
            if form.password.len() < 8 {
                return Ok(crate::web::render_err(
                    RenderData {
                        display_name: user.display_name().as_raw(),
                        public_id: user.public_id().as_raw(),
                        password: form.password.clone(),
                        message: None,
                    },
                    "Password too short (minimum 8 characters)".to_owned(),
                    handlebars_ref.as_ref(),
                    FORM_NAME,
                ))
            }
            if form.password.len() > 128 {
                return Ok(crate::web::render_err(
                    RenderData {
                        display_name: user.display_name().as_raw(),
                        public_id: user.public_id().as_raw(),
                        password: form.password.clone(),
                        message: None,
                    },
                    "Password too long (maximum 128 characters)".to_owned(),
                    handlebars_ref.as_ref(),
                    FORM_NAME,
                ))
            }
            match user.set_password(oj_rc_core::persist::user::PasswordVerificationMethod::SecretKey(path.1.clone()), &form.password).await {
                Ok(_) => {
                    Ok(crate::web::render_ok(
                        RenderData {
                            display_name: user.display_name().as_raw(),
                            public_id: user.public_id().as_raw(),
                            password: String::default(),
                            message: Some("Password changed successfully".to_owned()),
                        },
                        handlebars_ref.as_ref(),
                        FORM_NAME,
                    ))
                },
                Err(e) => {
                    Ok(crate::web::render_err(
                        RenderData {
                            display_name: user.display_name().as_raw(),
                            public_id: user.public_id().as_raw(),
                            password: form.password.clone(),
                            message: None,
                        },
                        format!("Password change failed: {}", e),
                        handlebars_ref.as_ref(),
                        FORM_NAME,
                    ))
                }
            }

        },
        Err(e) => {
            // TODO render an error page
            log::error!("Failed to validate password reset key for {}: {}", path.0, e.message);
            Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, e.message).into())
        }
    }
}
