use actix_web::{web::{Data, Json}, Error, post};
use serde::{Deserialize, Serialize};

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
    // url and data should be mutually exclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>, // open browser to URL
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>, // change price to new price
}

type ItemResponse = Response<TokenResponse>;

#[post("/robopay/token")]
pub async fn robopay_token(_cli: Data<crate::cli::CliArgs>, body: Json<TokenRequest>) -> Result<Json<ItemResponse>, Error> {
    // TODO authentication (Authorization header is "Robocraft {JWT token from auth}")
    log::debug!("robopay token post body: {:?}", body);
    Ok(Json(Response {
        response: TokenResponse {
            url: Some("https://cncycle.cheofoundation.com/".to_owned()),
            data: None,
        }
    }))
}
