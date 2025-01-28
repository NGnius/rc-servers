use rocket::{post, routes};

#[post("/", data = "<body>")]
pub fn debug_endpoint(body: String) -> String {
    log::info!("got body: `{}`", body);
    body.to_string()
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Robocraft debug", |rocket| async {
        rocket.mount("/", routes![debug_endpoint])
    })
}
