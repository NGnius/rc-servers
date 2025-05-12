mod debug;
mod username;
mod steam;
mod email;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("robocraft", |rocket| async {
        rocket.attach(username::stage())
            .attach(email::stage())
            .attach(steam::stage())
            .attach(debug::stage())
            .register("/", rocket::catchers![unauthorized])
    })
}

#[allow(dead_code)]
pub struct RcConfig {
    pub root: std::path::PathBuf,
    pub account_provider: rc_core::UserImpl,
}

impl RcConfig {
    pub async fn from_args(args: &crate::common::cli::CliArgs) -> Self {
        let conf = rc_core::persist::config::ConfigImpl::load(&args.assets_robocraft).expect("Bad config data");
        Self {
            account_provider: rc_core::UserImpl::load(&args.data_robocraft, &conf).await.expect("Invalid Robocraft user data"),
            root: args.data_robocraft.clone().into(),
        }
    }
}

use rocket::{catch, serde::json::Json};

#[catch(401)]
fn unauthorized() -> Json<libfj::robocraft::ErrorInfo> {
    Json(libfj::robocraft::ErrorInfo {
        error_code: "204".to_owned(),
        error_message: "Invalid username, password, or SteamID".to_owned(),
    })
}
