use actix_web::{get, post, web::{Data, Path, Bytes}, Responder};

#[get("/clanavatar/Live/{name}")]
pub async fn get(cli: Data<crate::cli::CliArgs>, name: Path<String>) -> impl Responder {
    let path = std::path::PathBuf::from(&cli.data_robocraft).join("clanavatar").join(format!("{}.jpg", *name));
    log::debug!("RC asset at {} (exists? {})", path.display(), path.exists());
    if path.exists() {
        actix_files::NamedFile::open_async(path).await
    } else {
        log::info!("Not found /clanavatar/Live/{} -> {}, using default image", name, path.display());
        actix_files::NamedFile::open_async(std::path::PathBuf::from(&cli.assets_robocraft).join(super::DEFAULT_IMAGE)).await
    }
}

#[post("/clanavatar/Live/{name}")]
pub async fn post(cli: Data<crate::cli::CliArgs>, auth: Data<crate::robocraft::IntercomAuth>, name: Path<String>, body: Bytes, req: actix_web::HttpRequest) -> Result<actix_web::HttpResponse, super::IntercomOpError> {
    auth.validate(&req, &name)?;
    let path = std::path::PathBuf::from(&cli.data_robocraft).join("clanavatar").join(format!("{}.jpg", *name));
    log::debug!("Saving clanavatar for {} to {}: {}B", name, path.display(), body.len());
    std::fs::write(path, &body).map_err(super::IntercomOpError::Io)?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}
