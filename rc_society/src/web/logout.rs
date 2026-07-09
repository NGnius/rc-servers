use actix_web::{get, web::Redirect, Responder, HttpRequest};
use actix_identity::Identity;

#[get("/logout")]
pub async fn get(user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    if let Some(user) = user_opt {
        user.logout();
    }
    Ok(Redirect::to("/")
        .respond_to(&req)
        .map_into_boxed_body())
}
