use rocket::{post, routes, serde::json::Json, http::Status};

#[post("/api/auth/steamauthenticate", data = "<body>")]
pub fn steam_auth(body: Json<libfj::cardlife::SteamAuthenticationPayload>) -> Result<Json<libfj::cardlife::AuthenticationInfo>, Status> {
    log::debug!("steam ticket: {}", body.steam_ticket);
    let steam_id = crate::common::steam_utils::authenticate_steam_ticket(&body.steam_ticket)
        .map_err(|_| Status { code: 400 })?;
    log::debug!("Found steamId {}", steam_id);
    Ok(Json(libfj::cardlife::AuthenticationInfo {
        public_id: uuid::Uuid::from_u64_pair(steam_id, steam_id).to_string(),
        email_address: "nobody@openjamgames.com".to_string(),
        display_name: steam_id.to_string(),
        purchases: vec![1, 2, 3],
        flags: vec![],
        /*flags: //vec![
            "Dev".to_string(),
            "GiveInv".to_string(),
            "NoDrop".to_string(),
            "DelStruct".to_string(),
            "BucketA".to_string(),
            "BucketB".to_string(),
            "userlogged".to_string(),
        ],*/
        confirmed: true,
        token: uuid::Uuid::from_u64_pair(steam_id, steam_id).to_string(),
        steam_id: Some(steam_id.to_string()),
        id: (steam_id & (i32::MAX as u64)) as i32,
    }))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CardLife Steam", |rocket| async {
        rocket.mount("/", routes![steam_auth])
    })
}
