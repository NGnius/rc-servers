use std::io::Read;

use actix_web::{get, web::{Data, Path}, HttpResponse};

static ZIP_FILE: std::sync::Mutex<Option<zip::read::ZipArchive<std::io::BufReader<std::fs::File>>>> = std::sync::Mutex::new(None);

#[get("/roboshop/arc/Live/{id}")]
pub async fn get(cli: Data<crate::cli::CliArgs>, id: Path<u32>) -> HttpResponse {
    let id: u32 = *id;
    let zip_path = std::path::PathBuf::from(&cli.data_robocraft).join("rc_archive_thumbnails.zip");
    try_find_file(zip_path, id).await
}

async fn try_find_file(zip_path: std::path::PathBuf, id: u32) -> HttpResponse {
    let result = tokio::task::spawn_blocking(move || get_file_in_zip(zip_path, id)).await.unwrap();
    match result {
        Ok(bytes) => {
            log::debug!("Found id {} in factory arc", id);
            actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::OK)
                .append_header(("Content-Type", "image/jpeg"))
                .body(bytes)
        },
        Err(e) => {
            log::debug!("Failed to find id {} in factory arc: {}", id, e);
            match e {
                zip::result::ZipError::Io(e) => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("zip io error: {}", e.to_string()))
                },
                zip::result::ZipError::InvalidArchive(e) => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("invalid zip file: {}", e.to_string()))
                },
                zip::result::ZipError::UnsupportedArchive(e) => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("unsupported zip file: {}", e.to_string()))
                },
                zip::result::ZipError::FileNotFound => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::NOT_FOUND)
                        .body(format!("file not found in zip archive"))
                },
                zip::result::ZipError::InvalidPassword => {
                    actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("invalid zip password"))
                },
                _ => actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("unknown zip error")),
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
