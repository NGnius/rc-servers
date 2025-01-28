use rocket::{post, routes, serde::json::Json, http::Status};

fn generate_token(user_auth: &libfj::robocraft::SteamAuthenticationPayload, steam_id: u64) -> String {
    let header = jsonwebtoken::Header {
        typ: Some("JWT".to_string()),
        alg: jsonwebtoken::Algorithm::HS256,
        ..Default::default()
    };
    let payload = libfj::robocraft::TokenPayload {
        public_id: steam_id.to_string(),
        display_name: steam_id.to_string(),
        robocraft_name: steam_id.to_string(),
        email_address: format!("{}.rc.steam@ngni.us", steam_id),
        email_verified: true,
        flags: Vec::new(),
    };
    let secret = jsonwebtoken::EncodingKey::from_secret(user_auth.steam_ticket.as_ref()); // FIXME use an actually secret secret
    jsonwebtoken::encode(&header, &payload, &secret)
        .unwrap_or_else(|e| {
            log::error!("Failed to encode JWT: {}", e);
            libfj::robocraft::DEFAULT_TOKEN.to_owned()
        })
}

#[post("/authenticate/steam/game", data = "<body>")]
pub fn steam_auth(body: Json<libfj::robocraft::SteamAuthenticationPayload>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, Status> {
    let steam_id = crate::common::steam_utils::authenticate_steam_ticket(&body.steam_ticket)
        .map_err(|_| Status { code: 401 })?;
    log::info!("Authenticating {} steam user {}", body.target, steam_id);
    Ok(Json(libfj::robocraft::AuthenticationResponseInfo {
        token: generate_token(&body, steam_id),
        refresh_token: "qwertyuiop".to_string(), // TODO
        refresh_token_expiry: "0".to_string(), // TODO (seems like this isn't actually considered by the client)
    }))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Robocraft Steam", |rocket| async {
        rocket.mount("/", routes![steam_auth])
    })
}
