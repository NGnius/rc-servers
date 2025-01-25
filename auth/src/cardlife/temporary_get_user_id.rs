//! Not a temporary file, this is the name of the endpoint
use rocket::{post, routes, serde::json::Json};

#[post("/api/auth/temporarygetuserid", data = "<body>")]
pub fn temp_migration(body: Json<libfj::cardlife::TempGetUserIdPayload>) -> Json<libfj::cardlife::TempGetUserIdResponse> {
    Json(libfj::cardlife::TempGetUserIdResponse {
        public_id: body.public_id.to_owned(),
        token: body.token.to_owned(),
        user_id: 123456, // FIXME
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CardLife Temp UserId", |rocket| async {
        rocket.mount("/", routes![temp_migration])
    })
}
