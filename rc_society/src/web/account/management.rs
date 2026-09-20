use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web::{Data, Form}};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

use crate::web::{LoginReturn, try_auth_user, render_ok, render_err};

const FORM_NAME: &str = "user_management";

#[derive(Serialize, Deserialize)]
struct RenderData {
    display_name: String,
    public_id: String,
    success_message: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "action")]
enum FormData {
    NameChange {
        original_name: String,
        new_name: String,
    },
}

async fn render_page(handlebars_ref: &handlebars::Handlebars<'_>, user: Box<dyn oj_rc_core::persist::user::WebUser>, req: HttpRequest, message: Option<String>) -> HttpResponse {
    let html = render_ok(
        RenderData {
            display_name: user.display_name().to_owned(),
            public_id: user.public_id().to_owned(),
            success_message: message,
        },
        handlebars_ref,
        FORM_NAME,
    );
    html
        .respond_to(&req)
        .map_into_boxed_body()
}

#[get("/user/manage")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    let resp = match try_auth_user(&user_opt, auth.as_ref(), &req).await? {
        LoginReturn::Success(user) => {
            render_page(handlebars_ref.as_ref(), user, req, None).await
        },
        LoginReturn::AuthFail(resp) => resp,
    };
    Ok(resp)
}

#[post("/user/manage")]
pub async fn post(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, form: Form<FormData>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    let resp = match try_auth_user(&user_opt, auth.as_ref(), &req).await? {
        LoginReturn::Success(user) => {
            match &*form {
                FormData::NameChange { original_name, new_name } => {
                    let resp = name_change_impl(original_name, new_name, handlebars_ref.as_ref(), user, req).await?;
                    user_opt.expect("Must be authenticated by this point").logout();
                    resp
                },
            }
        },
        LoginReturn::AuthFail(resp) => resp,
    };
    Ok(resp)
}

async fn name_change_impl(old_name: &str, new_name: &str, handlebars: &handlebars::Handlebars<'_>, user: Box<dyn oj_rc_core::persist::user::WebUser>, req: HttpRequest) -> Result<HttpResponse, actix_web::error::Error> {
    let resp = if new_name == old_name {
        render_err(
            RenderData {
                display_name: user.display_name().to_owned(),
                public_id: user.public_id().to_owned(),
                success_message: None,
            },
            "Display name is already set to that".to_owned(),
            handlebars,
            FORM_NAME,
        )
            .respond_to(&req)
            .map_into_boxed_body()
    } else if old_name != user.display_name() {
        render_err(
            RenderData {
                display_name: user.display_name().to_owned(),
                public_id: user.public_id().to_owned(),
                success_message: None,
            },
            "Display name is out of sync with this page".to_owned(),
            handlebars,
            FORM_NAME,
        )
            .respond_to(&req)
            .map_into_boxed_body()
    } else {
        let html = match user.set_display_name(new_name).await {
            Ok(_) => {
                render_ok(
                    RenderData {
                        display_name: new_name.to_owned(),
                        public_id: user.public_id().to_owned(),
                        success_message: Some(format!("Successfully changed display name from {} to {}", user.display_name(), new_name)),
                    },
                    handlebars,
                    FORM_NAME,
                )
            },
            Err(e) => {
                render_err(
                    RenderData {
                        display_name: user.display_name().to_owned(),
                        public_id: user.public_id().to_owned(),
                        success_message: None,
                    },
                    format!("Name change failed: {}", e),
                    handlebars,
                    FORM_NAME,
                )
            }
        };
        html
            .respond_to(&req)
            .map_into_boxed_body()
    };
    Ok(resp)
}
