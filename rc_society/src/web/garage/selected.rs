use actix_web::{get, web::{Data, Redirect}, Responder, HttpRequest};
use actix_identity::Identity;

use crate::web::{LoginReturn, try_auth_user};

#[get("/garages/selected")]
pub async fn get(auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match try_auth_user(user_opt, auth.as_ref(), &req).await? {
        LoginReturn::AuthFail(resp) => Ok(resp),
        LoginReturn::Success(user) => {
            let resp = if let Some(selected_garage) = user.garage_id_selected().await? {
                Redirect::to(format!("/garages/{}/info", selected_garage))
                    .respond_to(&req)
                    .map_into_boxed_body()

            } else {
                log::error!("User {} has no garage selected (bad database state?)", user.display_name());
                Redirect::to("/garages/list".to_owned())
                    .respond_to(&req)
                    .map_into_boxed_body()
            };
            Ok(
                resp
                    .respond_to(&req)
                    .map_into_boxed_body()
            )
        }
    }
}
