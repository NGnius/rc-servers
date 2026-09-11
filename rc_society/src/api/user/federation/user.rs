use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header::{ContentType, AUTHORIZATION}, web::{Data, Path}};
use oj_serdes::society::activitypub::ObjectContext;

#[get("/api/v1/activitypub/user/{name}")]
pub async fn get(name: Path<String>, auth: Data<Box<oj_rc_core::UserImpl>>, req: HttpRequest) -> impl Responder {
    let json = if let Some(auth_token) = req.headers().get(AUTHORIZATION) {
        match super::auth_user(auth, auth_token).await {
            Ok(user) => {
                if user.public_id() != *name {
                    log::error!("Wrong authenticated user: requested {}, token for {}", name, user.public_id());
                    super::build_person_from_public_id(&name)
                } else {
                    super::build_person_from_user(user.as_ref()).await.unwrap_or_else(|e| {
                        log::error!("Failed to build ActivityPub Person: {}", e);
                        super::build_person_from_public_id(&name)
                    })
                }
            },
            Err(e) => {
                log::error!("Failed to authenticated user: {}", e);
                super::build_person_from_public_id(&name)
            }
        }
    } else {
        super::build_person_from_public_id(&name)
    };
    let json_ctx = json.default_context();
    let data = serde_json::to_string(&json_ctx).expect("Bad JSON serialization");
    HttpResponse::Ok()
        .insert_header(ContentType("application/activity+json".parse().unwrap()))
        .body(data)
}
