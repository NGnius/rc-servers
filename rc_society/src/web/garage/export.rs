use actix_web::{HttpRequest, HttpResponse, Responder, get, http::StatusCode, web::{Data, Path, Query}};
use actix_identity::Identity;

use oj_rc_plugins::vehicle_import::VehicleImportData;

use crate::web::{LoginReturn, try_auth_user};
use crate::api::garage::plugins::ImportPlugins;

#[get("/garages/{id}/export")]
pub async fn get(id: Path<i32>, query: Query<super::PortQuery>, exporter: Data<ImportPlugins>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            match user.garage_by_id(*id).await {
                Ok(Some(garage)) => {
                    let data = VehicleImportData {
                        cube_data: garage.robot_data,
                        colour_data: garage.colour_data,
                        vehicle_name: garage.name,
                        vehicle_author: Some(user.display_name().to_owned()),
                    };
                    match exporter.export_by_name(&query.plugin, &data) {
                        Ok(export) => {
                            //export.push(b'\n');
                            use actix_web::http::header::{ContentDisposition, TryIntoHeaderPair};
                            let ext = exporter.file_ext(&query.plugin).unwrap();
                            // TODO sanitise vehicle name and include it in the filename
                            let dispo = ContentDisposition::attachment(format!("export-{}-{}.{}", &query.plugin, *id, ext));
                            let (key, val) = dispo.try_into_pair().unwrap();
                            let mut resp = HttpResponse::with_body(StatusCode::OK, export);
                            resp.headers_mut().append(key, val);
                            Ok(resp.map_into_boxed_body())
                        },
                        Err(e) => {
                            log::error!("Failed export of vehicle {} for user {}: {}", id, user.public_id(), e);
                            Err(super::PluginPortError::from(e).into())
                        }
                    }
                },
                Ok(None) => {
                    Ok(HttpResponse::NotFound()
                        .finish())
                },
                Err(e) => {
                    log::error!("Failed to load garage ID for user {}: {}", user.public_id(), e);
                    Ok(HttpResponse::InternalServerError()
                        .finish())
                }
            }
        }
    }
}
