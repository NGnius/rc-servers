use actix_web::{get, post, web::{Data, Path}, Responder, HttpRequest};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

use crate::web::{LoginReturn, try_auth_user, render_ok, render_err};
use crate::api::garage::plugins::ImportPlugins;

const FORM_NAME: &str = "garage_import";

#[derive(Serialize, Deserialize)]
struct RenderData {
    display_name: String,
    public_id: String,
    garage: Option<GarageData>,
    importers: Vec<String>,
    selected_importer: Option<String>,
    success: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GarageData {
    garage_id: i32,
    slot: i32,
}

async fn import_impl(id: Option<i32>, handlebars_ref: Data<handlebars::Handlebars<'_>>, importer: Data<ImportPlugins>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let html = if let Some(id) = id {
                match user.garage_by_id(id).await {
                    Ok(Some(garage)) => {
                        render_ok(
                            RenderData {
                                display_name: user.display_name().to_owned(),
                                public_id: user.public_id().to_owned(),
                                garage: Some(GarageData {
                                    garage_id: id,
                                    slot: garage.slot,
                                }),
                                importers: importer.plugin_names()
                                    .map(|x| x.to_owned())
                                    .collect(),
                                selected_importer: None,
                                success: None,
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
                                garage: Some(GarageData {
                                    garage_id: -1,
                                    slot: -1,
                                }),
                                importers: Vec::new(),
                                selected_importer: None,
                                success: None,
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
                                garage: Some(GarageData {
                                    garage_id: -1,
                                    slot: -1,
                                }),
                                importers: Vec::new(),
                                selected_importer: None,
                                success: None,
                            },
                            e.to_string(),
                            handlebars_ref.as_ref(),
                            FORM_NAME,
                        )
                    }
                }
            } else {
                render_ok(
                    RenderData {
                        display_name: user.display_name().to_owned(),
                        public_id: user.public_id().to_owned(),
                        garage: None,
                        importers: importer.plugin_names()
                            .map(|x| x.to_owned())
                            .collect(),
                        selected_importer: None,
                        success: None,
                    },
                    handlebars_ref.as_ref(),
                    FORM_NAME,
                )
            };
            Ok(
                html
                    .respond_to(&req)
                    .map_into_boxed_body()
            )
        }
    }
}

#[get("/garages/{id}/import")]
pub async fn get_existing(id: Path<i32>, handlebars_ref: Data<handlebars::Handlebars<'_>>, importer: Data<ImportPlugins>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    import_impl(Some(*id), handlebars_ref, importer, auth, user_opt, req).await
}

#[get("/garages/import")]
pub async fn get_new(handlebars_ref: Data<handlebars::Handlebars<'_>>, importer: Data<ImportPlugins>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    import_impl(None, handlebars_ref, importer, auth, user_opt, req).await
}

#[derive(Debug, actix_multipart::form::MultipartForm)]
struct ImportForm {
    //#[multipart]
    files: Vec<actix_multipart::form::bytes::Bytes>,
    garage_id: actix_multipart::form::text::Text<i32>,
    plugin: actix_multipart::form::text::Text<String>,

}

#[post("/garages/import")]
pub async fn post(form: actix_multipart::form::MultipartForm<ImportForm>, handlebars_ref: Data<handlebars::Handlebars<'_>>, importer: Data<ImportPlugins>, parsers: Data<oj_rc_core::cubes::CubeParsers>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let total_size: usize = form.files.iter().map(|f| f.data.len()).sum();
            if form.files.is_empty() {
                return Err(super::PluginPortError {
                    code: oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid,
                }.into());
            }
            if *form.garage_id != -1 && form.files.len() != 1 {
                return Err(super::PluginPortError {
                    code: oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid,
                }.into());
            }
            for (i, file) in form.files.iter().enumerate() {
                #[cfg(debug_assertions)]
                log::trace!("import {} file {} data: {:?}", &*form.plugin, i, &file.data[..]);
                let import_data = match importer.import_by_name(&form.plugin, &file.data)
                    .map_err(|e| super::PluginPortError { code: e }) {
                    Ok(x) => x,
                    Err(e) => {
                        let html = render_err(
                            RenderData {
                                display_name: user.display_name().to_owned(),
                                public_id: user.public_id().to_owned(),
                                garage: None,
                                importers: importer.plugin_names()
                                    .map(|x| x.to_owned())
                                    .collect(),
                                selected_importer: Some(form.plugin.0.clone()),
                                success: None,
                            },
                            format!("Failed to import file {}: {}", i, e),
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
                let vehicle_data = oj_rc_core::persist::user::VehicleData {
                    name: import_data.vehicle_name,
                    slot: -1, // will be overriden in oj_rc_core
                    robot_data: import_data.cube_data,
                    colour_data: import_data.colour_data,
                    weapon_order: Vec::default(), // will be overriden in oj_rc_core
                    crf_id: None, // irrelevant
                    was_rated: None, // irrelevant
                };
                user.save_garage(
                    vehicle_data,
                    if *form.garage_id < 0 { None } else { Some(*form.garage_id) }, // multipart crate silliness
                    parsers.cpu_counter().as_ref(),
                    parsers.weapon_order().as_ref(),
                ).await?;
            }
            let html = render_ok(
                RenderData {
                    display_name: user.display_name().to_owned(),
                    public_id: user.public_id().to_owned(),
                    garage: None,
                    importers: importer.plugin_names()
                        .map(|x| x.to_owned())
                        .collect(),
                    selected_importer: Some(form.plugin.0.clone()),
                    success: Some(format!(
                        "Imported {} vehicle{} ({:.2}KiB)",
                        form.files.len(),
                        if form.files.len() > 1 { "s" } else { "" },
                        total_size as f64 / 1024.0,
                    )),
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
