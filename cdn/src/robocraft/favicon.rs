use actix_web::{get, Responder, web::Data};

#[get("/robocraft/favicon")]
pub async fn get(cli: Data<crate::cli::CliArgs>) -> impl Responder {
    let path = std::path::PathBuf::from(&cli.assets_robocraft).join("favicon.jpg");
    actix_files::NamedFile::open_async(path).await
}
