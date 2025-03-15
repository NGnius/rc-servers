mod account_json;
pub use account_json::{AccountProvider, AccountInfo};

mod garage_data;
pub use garage_data::SelectedGarage;

mod initial_data;
pub use initial_data::setup_directory;

mod inventory;
pub use inventory::UnlockedParts;

mod traits;
pub use traits::{UserProvider, User, UserToken, UserSlots, UserSlotData, VehicleData};

pub const USERS_DIR: &str = "accounts";
pub const USER_FILE: &str = "user.json";
pub const GARAGE_DIR: &str = "vehicles";

pub type UserImpl = AccountProvider;

fn __must_impl<T: UserProvider<()>>() {}

fn __test_impl() {
    __must_impl::<UserImpl>();
}
