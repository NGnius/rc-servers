use actix_web::{web::{Data, Json}, Error, post};
use serde::Serialize;

use super::{Response, ResponseData};

#[derive(Serialize)]
struct StoreBundle {
    #[serde(rename = "items")]
    items: Vec<StoreItem>,
    #[serde(rename = "itemSku")]
    item_sku: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
    #[serde(rename = "currencyString")]
    currency_string: String,
    #[serde(rename = "oldCurrencyString")]
    old_currency_string: String,
    #[serde(rename = "priceForCheck")]
    price_for_check: f32,
    #[serde(rename = "oldPriceForCheck")]
    old_price_for_check: f32,
    #[serde(rename = "additionalValue")]
    additional_value: i32,
    #[serde(rename = "mostPopular")]
    most_popular: bool,
    #[serde(rename = "bestValue")]
    best_value: bool,
    #[serde(rename = "currencyType")]
    currency_type: String,
    #[serde(rename = "available")]
    is_available: bool,
    #[serde(rename = "owned")]
    is_owned: bool,
}

#[derive(Serialize)]
struct StoreItem {
    #[serde(rename = "itemType")]
    item_type: StoreItemType,
    #[serde(rename = "amount")]
    amount: i32,
    #[serde(rename = "data")]
    data: String,
}

#[allow(dead_code)]
#[derive(Serialize)]
enum StoreItemType {
    RoboPass = 0,
    CosmeticCredits = 1,
    Premium = 2,
    PremiumForLife = 3,
    Cube = 4,
    Robits = 5,
    Other = 6
}

type ItemResponse = Response<ResponseData<StoreBundle>>;

#[post("/robopay/store")]
pub async fn robopay_store(_cli: Data<crate::cli::CliArgs>) -> Result<Json<ItemResponse>, Error> {
    // TODO authentication (Authorization header is "Robocraft {JWT token from auth}")
    Ok(Json(Response {
        response: ResponseData {
            data: vec![
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::CosmeticCredits,
                            amount: 1,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "CosmeticCredits1".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "RE_DO_NOT_BUY_01".to_owned(),
                    old_currency_string: "RE_DO_NOT_BUY_OLD_01".to_owned(),
                    price_for_check: 42.0,
                    old_price_for_check: 41.0,
                    additional_value: 999888777,
                    most_popular: true,
                    best_value: true,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
            ]
        }
    }))
}
