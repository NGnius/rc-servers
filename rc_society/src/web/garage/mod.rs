pub mod list;
pub mod export;
pub mod import;
pub mod info;
pub mod selected;

#[derive(serde::Deserialize)]
struct PortQuery {
    pub plugin: String,
}

#[derive(Debug)]
struct PluginPortError {
    code: oj_rc_plugins::vehicle_import::VehicleImportErrorCode,
}

impl std::convert::From<oj_rc_plugins::vehicle_import::VehicleImportErrorCode> for PluginPortError {
    fn from(value: oj_rc_plugins::vehicle_import::VehicleImportErrorCode) -> Self {
        Self {
            code: value,
        }
    }
}

impl core::fmt::Display for PluginPortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl core::error::Error for PluginPortError {}

impl actix_web::error::ResponseError for PluginPortError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNPROCESSABLE_ENTITY
    }
}
