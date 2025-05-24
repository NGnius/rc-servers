use actix_web::{dev::ResourcePath, get, web::{Data, Path}, Responder};

#[get("/clanavatar/Live/{name}")]
pub async fn get(cli: Data<crate::cli::CliArgs>, name: Path<String>) -> impl Responder {
    let path = std::path::PathBuf::from(&cli.data_robocraft).join("clanavatar").join(format!("{}.jpg", name.path()));
    log::debug!("RC asset at {} (exists? {})", path.display(), path.exists());
    actix_files::NamedFile::open_async(path).await
}
