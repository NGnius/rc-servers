use actix_web::{HttpRequest, Responder, get, web::{Data, Path, Redirect}};
use actix_identity::Identity;
use oj_rc_core::types::RawType;

use crate::web::{LoginReturn, try_auth_user, render_err};

#[get("/user/manage/password_reset_as/{id}")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest, path: Path<i32>) -> Result<impl Responder, actix_web::error::Error> {
    let resp = match try_auth_user(&user_opt, auth.as_ref(), &req).await? {
        LoginReturn::Success(user) => {
            match user.do_as(oj_rc_core::types::AccountId::UserId(oj_rc_core::types::UserId::from_raw(*path))).await {
                Ok(new_user) => {
                    match new_user.password_reset().generate_password_reset_key().await {
                        Ok(secret) => {
                            let user_id = new_user.user_id();
                            let url = format!("/user/{}/password_reset/{}", user_id.as_raw(), secret);
                            Redirect::to(url)
                                .temporary()
                                .respond_to(&req)
                                .map_into_boxed_body()
                        },
                        Err(e) => {
                            log::error!("Failed to generate password secret for {} as {}: {}", user.user_id(), new_user.user_id(), e);
                            let html = render_err(
                                crate::web::MessageRenderData {
                                    display_name: Some(user.display_name().as_raw()),
                                    public_id: Some(user.public_id().as_raw()),
                                    message: None,
                                },
                                format!("Failed to generate password reset URL: {}", e),
                                handlebars_ref.as_ref(),
                                crate::web::MESSAGE_FORM_NAME,
                            );
                            html
                                .respond_to(&req)
                                .map_into_boxed_body()
                        }
                    }
                },
                Err(e) => {
                    let html = render_err(
                        crate::web::MessageRenderData {
                            display_name: Some(user.display_name().as_raw()),
                            public_id: Some(user.public_id().as_raw()),
                            message: None,
                        },
                        format!("Failed to generate password reset URL: {}", e),
                        handlebars_ref.as_ref(),
                        crate::web::MESSAGE_FORM_NAME,
                    );
                    html
                        .respond_to(&req)
                        .map_into_boxed_body()
                }
            }
        },
        LoginReturn::AuthFail(resp) => resp,
    };
    Ok(resp)
}
