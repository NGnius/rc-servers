use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header::{ContentType, AUTHORIZATION}, web::{Data, Path}};

#[get("/api/v1/activitypub/user/{name}/outbox")]
pub async fn get(name: Path<String>, auth: Data<Box<oj_rc_core::UserImpl>>, req: HttpRequest) -> impl Responder {
    get_impl(name.as_ref().to_owned(), auth, req).await
}

pub async fn get_impl(name: String, auth: Data<Box<oj_rc_core::UserImpl>>, req: HttpRequest) -> impl Responder {
    let json = if let Some(auth_token) = req.headers().get(AUTHORIZATION) {
        match super::auth_user(auth, auth_token).await {
            Ok(user) => {
                if user.public_id() != name {
                    log::error!("Wrong authenticated user: requested {}, token for {}", name, user.public_id());
                    empty_outbox(&name)
                } else {
                    match user.garages_full_ordered().await {
                        Ok(garages) => {
                            into_outbox(
                                &name,
                                garages.into_iter()
                                    .map(|g| super::super::super::garage::federation::build_vehicle_from_garage(g, &name))
                                    .collect(),
                            )
                        },
                        Err(e) => {
                            log::error!("Failed to retrieve garages for user {}: {}", name, e);
                            empty_outbox(&name)
                        }
                    }
                }
            },
            Err(e) => {
                log::error!("Failed to authenticated user: {}", e);
                empty_outbox(&name)
            }
        }
    } else {
        empty_outbox(&name)
    };
    let json_ctx = activitypub_federation::protocol::context::WithContext::new_default(json);
    let data = serde_json::to_string(&json_ctx).expect("Bad JSON serialization");
    HttpResponse::Ok()
        .insert_header(ContentType("application/activity+json".parse().unwrap()))
        .body(data)
}

#[inline]
fn empty_outbox(public_id: &str) -> super::Outbox<oj_serdes::society::activitypub::Vehicle> {
    into_outbox(public_id, Vec::default())
}

fn into_outbox(public_id: &str, vehicles: Vec<oj_serdes::society::activitypub::Vehicle>) -> super::Outbox<oj_serdes::society::activitypub::Vehicle> {
    let outbox_url = super::outbox(public_id);
    let inbox_url = super::inbox(public_id);
    super::Outbox {
        kind: super::ItemType::OrderedCollection,
        id: outbox_url.to_string(),
        total_items: vehicles.len() as _,
        ordered_items: vehicles,
        endpoints: [
            ("inbox".to_owned(), inbox_url.to_string()),
            ("outbox".to_owned(), outbox_url.to_string()),
        ].into_iter().collect(),
        icon: Some(oj_serdes::society::activitypub::Image::new(super::avatar_url(public_id))),
        image: None,
    }
}
