pub mod live_data;
pub mod user_avatar;
pub mod clan_avatar;
pub mod brawl_data;
pub mod campaign_data;
pub mod factory;
pub mod favicon;
mod internal_auth;
pub use internal_auth::{IntercomAuth, IntercomOpError};

pub(self) const DEFAULT_IMAGE: &str = "default.jpg";
