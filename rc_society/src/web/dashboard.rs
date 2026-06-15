use actix_web::{get, post, web::Data, Responder, HttpRequest};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

const FORM_NAME: &str = "dashboard";

#[derive(Serialize, Deserialize)]
struct RenderData {
    // TODO
    display_name: String,
    public_id: String,
    debug: DebugData,
    perms: PermissionData,
}

#[derive(Serialize, Deserialize)]
struct DebugData {
    user_id: i32,
    creation_time_unix: i64,
    creation_time_iso: String,
}

#[derive(Serialize, Deserialize)]
struct PermissionData {
    r#mod: bool,
    admin: bool,
    dev: bool,
    royal: bool,
    banned: bool,
}

pub async fn dashboard_impl(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match super::try_auth_user(user_opt, auth.as_ref(), &req).await? {
        super::LoginReturn::AuthFail(resp) => Ok(resp),
        super::LoginReturn::Success(user) => {
            // TODO
            log::debug!("Rendering user's dashboard");
            let creation_time = user.creation();
            let creation_time_chrono = chrono::DateTime::<chrono::Utc>::from_timestamp_secs(creation_time).unwrap_or_default();
            Ok(super::render_ok(
                RenderData {
                    display_name: user.display_name().to_owned(),
                    public_id: user.public_id().to_owned(),
                    debug: DebugData {
                        user_id: user.account_id(),
                        creation_time_unix: creation_time,
                        creation_time_iso: creation_time_chrono.to_rfc3339(),
                    },
                    perms: PermissionData {
                        r#mod: user.is_mod(),
                        admin: user.is_admin(),
                        dev: user.is_dev(),
                        royal: user.is_royal(),
                        banned: user.is_banned(),
                    }
                },
                handlebars_ref.as_ref(),
                FORM_NAME,
            )
                .respond_to(&req)
                .map_into_boxed_body()
            )
        }
    }
}

#[get("/dashboard")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    dashboard_impl(handlebars_ref, auth, user_opt, req).await
}

#[post("/dashboard")]
pub async fn post(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    dashboard_impl(handlebars_ref, auth, user_opt, req).await
}
