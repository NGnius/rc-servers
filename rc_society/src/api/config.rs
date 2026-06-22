use actix_web::{HttpResponse, Responder, get, web::Query, http::header::{ContentDisposition, ContentType}};

static JSON_DATA: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub(super) fn init(config: &dyn oj_rc_core::ConfigProvider<()>) {
    JSON_DATA.get_or_init(|| config.redacted_json());
}

#[derive(serde::Deserialize)]
struct ConfigQuery {
    #[serde(default)]
    pub download: bool,
}

#[get("/api/v1/config.json")]
pub async fn get(query: Query<ConfigQuery>) -> impl Responder {
    let data = JSON_DATA.get().expect("Config JSON init failure").to_owned();
    if query.download {
        HttpResponse::Ok()
            .insert_header(ContentDisposition::attachment("config.json"))
            .insert_header(ContentType::json())
            .body(data)
    } else {
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(data)
    }
}
