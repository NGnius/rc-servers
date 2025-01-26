use rocket::{post, routes, serde::json::Json, http::Status};

#[cfg(feature = "steam")]
static STEAM_SERVER: std::sync::OnceLock<steamworks::Server> = OnceLock::new();

#[cfg(feature = "steam")]
fn init_steam() -> steamworks::Server {
    let (server, single_client) = steamworks::Server::init(core::net::Ipv4Addr::new(127, 0, 0, 1), 9000, 9001, steamworks::ServerMode::NoAuthentication, "")
        .expect("Steam is unavailable");
    std::thread::spawn(move || {
        loop {
            single_client.run_callbacks();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
    server.set_product("920690");
    server.log_on_anonymous();
    server
}

fn get_u64_with_offset(arr: &[u8], start: usize) -> u64 {
    let arr_actual: [u8; 8] = [
        arr[start],
        arr[start+1],
        arr[start+2],
        arr[start+3],
        arr[start+4],
        arr[start+5],
        arr[start+6],
        arr[start+7],
    ];
    u64::from_le_bytes(arr_actual)
}

#[post("/api/auth/steamauthenticate", data = "<body>")]
pub fn steam_auth(body: Json<libfj::cardlife::SteamAuthenticationPayload>) -> Result<Json<libfj::cardlife::AuthenticationInfo>, Status> {
    log::debug!("steam ticket: {}", body.steam_ticket);
    match hex::decode(&body.steam_ticket) {
        Ok(ticket) => {
            if ticket.len() < 72 {
                return Err(Status { code: 400 })
            }
            let steam_id = get_u64_with_offset(&ticket, 12 /* also at 64 ??? */); // should be 76600000000000000 > number > 76500000000000000
            log::debug!("Found steamId {}", steam_id);
            #[cfg(feature = "steam")]
            {
                let steam = STEAM_SERVER.get_or_init(init_steam);
                if let Err(e) = steam.begin_authentication_session(steamworks::SteamId::from_raw(steam_id), &ticket) {
                    log::error!("steam server auth session error: {}", e);
                    return Err(Status { code: 400 })
                }
            }
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
                    "DekStruct".to_string(),
                    "BucketA".to_string(),
                    "BucketB".to_string(),
                    "userlogged".to_string(),
                ],*/
                confirmed: true,
                token: uuid::Uuid::from_u64_pair(steam_id, steam_id).to_string(),
                steam_id: Some(steam_id.to_string()),
                id: (steam_id & (i32::MAX as u64)) as i32,
            }))
        },
        Err(e) => {
            log::error!("request with bad steam ticket: {}", e);
            Err(Status { code: 400 })
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    #[cfg(feature = "steam")]
    STEAM_SERVER.get_or_init(init_steam);
    rocket::fairing::AdHoc::on_ignite("CardLife Steam", |rocket| async {
        rocket.mount("/", routes![steam_auth])
    })
}
