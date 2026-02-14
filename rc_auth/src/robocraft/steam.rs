use oj_rc_core::UserAuthenticator;
use actix_web::{post, web::{Data, Json}};

fn authenticate_steam_ticket(hex_ticket: &str) -> Result<u64, ()> {
    get_steam_id_from_ticket_hex(hex_ticket)
        .map_err(|e| {
            log::error!("Failed to parse steamId: {}", e);
            
        })
}

fn get_steam_id_from_ticket_hex(hex_ticket: &str) -> Result<u64, hex::FromHexError> {
    let decoded_ticket = hex::decode(hex_ticket)?;
    if decoded_ticket.len() < 72 {
        Err(hex::FromHexError::InvalidStringLength)
    } else {
        Ok(get_steam_id_from_ticket(&decoded_ticket))
    }
}

fn get_steam_id_from_ticket(ticket: &[u8]) -> u64 {
    get_u64_with_offset(ticket, 12 /* also at 64 ??? */) // should be 76600000000000000 > number > 76500000000000000
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

#[post("/authenticate/steam/game")]
pub async fn steam_auth(body: Json<libfj::robocraft::SteamAuthenticationPayload>, config: Data<super::RcConfig>) -> Result<Json<libfj::robocraft::AuthenticationResponseInfo>, super::ErrorTy> {
    let steam_id = authenticate_steam_ticket(&body.steam_ticket)
        .map_err(|_| super::ErrorTy::from_err(oj_rc_core::persist::user::AuthError {
            message: "Bad SteamId".to_owned(),
            code: oj_rc_core::data::error_codes::AuthErrorCode::BadCredentials,
        }))?;
    log::info!("Authenticating {} steam user {}", body.target, steam_id);
    let user_info = oj_rc_core::persist::user::UserAuthInfo::Steam { id: steam_id };
    let response = config.account_provider.login(user_info).await
        .map_err(|e| {
            log::error!("Failed to authenticate {} steam user {}: {}", body.target, steam_id, e.message);
            super::ErrorTy::from_err(e)
        })?;
    Ok(Json(response.response))
}
