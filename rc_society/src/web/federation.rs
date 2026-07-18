use actix_web::{get, web::Data, Responder, HttpRequest};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

use crate::web::{LoginReturn, try_auth_user, render_ok, render_err};

const FORM_NAME: &str = "federation";

#[derive(Serialize, Deserialize)]
struct RenderData {
    display_name: String,
    public_id: String,
    instances: Vec<FederatedServer>,
}

#[derive(Serialize, Deserialize)]
struct FederatedServer {
    id: i32,
    domain: String,
    auth: String,
    society: String,
    last_used_unix: i64,
    last_used_iso: String,
    first_used_unix: i64,
    first_used_iso: String,
}

#[get("/federation")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let fedi_info = match build_fedi_info(user.as_ref()).await {
                Ok(x) => x,
                Err(e) => {
                    let html = render_err(
                        RenderData {
                            display_name: user.display_name().to_owned(),
                            public_id: user.public_id().to_owned(),
                            instances: Vec::default(),
                        },
                        e.to_string(),
                        handlebars_ref.as_ref(),
                        FORM_NAME,
                    );
                    return Ok(
                        html
                            .respond_to(&req)
                            .map_into_boxed_body()
                    );
                },
            };
            let html = render_ok(
                RenderData {
                    display_name: user.display_name().to_owned(),
                    public_id: user.public_id().to_owned(),
                    instances: fedi_info,
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

async fn build_fedi_info(user: &dyn oj_rc_core::persist::user::WebUser) -> Result<Vec<FederatedServer>, Box<dyn std::error::Error>> {
    let info = user.fedi_info().await?;
    Ok(info.federations.into_iter()
        .map(|instance| FederatedServer {
            id: instance.id,
            domain: instance.domain,
            auth: instance.auth,
            society: instance.society,
            last_used_unix: instance.last_used,
            last_used_iso: chrono::DateTime::<chrono::Utc>::from_timestamp_secs(instance.last_used).unwrap_or_default().to_rfc3339(),
            first_used_unix: instance.first_used,
            first_used_iso: chrono::DateTime::<chrono::Utc>::from_timestamp_secs(instance.first_used).unwrap_or_default().to_rfc3339(),
        }).collect()
    )
}
