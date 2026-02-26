mod store;
pub use store::robopay_store;

mod token;
pub use token::robopay_token;

mod auth_jwt;
pub use auth_jwt::{PaymentToken, TokenSecret};

use serde::Serialize;

#[derive(Serialize)]
struct Response<T> {
    pub response: T
}

#[derive(Serialize)]
struct ResponseData<T> {
    pub data: Vec<T>
}
