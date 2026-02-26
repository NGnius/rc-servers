use actix_web::{web::{Data, Json}, Error, post};
use serde::{Deserialize, Serialize};
use sha2::Digest;

use super::Response;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TokenRequest {
    #[serde(rename = "Steam")]
    steam: bool,
    #[serde(rename = "SteamId")]
    steam_id: serde_json::Value, // empty string when client not running through steam, number (SteamId64) otherwise
    #[serde(rename = "Language")]
    language: String,
    #[serde(rename = "ItemSku")]
    item_sku: String,
    #[serde(rename = "ItemPrice")]
    item_price: f32,
    #[serde(rename = "ItemCurrency")]
    item_currency: String,
    #[serde(rename = "AnalyticsId")]
    analytics_id: String,
    #[serde(rename = "Country")]
    country: Option<String>,
}

#[derive(Serialize)]
struct TokenResponse {
    // TODO url and data should be mutually exclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>, // open browser to URL
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>, // change price to new price
}

type ItemResponse = Response<TokenResponse>;

#[post("/robopay/token")]
pub async fn robopay_token(_cli: Data<crate::cli::CliArgs>, _body: Json<TokenRequest>, auth: super::PaymentToken) -> Result<Json<ItemResponse>, Error> {
    //log::debug!("robopay token post body: {:?}", body);
    //log::debug!("robopay token post token: {:?}", auth.token);
    //log::debug!("robopay token post user: {:?}", auth.data.public_id);
    let hashed_token = sha2::Sha512::digest(auth.token.as_bytes());
    let hex_token = hex::encode(&hashed_token);
    let tagged_url = format!("https://cheofoundation.donordrive.com/participants/64767?referrer=openjam&token={}", hex_token);
    Ok(Json(Response {
        response: TokenResponse {
            url: Some(tagged_url),
            data: None,
        }
    }))
}
