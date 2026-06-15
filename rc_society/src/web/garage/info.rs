use actix_web::{get, web::{Data, Path}, Responder, HttpRequest};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

use crate::web::{LoginReturn, try_auth_user, render_ok, render_err};
use crate::api::garage::plugins::ImportPlugins;

const FORM_NAME: &str = "garage_info";

#[derive(Serialize, Deserialize)]
struct RenderData {
    display_name: String,
    public_id: String,
    garage: GarageData,
    exporters: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GarageData {
    garage_id: i32,
    slot: i32,
    name: String,
}

#[get("/garages/{id}/info")]
pub async fn get(id: Path<i32>, handlebars_ref: Data<handlebars::Handlebars<'_>>, exporter: Data<ImportPlugins>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let html = match user.garage_by_id(*id).await {
                Ok(Some(garage)) => {
                    render_ok(
                        RenderData {
                            display_name: user.display_name().to_owned(),
                            public_id: user.public_id().to_owned(),
                            garage: GarageData {
                                garage_id: *id,
                                slot: garage.slot,
                                name: garage.name.unwrap_or_default(),
                            },
                            exporters: exporter.plugin_names()
                                .map(|x| x.to_owned())
                                .collect(),
                        },
                        handlebars_ref.as_ref(),
                        FORM_NAME,
                    )
                },
                Ok(None) => {
                    log::error!("Failed to find vehicle {} for user {}", id, user.public_id());
                    render_err(
                        RenderData {
                            display_name: user.display_name().to_owned(),
                            public_id: user.public_id().to_owned(),
                            garage: GarageData {
                                garage_id: -1,
                                slot: -1,
                                name: String::default(),
                            },
                            exporters: Vec::new(),
                        },
                        "Vehicle not found".to_owned(),
                        handlebars_ref.as_ref(),
                        FORM_NAME,
                    )
                },
                Err(e) => {
                    log::error!("Failed to retrieve vehicle {} for user {}: {}", id, user.public_id(), e);
                    render_err(
                        RenderData {
                            display_name: user.display_name().to_owned(),
                            public_id: user.public_id().to_owned(),
                            garage: GarageData {
                                garage_id: -1,
                                slot: -1,
                                name: String::default(),
                            },
                            exporters: Vec::new(),
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
