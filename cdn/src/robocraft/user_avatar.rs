use actix_web::{get, web::{Data, Path}, Responder};

#[get("/customavatar/Live/{name}")]
pub async fn get(cli: Data<crate::cli::CliArgs>, name: Path<String>) -> impl Responder {
    let path = std::path::PathBuf::from(&cli.data_robocraft).join("customavatars").join(format!("{}.jpg", *name));
    log::debug!("RC asset at {} (exists? {})", path.display(), path.exists());
    actix_files::NamedFile::open_async(path).await
}
