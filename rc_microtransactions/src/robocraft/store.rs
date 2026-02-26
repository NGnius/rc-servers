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
pub async fn robopay_store(_cli: Data<crate::cli::CliArgs>, _auth: super::PaymentToken) -> Result<Json<ItemResponse>, Error> {
    // TODO authentication (Authorization header is "Robocraft {JWT token from auth}")
    Ok(Json(Response {
        response: ResponseData {
            data: vec![
                // Robopass
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::RoboPass,
                            amount: 1,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "RoboPass".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "I wish :(".to_owned(),
                    old_currency_string: "Robopass :(".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                // Premium
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::PremiumForLife,
                            amount: 1,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "PremiumPackLife".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "15.99/month".to_owned(),
                    old_currency_string: "Line go up".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: true,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::Premium,
                            amount: 1,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "PremiumPack1".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "0.99".to_owned(),
                    old_currency_string: "".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::Premium,
                            amount: 3,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "PremiumPack2".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "1.99".to_owned(),
                    old_currency_string: "2.99".to_owned(),
                    price_for_check: 1.99,
                    old_price_for_check: 2.99,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::Premium,
                            amount: 7,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "PremiumPack3".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "4.99".to_owned(),
                    old_currency_string: "6.7".to_owned(),
                    price_for_check: 4.99,
                    old_price_for_check: 6.7,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                // Cosmic credit
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::CosmeticCredits,
                            amount: 100,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "CosmeticCredits1".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "$0.99".to_owned(),
                    old_currency_string: "Macrotransactions".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::CosmeticCredits,
                            amount: 500,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "CosmeticCredits2".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "Give me money".to_owned(),
                    old_currency_string: "Open source".to_owned(),
                    price_for_check: 42.99,
                    old_price_for_check: 12.99,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::CosmeticCredits,
                            amount: 1_000,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "CosmeticCredits3".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "Capitalism".to_owned(),
                    old_currency_string: "Communism".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: true,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::CosmeticCredits,
                            amount: 2_000,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "CosmeticCredits4".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "$42.99".to_owned(),
                    old_currency_string: "Math".to_owned(),
                    price_for_check: 42.99,
                    old_price_for_check: 0.1,
                    additional_value: 0,
                    most_popular: false,
                    best_value: true,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                // Robits
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::Robits,
                            amount: 1,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "RobitsBundle1".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "A few bills".to_owned(),
                    old_currency_string: "Saving".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::Robits,
                            amount: 1,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "RobitsBundle2".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "Some bills".to_owned(),
                    old_currency_string: "Green redstone".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::Robits,
                            amount: 1,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "RobitsBundle3".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "Bill Clinton".to_owned(),
                    old_currency_string: "Green cocaine".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: false,
                    best_value: false,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
                StoreBundle {
                    items: vec![
                        StoreItem {
                            item_type: StoreItemType::Robits,
                            amount: 1,
                            data: "RE_store_item_data_01".to_owned(),
                        },
                    ],
                    item_sku: "RobitsBundle4".to_owned(), // SKUs can be found in RealMoneyStoreExtraData of sharedassets2.assets
                    currency_code: "RE_no_currency".to_owned(),
                    currency_string: "All them bills".to_owned(),
                    old_currency_string: "Ribbits".to_owned(),
                    price_for_check: 1.0,
                    old_price_for_check: 1.0,
                    additional_value: 0,
                    most_popular: false,
                    best_value: true,
                    currency_type: "RE_price_type_01".to_owned(),
                    is_available: true,
                    is_owned: false,
                },
            ]
        }
    }))
}
