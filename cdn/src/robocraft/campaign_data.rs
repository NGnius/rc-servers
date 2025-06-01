use actix_web::{get, web::{Data, Path}, Responder};

#[get("/campaigndata/Live/{name}")]
pub async fn get(cli: Data<crate::cli::CliArgs>, name: Path<String>) -> impl Responder {
    let path = std::path::PathBuf::from(&cli.data_robocraft).join("campaigndata").join(format!("{}.jpg", *name));
    log::debug!("RC asset at {} (exists? {})", path.display(), path.exists());
    if path.exists() {
        actix_files::NamedFile::open_async(path).await
    } else {
        log::info!("Not found /campaigndata/Live/{} -> {}, using default image", name, path.display());
        actix_files::NamedFile::open_async(std::path::PathBuf::from(&cli.assets_robocraft).join(super::DEFAULT_IMAGE)).await
    }
}
