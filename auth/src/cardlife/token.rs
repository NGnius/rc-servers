use rocket::{post, routes, serde::json::Json};

#[post("/api/auth/token", data = "<body>")]
pub fn token_auth(body: Json<libfj::cardlife::TokenPayload>) -> Json<libfj::cardlife::AuthenticationInfo> {
    todo!()
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CardLife Token", |rocket| async {
        rocket.mount("/", routes![token_auth])
    })
}
