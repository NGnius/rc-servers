use actix_web::{HttpResponse, Responder, get, http::header::ContentType};

static JSON_DATA: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub(super) fn init(config: &dyn oj_rc_core::ConfigProvider<()>) {
    JSON_DATA.get_or_init(|| {
        let urls = config.server_config();
        let data = oj_serdes::society::ServiceDomains {
            root: urls.domain,
            auth: urls.auth_url,
            cdn: urls.cdn_url,
            factory: urls.factory_url,
            society: urls.society_url,
        };
        serde_json::to_string(&data).expect("ServiceDomains did not serialize")
    });
}

#[get("/api/v1/services.json")]
pub async fn get() -> impl Responder {
    let data = JSON_DATA.get().expect("ServiceDomains JSON init failure").to_owned();
    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(data)
}
