use actix_web::{get, web::Data, Responder, HttpRequest};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

pub const FORM_NAME: &str = "index";

#[derive(Serialize, Deserialize)]
struct RenderData {
    is_logged_in: bool,
    display_name: Option<String>,
    server: ServerDetails,
}

#[derive(Serialize, Deserialize)]
struct ServerDetails {
    domain: String,
    cdn: String,
    auth: String,
    min_version: i32,
    server_version: String,
    start_time_iso: String,
    start_time_unix: i64,
}

fn server_details(conf: &oj_rc_core::persist::config::ServerConfig) -> ServerDetails {
    let start_time_unix = crate::START_TIME.load(std::sync::atomic::Ordering::Relaxed);
    let start_time_chrono = chrono::DateTime::<chrono::Utc>::from_timestamp_secs(start_time_unix).unwrap_or_default();
    let version = env!("CARGO_PKG_VERSION");
    let git_version = git_version::git_version!(args = ["--always", "--dirty=+"]);
    let server_version = format!("{}:{}", version, git_version);
    ServerDetails {
        domain: conf.domain.clone(),
        cdn: conf.cdn_url.clone(),
        auth: conf.auth_url.clone(),
        min_version: conf.minimum_version,
        server_version,
        start_time_unix,
        start_time_iso: start_time_chrono.to_rfc3339(),
    }
}

#[get("/")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, server_config: Data<oj_rc_core::persist::config::ServerConfig>,  user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    let server_info = server_details(server_config.as_ref());
    if let Some(user) = user_opt {
        match super::try_auth_user(Some(user), auth.as_ref(), &req).await? {
            super::LoginReturn::AuthFail(resp) => Ok(resp),
            super::LoginReturn::Success(user) => {
                Ok(super::render_ok(
                    RenderData {
                        is_logged_in: true,
                        display_name: Some(user.display_name().to_owned()),
                        server: server_info,
                    },
                    handlebars_ref.as_ref(),
                    FORM_NAME,
                )
                    .respond_to(&req)
                    .map_into_boxed_body()
                )
            }
        }
    } else {
        Ok(super::render_ok(
            RenderData {
                is_logged_in: false,
                display_name: None,
                server: server_info,
            },
            handlebars_ref.as_ref(),
            FORM_NAME,
        )
            .respond_to(&req)
            .map_into_boxed_body()
        )
    }
}
