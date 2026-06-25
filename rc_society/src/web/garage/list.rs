use actix_web::{get, web::Data, Responder, HttpRequest};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

use crate::web::{LoginReturn, try_auth_user, render_ok, render_err};

const FORM_NAME: &str = "garage_list";

#[derive(Serialize, Deserialize)]
struct RenderData {
    display_name: String,
    public_id: String,
    factory: String,
    garages: Vec<GarageData>,
}

#[derive(Serialize, Deserialize)]
struct GarageData {
    garage_id: i32,
    slot: i32,
    name: String,
    cpu: i32,
    max_cpu: i32,
    creation_time_unix: i64,
    creation_time_iso: String,
}

#[get("/garages/list")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, server_config: Data<oj_rc_core::persist::config::ServerConfig>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let html = match user.garages().await {
                Ok(mut garage_infos) => {
                    garage_infos.sort_by_key(|g| g.slot);
                    render_ok(
                        RenderData {
                            display_name: user.display_name().to_owned(),
                            public_id: user.public_id().to_owned(),
                            factory: server_config.factory_url.clone(),
                            garages: garage_infos.into_iter()
                                .map(|g| GarageData {
                                    garage_id: g.id,
                                    slot: g.slot,
                                    name: g.name,
                                    cpu: g.total_robot_cpu,
                                    max_cpu: g.bay_cpu,
                                    creation_time_unix: g.creation_time,
                                    creation_time_iso: chrono::DateTime::<chrono::Utc>::from_timestamp_secs(g.creation_time).unwrap_or_default().to_rfc3339(),
                                })
                                .collect(),
                        },
                        handlebars_ref.as_ref(),
                        FORM_NAME,
                    )
                },
                Err(e) => {
                    log::error!("Failed to retrieve garages for user {}: {}", user.public_id(), e);
                    render_err(
                        RenderData {
                            display_name: user.display_name().to_owned(),
                            public_id: user.public_id().to_owned(),
                            factory: server_config.factory_url.clone(),
                            garages: Vec::new(),
                        },
                        e.to_string(),
                        handlebars_ref.as_ref(),
                        FORM_NAME,
                    )
                }
            };
            Ok(
                html
                    .respond_to(&req)
                    .map_into_boxed_body()
            )
        }
    }
}
