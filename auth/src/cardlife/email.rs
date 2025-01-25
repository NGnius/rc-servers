use rocket::{post, routes, serde::json::Json};

#[post("/api/auth/authenticate", data = "<body>")]
pub fn email_password_auth(body: Json<libfj::cardlife::AuthenticationPayload>) -> Json<libfj::cardlife::AuthenticationInfo> {
    todo!()
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CardLife Email/Password", |rocket| async {
        rocket.mount("/", routes![email_password_auth])
    })
}
