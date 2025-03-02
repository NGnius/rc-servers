use rocket::{post, routes, serde::json::{json, Value}};

#[post("/robopay/store")]
pub fn robopay_store() -> Value {
    // TODO authentication (Authorization header is "Robocraft {JWT token from auth}")
    json!({
        "response": {
            "data": []
        }
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Robocraft store info", |rocket| async {
        rocket.mount("/", routes![robopay_store])
    })
}
