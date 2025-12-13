use std::io::Read;

use actix_web::{get, web::{Data, Path}, HttpResponse};

static ZIP_FILE: std::sync::Mutex<Option<zip::read::ZipArchive<std::io::BufReader<std::fs::File>>>> = std::sync::Mutex::new(None);

#[get("/roboshop/arc/Live/{id}")]
pub async fn get(cli: Data<crate::cli::CliArgs>, id: Path<u32>) -> HttpResponse {
    let id: u32 = *id;
    let zip_path = std::path::PathBuf::from(&cli.data_robocraft).join("rc_archive_thumbnails.zip");
    let thumb_dir_path = std::path::PathBuf::from(&cli.data_robocraft).join(super::THUMBNAIL_DIR);
    try_find_file(zip_path, thumb_dir_path, id).await
}

async fn try_find_file(zip_path: std::path::PathBuf, thumb_dir: std::path::PathBuf, id: u32) -> HttpResponse {
    let result = tokio::task::spawn_blocking(move || get_file_in_zip(zip_path, id)).await.unwrap();
    match result {
        Ok(bytes) => {
            log::debug!("Found id {} in factory arc", id);
            actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::OK)
                .append_header(("Content-Type", "image/jpeg"))
                .body(bytes)
        },
        Err(zip::result::ZipError::FileNotFound) => {
            let result = tokio::task::spawn_blocking(move || get_file_in_thumbnails(thumb_dir, id)).await.unwrap();
            match result {
                Ok(bytes) => {
                    log::debug!("Found id {} in thumbnails dir", id);
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::OK)
                        .append_header(("Content-Type", "image/jpeg"))
                        .body(bytes)
                },
                Err(_) => actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::NOT_FOUND)
                    .body("file not found in zip archive or thumbnails dir".to_string()),
            }
        },
        Err(e) => {
            log::debug!("Failed to find id {} in factory arc: {}", id, e);
            match e {
                zip::result::ZipError::Io(e) => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("zip io error: {}", e))
                },
                zip::result::ZipError::InvalidArchive(e) => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("invalid zip file: {}", e))
                },
                zip::result::ZipError::UnsupportedArchive(e) => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("unsupported zip file: {}", e))
                },
                zip::result::ZipError::InvalidPassword => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body("invalid zip password".to_string())
                },
                _ => actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body("unknown zip error".to_string()),
            }
        }
    }
}

fn get_file_in_zip(zip_path: std::path::PathBuf, id: u32) -> zip::result::ZipResult<Vec<u8>> {
    let prefix = format!("rc_archive_thumbnails/{} - ", id);
    let mut lock = ZIP_FILE.lock().unwrap();
    if let Some(archive) = lock.as_mut() {
        Ok(read_file_with_prefix(&prefix, archive)?)
    } else {
        // this is usually very slow, but thankfully should only run the first time this endpoint is hit
        let file = std::fs::File::open(zip_path)?;
        let buf = std::io::BufReader::new(file);
        let mut archive = zip::read::ZipArchive::new(buf)?;
        let bytes = read_file_with_prefix(&prefix, &mut archive)?;
        *lock = Some(archive);
        Ok(bytes)
    }


}

fn get_file_in_thumbnails(dir: std::path::PathBuf, id: u32) -> std::io::Result<Vec<u8>> {
    // in case someone has extracted the thumbnail zip
    // (newly-uploaded vehicles use the general thumbnail CDN endpoint)
    let prefix = format!("{} - ", id);
    for ent in std::fs::read_dir(&dir)? {
        let ent = ent?;
        let name = ent.file_name().to_string_lossy().into_owned();
        if name.starts_with(&prefix) && name.ends_with(".jpg") {
            return std::fs::read(ent.path());
        }
    }

    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "thumbnail not found"))
}

fn read_file_with_prefix(prefix: &str, archive: &mut zip::read::ZipArchive<std::io::BufReader<std::fs::File>>) -> zip::result::ZipResult<Vec<u8>> {
    let index = if let Some((index, _)) = archive.file_names().enumerate().find(|(_, name)| name.starts_with(prefix)) {
        index
    } else {
        return Err(zip::result::ZipError::FileNotFound);
    };
    let mut buf = Vec::new();
    let mut file = archive.by_index(index)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)

}
