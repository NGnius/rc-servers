mod store;
pub use store::robopay_store;

mod token;
pub use token::robopay_token;

use serde::Serialize;

#[derive(Serialize)]
struct Response<T> {
    pub response: T
}

#[derive(Serialize)]
struct ResponseData<T> {
    pub data: Vec<T>
}
