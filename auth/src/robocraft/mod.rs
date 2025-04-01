mod debug;
mod email;
mod steam;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("robocraft", |rocket| async {
        rocket.attach(email::stage())
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
    pub fn from_args(args: &crate::common::cli::CliArgs) -> Self {
        Self {
            account_provider: rc_core::UserImpl::load_for_auth(&args.data_robocraft).expect("Invalid Robocraft user data"),
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
