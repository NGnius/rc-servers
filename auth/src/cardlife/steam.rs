use std::sync::RwLock;
use std::collections::HashMap;

use rocket::{post, routes, serde::json::Json};

// FIXME don't have global state
static AUTH_MAP: RwLock<Option<HashMap<String, libfj::cardlife::AuthenticationInfo>>> = RwLock::new(None);

fn generate_auth(ticket: &str, addr: &str) -> libfj::cardlife::AuthenticationInfo {
    let public_guid = uuid::Uuid::new_v4().to_string();
    log::warn!("assigning GUID {} to IP address {} (steam ticket {})", public_guid, addr, ticket);
    libfj::cardlife::AuthenticationInfo {
        public_id: public_guid,
        email_address: "nobody@openjamgames.com".to_string(),
        display_name: "gaben".to_string(),
        purchases: vec![1, 2, 3],
        flags: vec![],
        /*flags: //vec![
            "Dev".to_string(),
            "GiveInv".to_string(),
            "NoDrop".to_string(),
            "DekStruct".to_string(),
            "BucketA".to_string(),
            "BucketB".to_string(),
            "userlogged".to_string(),
        ],*/
        confirmed: true,
        token: "1234567890-qwertyuiop-asdfghjkl-zxcvbnm".to_string(),
        steam_id: Some("gaben".to_string()),
        id: 123456, // FIXME
    }
}

#[post("/api/auth/steamauthenticate", data = "<body>")]
pub fn steam_auth(addr: &rocket_client_addr::ClientAddr, body: Json<libfj::cardlife::SteamAuthenticationPayload>) -> Json<libfj::cardlife::AuthenticationInfo> {
    let addr_str = addr.get_ipv4_string().unwrap_or_else(|| addr.get_ipv6_string());
    if AUTH_MAP.read().unwrap().is_none() {
        let new_info = generate_auth(&body.steam_ticket, &addr_str);
        let mut new_map = HashMap::new();
        new_map.insert(addr_str, new_info.clone());
        *AUTH_MAP.write().unwrap() = Some(new_map);
        Json(new_info)
    } else {
        if let Some(auth_info) = AUTH_MAP.read().unwrap().as_ref().unwrap().get(&addr_str).map(|x| x.to_owned()) {
            Json(auth_info)
        } else {
            let new_info = generate_auth(&body.steam_ticket, &addr_str);
            AUTH_MAP.write().unwrap().as_mut().unwrap().insert(addr_str, new_info.clone());
            Json(new_info)
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CardLife Steam", |rocket| async {
        rocket.mount("/", routes![steam_auth])
    })
}
