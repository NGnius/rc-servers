use actix_web::{get, post, web::{Data, Form}, Responder};
use oj_rc_core::{UserProvider, types::RawType};
use serde::{Serialize, Deserialize};

const FORM_NAME: &str = "user_password_reset_email";

#[derive(Serialize, Deserialize)]
struct PasswordResetForm {
    email: String,
}

#[post("/reset")]
pub async fn post(form: Form<PasswordResetForm>, email_sender: Data<oj_rc_core::email::EmailSender>, auth: Data<Box<oj_rc_core::UserImpl>>, handlebars_ref: Data<handlebars::Handlebars<'_>>, server_config: Data<oj_rc_core::persist::config::ServerConfig>) -> Result<impl Responder, actix_web::error::Error> {
    if form.email.is_empty() {
        return Ok(crate::web::render_err(
            form.into_inner(),
            "Missing email address".to_owned(),
            handlebars_ref.as_ref(),
            FORM_NAME,
        ));
    }
    let account_id = oj_rc_core::types::AccountId::Email(form.email.clone());
    match auth.web_password_reset_authenticate(account_id.clone()).await {
        Err(e) => {
            // silently fail to prevent bots from brute forcing
            log::debug!("Failed to authenticate password reset: {}", e.message);
            Ok(crate::web::render_ok(
                crate::web::MessageRenderData {
                    display_name: None,
                    public_id: None,
                    message: Some("Message sent, check your email...".to_owned()),
                },
                handlebars_ref.as_ref(),
                crate::web::MESSAGE_FORM_NAME,
            ))
        },
        Ok(reset_user) => {
            let secret = match reset_user.generate_password_reset_key().await {
                Ok(s) => s,
                Err(e) => {
                    log::error!("Failed to generate password reset secret: {}", e);
                    return Ok(crate::web::render_err(
                        form.into_inner(),
                        "Failed to generate reset secret".to_owned(),
                        handlebars_ref.as_ref(),
                        FORM_NAME,
                    ));
                }
            };
            let base_url = server_config.society_url.clone();
            let full_url = format!("{}/user/{}/password_reset/{}", base_url, reset_user.user_id().as_raw(), secret);
            match email_sender.send_email(
                account_id,
                "OpenJam Password Reset".to_owned(),
                format!("<a href=\"{}\">Reset password here!</a><p>aka {}</p><p>If you did not try to reset your password, you can ignore this email.</p>", full_url, full_url),
            ).await {
                Ok(_) => Ok(crate::web::render_ok(
                    crate::web::MessageRenderData {
                        display_name: None,
                        public_id: None,
                        message: Some("Message sent, check your email...".to_owned()),
                    },
                    handlebars_ref.as_ref(),
                    crate::web::MESSAGE_FORM_NAME,
                )),
                Err(e) => Ok(crate::web::render_err(
                    form.into_inner(),
                    format!("Failed to send email: {}", e),
                    handlebars_ref.as_ref(),
                    FORM_NAME,
                ))
            }
        }
    }
}

#[get("/reset")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>) -> impl Responder {
    crate::web::render_ok(
        PasswordResetForm {
            email: String::default(),
        },
        handlebars_ref.as_ref(),
        FORM_NAME,
    )
}
