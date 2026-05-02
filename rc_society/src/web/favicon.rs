use actix_web::{get, web::Data, Responder};

async fn favicon_impl(cli_args: Data<crate::cli::LoadedArgs>) -> impl Responder {
    let path = std::path::PathBuf::from(&cli_args.assets).join("favicon.jpg");
    actix_files::NamedFile::open_async(path).await
}

#[get("/favicon.ico")]
pub async fn favicon_standard(cli_args: Data<crate::cli::LoadedArgs>) -> impl Responder {
    favicon_impl(cli_args).await
}
