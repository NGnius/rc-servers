use rocket::{post, routes, serde::json::Json, http::Status};

fn generate_token(user_auth: &libfj::robocraft::EmailUserAuthenticationPayload) -> String {
    let header = jsonwebtoken::Header {
        typ: Some("JWT".to_string()),
        alg: jsonwebtoken::Algorithm::HS256,
        ..Default::default()
    };
    let payload = libfj::robocraft::TokenPayload {
        public_id: user_auth.display_name.to_owned(),
        display_name: user_auth.display_name.to_owned(),
        robocraft_name: user_auth.display_name.to_owned(),
        email_address: user_auth.email_address.to_owned(),
        email_verified: true,
        flags: Vec::new(),
    };
    let secret = jsonwebtoken::EncodingKey::from_secret(user_auth.password.as_bytes()); // FIXME use an actually secret secret
    jsonwebtoken::encode(&header, &payload, &secret)
        .unwrap_or_else(|e| {
            log::error!("Failed to encode JWT: {}", e);
            libfj::robocraft::DEFAULT_TOKEN.to_owned()
        })
}

#[post("/authenticate/robocraft/game", data = "<body>")]
pub fn email_password_auth(body: Json<libfj::robocraft::EmailUserAuthenticationPayload>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, Status> {
    log::info!("Authenticating {} user {}", body.target, body.display_name);
    Ok(Json(libfj::robocraft::AuthenticationResponseInfo {
        token: generate_token(&body),
        refresh_token: "qwertyuiop".to_string(), // TODO
        refresh_token_expiry: "0".to_string(), // TODO (seems like this isn't actually considered by the client)
    }))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Robocraft Email/Password", |rocket| async {
        rocket.mount("/", routes![email_password_auth])
    })
}
