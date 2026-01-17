use actix_web::{get, web::Data, HttpResponse};
use handlebars::Handlebars;

const TEMPLATE_INDEX: &str = "rc_factory_web/index";
const TEMPLATE_APP_JS: &str = "rc_factory_web/app.js";

#[derive(serde::Serialize)]
struct Context {
    version: String,
    source_url: String,
}

fn version_string() -> String {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    //let license = env!("CARGO_PKG_LICENSE");
    //let repo = env!("CARGO_PKG_REPOSITORY");
    format!("OpenJam {} {}", name, version)
}

#[get("/")]
pub async fn index(hb: Data<Handlebars<'_>>) -> HttpResponse {
    let ctx = Context {
        version: version_string(),
        source_url: env!("CARGO_PKG_REPOSITORY").to_string(),
    };

    match hb.render(TEMPLATE_INDEX, &ctx) {
        Ok(body) => HttpResponse::Ok()
            .insert_header(("Content-Type", "text/html; charset=utf-8"))
            .body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("template render error: {e}")),
    }
}

#[get("/app.js")]
pub async fn app_js(hb: Data<Handlebars<'_>>) -> HttpResponse {
    match hb.render(TEMPLATE_APP_JS, &()) {
        Ok(body) => HttpResponse::Ok()
            .insert_header(("Content-Type", "application/javascript; charset=utf-8"))
            .body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("template render error: {e}")),
    }
}

async fn favicon_impl(assets_root: Data<std::path::PathBuf>) -> impl actix_web::Responder {
    let path = assets_root.join("favicon.jpg");
    actix_files::NamedFile::open_async(path).await
}

#[get("/robocraft/favicon")]
pub async fn favicon(assets_root: Data<std::path::PathBuf>) -> impl actix_web::Responder {
    favicon_impl(assets_root).await
}

#[get("/favicon.ico")]
pub async fn favicon_standard(assets_root: Data<std::path::PathBuf>) -> impl actix_web::Responder {
    favicon_impl(assets_root).await
}