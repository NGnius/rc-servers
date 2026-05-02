use actix_web::{get, post, web::{Data, Form, Redirect}, Responder, HttpRequest, HttpMessage};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

const FORM_NAME: &str = "login";

#[derive(Serialize, Deserialize, Default)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn form_submit(form: Form<LoginForm>, auth: Data<Box<oj_rc_core::UserImpl>>, handlebars_ref: Data<handlebars::Handlebars<'_>>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    use oj_rc_core::UserAuthenticator;
    let auth_result = auth.login(oj_rc_core::persist::user::UserAuthInfo::Username {
        username: form.username.to_owned(),
        password: form.password.to_owned(),
    }).await;
    match auth_result {
        Ok(user) => {
            let resp = Redirect::to("/dashboard")
                .respond_to(&req)
                .map_into_boxed_body();
            Identity::login(&req.extensions(), user.response.token)?;
            Ok(resp)
        },
        Err(e) => {
            Ok(super::render_err(form.0, e.message, handlebars_ref.as_ref(), FORM_NAME)
                .respond_to(&req)
                .map_into_boxed_body())
        }
    }
}

#[get("/login")]
pub async fn form_load(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    if let Some(user) = user_opt {
        let user_id = user.id()?;
        use oj_rc_core::UserAuthenticator;
        if auth.verify(user_id).await.is_ok() {
            Ok(Redirect::to("/dashboard")
                .respond_to(&req)
                .map_into_boxed_body())
        } else {
            Ok(super::render_err(
                LoginForm::default(),
                "Invalid login token".to_owned(),
                handlebars_ref.as_ref(),
                FORM_NAME,
            )
                .respond_to(&req)
                .map_into_boxed_body()
            )
        }
    } else {
        Ok(super::render_ok(
            LoginForm::default(),
            handlebars_ref.as_ref(),
            FORM_NAME,
        )
            .respond_to(&req)
            .map_into_boxed_body()
        )
    }
}
