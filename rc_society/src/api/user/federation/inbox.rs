use actix_web::{HttpRequest, HttpResponse, Responder, get, http::{StatusCode, header::AUTHORIZATION}, post, web::{Data, Path, Bytes}};

use oj_rc_core::persist::user::federation::{Activity, ActivityType};

#[get("/api/v1/activitypub/user/{name}/inbox")]
pub async fn get(name: Path<String>, auth: Data<Box<oj_rc_core::UserImpl>>, req: HttpRequest) -> impl Responder {
    super::outbox::get_impl(name.as_ref().to_owned(), auth, req).await
}

#[post("/api/v1/activitypub/user/{name}/inbox")]
pub async fn post(name: Path<String>, auth: Data<Box<oj_rc_core::UserImpl>>, parsers: Data<oj_rc_core::cubes::CubeParsers>,  req: HttpRequest, body: Bytes) -> Result<impl Responder, actix_web::error::Error> {
    let name: &str = &name;
    if let Some(auth_token) = req.headers().get(AUTHORIZATION) {
        match super::auth_user(auth, auth_token).await {
            Ok(user) => {
                if user.public_id() != name {
                    log::error!("Wrong authenticated user: requested {}, token for {}", name, user.public_id());
                    Err(InboxError {
                        message: "Forbidden Authorization token".to_owned(),
                        code: StatusCode::FORBIDDEN,
                    }.into())
                } else {
                    let activity: Activity<oj_serdes::society::activitypub::Vehicle> = serde_json::from_reader(std::io::Cursor::new(&body))
                        .map_err(|e| {
                            log::warn!("Received invalid vehicle activity for user {} inbox: {}", name, e);
                            InboxError {
                                message: format!("Invalid vehicle activity: {}", e),
                                code: StatusCode::BAD_REQUEST,
                            }
                        })?;
                    let is_create = matches!(activity.kind, ActivityType::Create);
                    let g = activity.object;
                    let transformed = oj_rc_core::persist::user::FullVehicleData {
                        id: -1,
                        creation_time: 0,
                        slot: g.slot as _,
                        name: g.name,
                        crf_id: None,
                        was_rated: false,
                        movement_categories: g.movement_categories,
                        uuid: 0,
                        total_robot_cpu: g.total_robot_cpu as _,
                        total_cosmetic_cpu: g.total_cosmetic_cpu as _,
                        total_robot_ranking: g.total_robot_ranking as _,
                        bay_cpu: g.bay_cpu as _,
                        control: oj_rc_core::persist::user::ControlData {
                            slot: g.slot as _,
                            control_ty: if g.is_camera_controls {
                                oj_rc_core::persist::user::ControlType::Camera
                            } else {
                                oj_rc_core::persist::user::ControlType::Keyboard
                            },
                            vertical_strafing: g.vertical_strafing,
                            sideways_driving: g.sideways_driving,
                            tracks_turn_on_spot: g.tracks_turn_on_spot,
                        },
                        mastery_level: g.mastery_level as _,
                        bay_skin: g.bay_skin,
                        death_animation: g.death_animation,
                        spawn_animation: g.spawn_animation,
                        weapon_order: g.weapon_order,
                        robot_data: g.robot_data,
                        colour_data: g.colour_data,
                        selected: g.selected,
                    };
                    user.save_federated_garage(
                        is_create,
                        transformed,
                        parsers.cpu_counter().as_ref(),
                        parsers.weapon_order().as_ref(),
                    ).await
                    .map_err(|e| {
                        log::error!("Failed to save vehicle from activitypub-ed user {} inbox: {}", name, e);
                        InboxError {
                            message: format!("Invalid vehicle activity: {}", e),
                            code: StatusCode::BAD_REQUEST,
                        }
                    })?;
                    Ok(HttpResponse::NoContent())
                }
            },
            Err(e) => {
                log::error!("Failed to authenticated user: {}", e);
                Err(InboxError {
                    message: "Invalid Authorization token".to_owned(),
                    code: StatusCode::FORBIDDEN,
                }.into())
            }
        }
    } else {
        Err(InboxError {
            message: "Missing Authorization header".to_owned(),
            code: StatusCode::UNAUTHORIZED,
        }.into())
    }
}

#[derive(Debug)]
struct InboxError {
    message: String,
    code: StatusCode,
}

impl core::fmt::Display for InboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Inbox error: {}", self.message)
    }
}

impl core::error::Error for InboxError {}

impl actix_web::error::ResponseError for InboxError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.code
    }
}
