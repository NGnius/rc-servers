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

#[allow(dead_code)]
fn get_steam_id_from_ticket_hex(hex_ticket: &str) -> Result<u64, hex::FromHexError> {
    let decoded_ticket = hex::decode(hex_ticket)?;
    if decoded_ticket.len() < 72 {
        Err(hex::FromHexError::InvalidStringLength)
    } else {
        Ok(get_steam_id_from_ticket(&decoded_ticket))
    }
}

#[allow(dead_code)]
fn get_steam_id_from_ticket(ticket: &[u8]) -> u64 {
    get_u64_with_offset(ticket, 12 /* also at 64 ??? */) // should be 76600000000000000 > number > 76500000000000000
}

#[cfg(all(feature = "steam", feature = "cardlife"))]
const STEAM_ID: &str = "920690"; // Cardlife steam app id

#[cfg(feature = "steam")]
static STEAM_SERVER: std::sync::OnceLock<steamworks::Server> = std::sync::OnceLock::new();

#[cfg(feature = "steam")]
fn init_steam() -> steamworks::Server {
    if let Err(e) = std::fs::write::<&str, &[u8]>("./steam_appid.txt", STEAM_ID.as_ref()) {
        log::error!("Failed to write appId to steam_appid.txt: {}", e);
    }
    let (server, single_client) = steamworks::Server::init(core::net::Ipv4Addr::new(127, 0, 0, 1), 9000, 9001, steamworks::ServerMode::NoAuthentication, "")
        .expect("Steam is unavailable");
    std::thread::spawn(move || {
        loop {
            single_client.run_callbacks();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
    server.set_product(STEAM_ID);
    server.log_on_anonymous();
    server
}

#[cfg(feature = "steam")]
fn get_steam() -> &'static steamworks::Server {
    STEAM_SERVER.get_or_init(init_steam)
}

#[cfg(not(feature = "steam"))]
pub fn authenticate_steam_ticket(hex_ticket: &str) -> Result<u64, ()> {
    get_steam_id_from_ticket_hex(hex_ticket)
        .map_err(|e| {
            log::error!("Failed to parse steamId: {}", e);
            
        })
}

#[cfg(feature = "steam")]
pub fn authenticate_steam_ticket(hex_ticket: &str) -> Result<u64, ()> {
    let decoded_ticket = hex::decode(hex_ticket)
        .map_err(|e| {
            log::error!("Failed to decode hexadecimal steam ticket: {}", e);
            ()
        })?;
    if decoded_ticket.len() < 72 {
        log::error!("Failed to parse steamId: ticket too short");
        return Err(())
    }
    let steam_id = get_steam_id_from_ticket(&decoded_ticket);
    let steam = get_steam();
    steam.begin_authentication_session(steamworks::SteamId::from_raw(steam_id), &decoded_ticket)
        .map_err(|e| {
            log::error!("steam server auth session error: {}", e);
            ()
        })?;
    Ok(steam_id)
}
