use actix_web::{HttpRequest, HttpResponse, web::{Data, Path, Json}, Error, get, post};

#[get("/intercom/.status")]
pub async fn status_get(reg: Data<super::Users>) -> Json<oj_serdes::Status> {
    Json(oj_serdes::Status {
        servers: reg.statuses().await
    })
}

#[post("/intercom/.status/{name}/{service}")]
pub async fn status_set(req: HttpRequest, body: Json<oj_serdes::ServerStatus>, auth: Data<super::IntercomAuth>, reg: Data<super::Users>, uri: Path<(String, String)>) -> Result<HttpResponse, Error> {
    let name = &uri.0;
    let service = &uri.1;
    log::debug!("Got intercom status message from {}/{}", name, service);
    auth.validate(&req, &format!(".status/{}/{}", name, service))?;
    log::debug!("Authenticated intercom status message from {}/{}", name, service);
    reg.save_status(service.clone(), body.clone()).await;
    Ok(HttpResponse::NoContent().finish())
}
