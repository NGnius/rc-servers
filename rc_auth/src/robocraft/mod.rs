pub mod email;
pub mod registration;
pub mod steam;
pub mod username;

pub struct RcConfig {
    //pub data: std::path::PathBuf,
    pub account_provider: oj_rc_core::UserImpl,
    pub assets: std::path::PathBuf,
}

impl RcConfig {
    pub async fn from_args(args: &crate::cli::CliArgs) -> Self {
        let conf = oj_rc_core::persist::config::ConfigImpl::load(&args.assets_robocraft).expect("Bad config data");
        Self {
            account_provider: oj_rc_core::UserImpl::load(&args.data_robocraft, &conf).await.expect("Invalid Robocraft user data"),
            //data: args.data_robocraft.clone().into(),
            assets: args.assets_robocraft.clone().into(),
        }
    }
}

 struct ErrorTy {
    json: libfj::robocraft::ErrorInfo,
}

impl ErrorTy {
    pub fn from_err(error: oj_rc_core::persist::user::AuthError) -> Self {
        Self {
            json: libfj::robocraft::ErrorInfo {
                error_code: error.code.to_str(),
                error_message: error.message,
            },
        }
    }
}

impl actix_web::error::ResponseError for ErrorTy {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponse::with_body(
            self.status_code(),
            serde_json::to_string(&self.json).unwrap(),
        ).map_into_boxed_body()
    }
}

impl core::fmt::Display for ErrorTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //use core::fmt::Write;
        write!(f, "({}) {}", self.json.error_code, self.json.error_message)
    }
}

impl core::fmt::Debug for ErrorTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorTy")
            .finish_non_exhaustive()
    }
}
