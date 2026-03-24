use oj_rc_core::UserAuthenticator;
use actix_web::{get, post, web::{Data, Form, Html}, Responder};
//use rocket_dyn_templates::{Template, context};
use serde::{Serialize, Deserialize};

const FORM_NAME: &str = "rc_register";
const FORM_NAME_SUCCESS: &str = "rc_register_success";

const VALID_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '_',
];

#[derive(Serialize, Deserialize)]
struct RegisterForm {
    display_name: String,
    password: String,
    password_c: String,
    email: Option<String>,
    steam_id: Option<String>,
}

#[derive(Serialize)]
struct Context {
    form: RegisterForm,
    error: Option<String>,
    version: String,
    source_url: String,
    is_registration_allowed: bool,
}

#[derive(Serialize)]
struct ContextSuccess {
    display_name: String,
    id: i32,
    version: String,
    source_url: String,
}

fn version_string() -> String {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    //let license = env!("CARGO_PKG_LICENSE");
    //let repo = env!("CARGO_PKG_REPOSITORY");
    format!("OpenJam {} {}", name, version)
}

fn all_valid_chars(s: &str) -> bool {
    for c in s.chars() {
        if !VALID_CHARS.contains(&c) {
            return false;
        }
    }
    true
}

fn registration_ok(form: RegisterForm, renderer: &handlebars::Handlebars<'_>) -> Html {
    let rendered = renderer.render(FORM_NAME, &Context {
        form,
        error: None,
        version: version_string(),
        source_url: env!("CARGO_PKG_REPOSITORY").to_owned(),
        is_registration_allowed: true,
    }).unwrap();
    Html::new(rendered)
}

fn registration_err(form: RegisterForm, error: String , renderer: &handlebars::Handlebars<'_>) -> Html {
    let rendered = renderer.render(FORM_NAME, &Context {
        form,
        error: Some(error),
        version: version_string(),
        source_url: env!("CARGO_PKG_REPOSITORY").to_owned(),
        is_registration_allowed: true,
    }).unwrap();
    Html::new(rendered)
}

fn registration_disabled(form: RegisterForm, renderer: &handlebars::Handlebars<'_>) -> Html {
    let rendered = renderer.render(FORM_NAME, &Context {
        form,
        error: Some("Registration not allowed".to_owned()),
        version: version_string(),
        source_url: env!("CARGO_PKG_REPOSITORY").to_owned(),
        is_registration_allowed: false,
    }).unwrap();
    Html::new(rendered)
}

#[post("/register")]
pub async fn form_submit(form: Form<RegisterForm>, config: Data<super::RcConfig>, handlebars_ref: Data<handlebars::Handlebars<'_>>, server_conf: Data<oj_rc_core::persist::config::ServerConfig>) -> Result<Html, actix_web::error::Error> {
    if !server_conf.allow_signup {
        return Ok(registration_disabled(form.into_inner(), &handlebars_ref));
    }
    // password confirmation validation
    if form.password != form.password_c {
        return Ok(registration_err(form.into_inner(), "Passwords do not match".to_owned(), &handlebars_ref));
    }
    if form.password.len() < 8 {
        return Ok(registration_err(form.into_inner(), "Password too short (minimum 8 characters)".to_owned(), &handlebars_ref));
    }
    if form.password.len() > 128 {
        return Ok(registration_err(form.into_inner(), "Password too long (maximum 128 characters)".to_owned(), &handlebars_ref));
    }

    // email validation
    let actual_email: Option<String>;
    if let Some(email) = &form.email {
        if email.is_empty() {
            actual_email = None;
        } else {
            if !email.contains('@') {
                return Ok(registration_err(form.into_inner(), "Email must contain @".to_owned(), &handlebars_ref));
            }
            let email_exists = config.account_provider.user_exists(oj_rc_core::persist::user::UserId::Email(email.to_owned()))
                .await
                .map_err(|e| {
                    log::error!("Failed to check if user email {} exists: {}", email, e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;
            if email_exists {
                return Ok(registration_err(form.into_inner(), "Email already registered".to_owned(), &handlebars_ref));
            }
            actual_email = Some(email.to_owned());
        }
    } else {
        actual_email = None;
    }

    // steam id validation
    let actual_steam_id: Option<u64>;
    if let Some(steam_id) = &form.steam_id {
        if steam_id.is_empty() {
            actual_steam_id = None;
        } else {
            let steam_id = match steam_id.parse() {
                Ok(id) => id,
                Err(_e) => return Ok(registration_err(form.into_inner(), "Invalid SteamID (not an integer)".to_owned(), &handlebars_ref)),
            };
            if !(7656119_0000000000..7656120_0000000000).contains(&steam_id) {
                return Ok(registration_err(form.into_inner(), "Invalid SteamID (should be like 7656119XXXXXXXXXX)".to_owned(), &handlebars_ref));
            }
            let steam_exists = config.account_provider.user_exists(oj_rc_core::persist::user::UserId::SteamId(steam_id))
                .await
                .map_err(|e| {
                    log::error!("Failed to check if user steam id {} exists: {}", steam_id, e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;
            if steam_exists {
                return Ok(registration_err(form.into_inner(), "SteamID already registered".to_owned(), &handlebars_ref));
            }
            actual_steam_id = Some(steam_id);
        }
    } else {
        actual_steam_id = None;
    }

    // username validation
    if form.display_name.len() < 4 {
        return Ok(registration_err(form.into_inner(), "Username too short (minimum 4 characters)".to_owned(), &handlebars_ref));
    }
    if form.display_name.len() > 32 {
        return Ok(registration_err(form.into_inner(), "Username too long (maximum 32 characters)".to_owned(), &handlebars_ref));
    }
    if !all_valid_chars(&form.display_name.to_lowercase()) {
        return Ok(registration_err(form.into_inner(), "Invalid username (only alphanumerics and _ allowed)".to_owned(), &handlebars_ref));
    }
    let username_exists = config.account_provider.user_exists(oj_rc_core::persist::user::UserId::Username(form.display_name.to_owned()))
        .await
        .map_err(|e| {
            log::error!("Failed to check if user name {} exists: {}", form.display_name, e);
            actix_web::error::ErrorInternalServerError(e)
        })?;
    if username_exists {
        return Ok(registration_err(form.into_inner(), "Username already registered".to_owned(), &handlebars_ref));
    }

    let user_id = match config.account_provider.register(oj_rc_core::persist::user::RegistrationInfo {
        display_name: form.display_name.clone(),
        password: form.password.clone(),
        email: actual_email,
        steam_id: actual_steam_id,
    }).await {
        Ok(id) => id,
        Err(e) => {
            return Ok(registration_err(form.into_inner(), format!("Registration failed: {}", e), &handlebars_ref));
        }
    };

    Ok(Html::new(handlebars_ref.render(FORM_NAME_SUCCESS, &ContextSuccess {
        display_name: form.display_name.clone(),
        id: user_id,
        version: version_string(),
        source_url: env!("CARGO_PKG_REPOSITORY").to_owned(),
    }).unwrap()))
}

#[get("/register")]
pub async fn form_load(handlebars_ref: Data<handlebars::Handlebars<'_>>, server_conf: Data<oj_rc_core::persist::config::ServerConfig>) -> Html {
    if server_conf.allow_signup {
        registration_ok(RegisterForm {
            display_name: "".to_owned(),
            password: "".to_owned(),
            password_c: "".to_owned(),
            email: None,
            steam_id: None,
        }, &handlebars_ref)
    } else {
        registration_disabled(RegisterForm {
            display_name: "".to_owned(),
            password: "".to_owned(),
            password_c: "".to_owned(),
            email: None,
            steam_id: None,
        }, &handlebars_ref)
    }
}

async fn favicon_impl(config: Data<super::RcConfig>) -> impl Responder {
    let path = config.assets.join("favicon.jpg");
    actix_files::NamedFile::open_async(path).await
}

#[get("/robocraft/favicon")]
pub async fn favicon(config: Data<super::RcConfig>) -> impl Responder {
    favicon_impl(config).await
}

#[get("/favicon.ico")]
pub async fn favicon_standard(config: Data<super::RcConfig>) -> impl Responder {
    favicon_impl(config).await
}
