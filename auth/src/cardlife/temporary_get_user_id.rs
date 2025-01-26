//! Not a temporary file, this is the name of the endpoint
use rocket::{post, routes, serde::json::Json};

#[post("/api/auth/temporarygetuserid", data = "<body>")]
pub fn temp_migration(body: Json<libfj::cardlife::TempGetUserIdPayload>) -> Json<libfj::cardlife::TempGetUserIdResponse> {
    let steam_id = uuid::Uuid::parse_str(&body.public_id).map(|guid| guid.as_u64_pair().0).unwrap_or(123456);
    Json(libfj::cardlife::TempGetUserIdResponse {
        public_id: body.public_id.to_owned(),
        token: body.token.to_owned(),
        user_id: (steam_id & (i32::MAX as u64)) as i32,
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CardLife Temp UserId", |rocket| async {
        rocket.mount("/", routes![temp_migration])
    })
}
