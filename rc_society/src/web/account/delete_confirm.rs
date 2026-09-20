use actix_web::{HttpRequest, Responder, get, post, web::{Data, Form, Redirect}};
use actix_identity::Identity;

use crate::web::{LoginReturn, try_auth_user, render_ok, render_err};

const URL: &str = "/user/delete";

#[get("/user/delete")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    let resp = match try_auth_user(&user_opt, auth.as_ref(), &req).await? {
        LoginReturn::Success(user) => {
            let html = render_ok(
                crate::web::ConfirmRenderData {
                    display_name: user.display_name().to_owned(),
                    public_id: user.public_id().to_owned(),
                    question: "Are you sure you want to delete your account?".to_owned(),
                    yes_url: URL.to_owned(),
                    no_url: "/user/manage".to_owned(),
                    message: None,
                },
                handlebars_ref.as_ref(),
                crate::web::CONFIRM_FORM_NAME,
            );
            html
                .respond_to(&req)
                .map_into_boxed_body()
        },
        LoginReturn::AuthFail(resp) => resp,
    };
    Ok(resp)
}

#[post("/user/delete")]
pub async fn post(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, form: Form<crate::web::ConfirmFormData>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    let resp = match try_auth_user(&user_opt, auth.as_ref(), &req).await? {
        LoginReturn::Success(user) => {
            match &*form {
                crate::web::ConfirmFormData::Yes => {
                    // user has confirmed
                    let html = if let Err(e) = user.delete_account().await {
                        render_err(
                            crate::web::ConfirmRenderData {
                                display_name: user.display_name().to_owned(),
                                public_id: user.public_id().to_owned(),
                                question: "Are you sure you want to delete your account?".to_owned(),
                                yes_url: URL.to_owned(),
                                no_url: "/user/manage".to_owned(),
                                message: None,
                            },
                            format!("Failed to delete user #{} ({}): {}", user.account_id(), user.display_name(), e),
                            handlebars_ref.as_ref(),
                            crate::web::CONFIRM_FORM_NAME,
                        )
                    } else {
                        user_opt.expect("Must be authenticated by this point").logout();
                        render_ok(
                            crate::web::ConfirmRenderData {
                                display_name: user.display_name().to_owned(),
                                public_id: user.public_id().to_owned(),
                                question: "Are you sure you want to delete your account?".to_owned(),
                                yes_url: "/".to_owned(),
                                no_url: "/".to_owned(),
                                message: Some(format!("User #{} ({}) deleted successfully", user.account_id(), user.display_name())),
                            },
                            handlebars_ref.as_ref(),
                            crate::web::CONFIRM_FORM_NAME,
                        )
                    };

                    html
                        .respond_to(&req)
                        .map_into_boxed_body()
                },
                crate::web::ConfirmFormData::No => {
                    // user has declined, abort
                    Redirect::to("/user/manage")
                        .temporary()
                        .respond_to(&req)
                        .map_into_boxed_body()
                }
            }
        },
        LoginReturn::AuthFail(resp) => resp,
    };
    Ok(resp)
}
