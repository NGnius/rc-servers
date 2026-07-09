pub mod dashboard;
pub mod login;
pub mod favicon;
pub mod index;
pub mod garage;
pub mod user_federation;
pub mod logout;

use serde::Serialize;
use actix_web::{web::{Html, Redirect}, Responder};

fn version_string() -> String {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    //let license = env!("CARGO_PKG_LICENSE");
    //let repo = env!("CARGO_PKG_REPOSITORY");
    format!("OpenJam {} {}", name, version)
}

#[derive(Serialize)]
struct Context<T: Serialize> {
    form: T,
    error: Option<String>,
    version: String,
    source_url: String,
}

fn render_ok<T: Serialize>(form: T, renderer: &handlebars::Handlebars<'_>, form_name: &str) -> Html {
    let rendered = renderer.render(form_name, &Context {
        form,
        error: None,
        version: version_string(),
        source_url: env!("CARGO_PKG_REPOSITORY").to_owned(),
    }).unwrap();
    Html::new(rendered)
}

fn render_err<T: Serialize>(form: T, error: String , renderer: &handlebars::Handlebars<'_>, form_name: &str) -> Html {
    let rendered = renderer.render(form_name, &Context {
        form,
        error: Some(error),
        version: version_string(),
        source_url: env!("CARGO_PKG_REPOSITORY").to_owned(),
    }).unwrap();
    Html::new(rendered)
}

enum LoginReturn {
    Success(Box<dyn oj_rc_core::persist::user::WebUser>),
    AuthFail(actix_web::HttpResponse<actix_web::body::BoxBody>),
}

async fn try_auth_user(user_opt: Option<actix_identity::Identity>, auth: &oj_rc_core::UserImpl, req: &actix_web::HttpRequest) -> Result<LoginReturn, actix_web::Error> {
    if let Some(user) = user_opt {
        let user_id = user.id()?;
        match <oj_rc_core::UserImpl as oj_rc_core::UserProvider<()>>::web_authenticate(auth, user_id.clone()).await {
            Ok(user) => {
                //log::debug!("Web auth success");
                Ok(LoginReturn::Success(user))
            },
            Err(e) => {
                log::warn!("Failed to login with token {}: {} ({:?})", user_id, e.message, e.code);
                Ok(LoginReturn::AuthFail(
                    Redirect::to("/login")
                        .temporary()
                        .respond_to(req)
                        .map_into_boxed_body()
                ))
            }
        }
    } else {
        log::debug!("No user identity, redirecting to login page");
        Ok(LoginReturn::AuthFail(
            Redirect::to("/login")
                .temporary()
                .respond_to(req)
                .map_into_boxed_body()
        ))
    }
}
