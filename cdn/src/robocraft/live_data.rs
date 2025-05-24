use actix_web::{get, Responder, web::Data};

#[get("/live/data.json")]
pub async fn live_data_json(cli: Data<crate::cli::CliArgs>) -> impl Responder {
    let path = std::path::PathBuf::from(&cli.assets_robocraft).join("live_data.json");
    actix_files::NamedFile::open_async(path).await
}
