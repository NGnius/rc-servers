use actix_web::{get, post, web::{Data, Path, Redirect, Form}, Responder, HttpRequest};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

use crate::web::{LoginReturn, try_auth_user, render_ok};

const FORM_NAME: &str = "user_federation";

#[derive(Serialize, Deserialize)]
struct RenderData {
    display_name: String,
    public_id: String,
    enabled: bool,
    defederated: Vec<String>,
}

async fn list_impl(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let fedi = user.fedi_get().await;
            let html = render_ok(
                RenderData {
                    display_name: user.display_name().to_owned(),
                    public_id: user.public_id().to_owned(),
                    enabled: fedi.enabled,
                    defederated: fedi.defederated,
                },
                handlebars_ref.as_ref(),
                FORM_NAME,
            );
            Ok(
                html
                    .respond_to(&req)
                    .map_into_boxed_body()
            )
        }
    }
}

#[get("/federation/list")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    list_impl(handlebars_ref, auth, user_opt, req).await
}

#[post("/federation/list")]
pub async fn post(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    list_impl(handlebars_ref, auth, user_opt, req).await
}

#[post("/federation/list/remove/{domain}")]
pub async fn post_remove(auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest, domain: Path<String>) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let mut fedi = user.fedi_get().await;
            if let Some((i, _domain_name)) = fedi.defederated.iter().enumerate().find(|(_i, domain_name)| *domain_name == &*domain) {
                fedi.defederated.remove(i);
                user.fedi_set(fedi).await;
            } else {
                log::warn!("Failed to find domain {} in defederated list for user {}", &*domain, user.public_id());
            }
            let resp = Redirect::to("/federation/list")
                .respond_to(&req)
                .map_into_boxed_body();
            Ok(
                resp
                    .respond_to(&req)
                    .map_into_boxed_body()
            )
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AddForm {
    domain: String,
}

#[post("/federation/list/add")]
pub async fn post_add(auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest, form: Form<AddForm>) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let mut fedi = user.fedi_get().await;
            let sanitized_domain = form.domain.trim().to_lowercase();
            if fedi.defederated.contains(&sanitized_domain) {
                log::warn!("Domain {} already in defederated list for user {}", sanitized_domain, user.public_id());
            } else {
                fedi.defederated.push(sanitized_domain);
                user.fedi_set(fedi).await;
            }
            let resp = Redirect::to("/federation/list")
                .respond_to(&req)
                .map_into_boxed_body();
            Ok(
                resp
                    .respond_to(&req)
                    .map_into_boxed_body()
            )
        }
    }
}

#[post("/federation/off")]
pub async fn post_off(auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let mut fedi = user.fedi_get().await;
            fedi.enabled = false;
            user.fedi_set(fedi).await;
            let resp = Redirect::to("/federation/list")
                .respond_to(&req)
                .map_into_boxed_body();
            Ok(
                resp
                    .respond_to(&req)
                    .map_into_boxed_body()
            )
        }
    }
}

#[post("/federation/on")]
pub async fn post_on(auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let mut fedi = user.fedi_get().await;
            fedi.enabled = true;
            user.fedi_set(fedi).await;
            let resp = Redirect::to("/federation/list")
                .respond_to(&req)
                .map_into_boxed_body();
            Ok(
                resp
                    .respond_to(&req)
                    .map_into_boxed_body()
            )
        }
    }
}
