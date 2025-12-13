use actix_web::{get, post, web::{Data, Path, Bytes}, Responder};

#[get("/roboshop/Live/{id}")]
pub async fn get(cli: Data<crate::cli::CliArgs>, id: Path<u32>) -> impl Responder {
    let path = std::path::PathBuf::from(&cli.data_robocraft).join(super::THUMBNAIL_DIR).join(format!("{}.jpg", id));
    log::debug!("RC asset at {} (exists? {})", path.display(), path.exists());
    if path.exists() {
        actix_files::NamedFile::open_async(path).await
    } else {
        log::info!("Not found /roboshop/Live/{} -> {}, using default image", id, path.display());
        actix_files::NamedFile::open_async(std::path::PathBuf::from(&cli.assets_robocraft).join(super::super::DEFAULT_IMAGE)).await
    }
}

#[post("/roboshop/Live/{id}")]
pub async fn post(cli: Data<crate::cli::CliArgs>, auth: Data<crate::robocraft::IntercomAuth>, id: Path<i32>, body: Bytes, req: actix_web::HttpRequest) -> Result<actix_web::HttpResponse, crate::robocraft::IntercomOpError> {
    auth.validate(&req, &id.to_string())?;
    let path = std::path::PathBuf::from(&cli.data_robocraft).join(super::THUMBNAIL_DIR).join(format!("{}.jpg", *id));
    log::debug!("Saving factory thumbnail for {} to {}: {}B", id, path.display(), body.len());
    std::fs::write(path, &body).map_err(crate::robocraft::IntercomOpError::Io)?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}
