mod account_json;
pub use account_json::AccountProvider;

mod garage_data;
pub use garage_data::SelectedGarage;

mod initial_data;
pub use initial_data::setup_new_user;

mod inventory;
pub use inventory::UnlockedParts;

mod traits;
pub use traits::{UserProvider, User, UserToken, UserSlots, UserSlotData, VehicleData, UserInfo, UserLoginInfo, ExtraUserInfo, UserAuthenticator};

pub const TOKEN_SECRET_FILENAME: &str = "token_secret.key";

pub const USERS_DIR: &str = "accounts";
pub const USER_FILE: &str = "user.json";
pub const GARAGE_DIR: &str = "vehicles";

pub type UserImpl = AccountProvider;

fn __must_impl<T: UserProvider<()>>() {}

fn __test_impl() {
    __must_impl::<UserImpl>();
}

pub fn since_windows_epoch(since_unix_epoch: i64) -> i64 {
    use chrono::TimeZone;
    let windows_epoch = chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::parse_from_str("1601-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());
    let time_in = chrono::DateTime::<chrono::Utc>::from_timestamp(since_unix_epoch, 0).unwrap();
    //let time_in = chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(since_unix_epoch, 0));
    time_in.signed_duration_since(windows_epoch).num_milliseconds() * 10_000
}

pub fn uuid_str(uuid: &(u32, u32)) -> String {
    format!("{}_{}", uuid.0, uuid.1)
}

pub fn i64_as_uuid_str(num: i64) -> String {
    uuid_str(&super::garage::i64_split(num))
}
