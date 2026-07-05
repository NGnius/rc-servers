use actix_web::{get, web::Json};
use openidconnect::core::{CoreJsonWebKeySet, /*CoreJsonWebKey*/};

#[get("/authenticate/oauth2/jwks")]
pub async fn get_oauth_jwks() -> Json<CoreJsonWebKeySet> {
    Json(CoreJsonWebKeySet::new(vec![
        // TODO ???
    ]))
}
