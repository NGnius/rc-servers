#![forbid(unsafe_code)]
mod cli;
mod robocraft;
mod oauth;

use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/")]
async fn index() -> impl Responder {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let git_version = git_version::git_version!(args = ["--always", "--dirty=+"]);
    let authors = env!("CARGO_PKG_AUTHORS");
    let license = env!("CARGO_PKG_LICENSE");
    let repo = env!("CARGO_PKG_REPOSITORY");
    format!("{} {}:{} by [{}]\n{}\n{}", name, version, git_version, authors, license, repo)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli_args = cli::CliArgs::get();

    let conf = oj_rc_core::ConfigImpl::load(&cli_args.assets_robocraft)?;

    if cli_args.validate {
        let res = if conf.self_validate(&cli_args.data_robocraft) {
            log::info!("Config validated successfully (exit success)");
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Config failed validation (exit failure)"))
        };
        return res;
    }

    let server_settings = actix_web::web::Data::new(<oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::server_config(&conf));

    let cli_args2 = actix_web::web::Data::new(cli_args.clone());
    let rc_preloaded = actix_web::web::Data::new(cli_args.clone().preloaded().await);
    let internal_auth = actix_web::web::Data::new(crate::robocraft::intercom::IntercomAuth::new(&cli_args.data_robocraft)?);
    let user_registry = actix_web::web::Data::new(crate::robocraft::intercom::Users::new());

    let mut handlebars_conf = handlebars::Handlebars::new();
    let mut dir_conf = handlebars::DirectorySourceOptions::default();
    dir_conf.tpl_extension = ".html.hbs".to_owned();
    dir_conf.hidden = false;
    dir_conf.temporary = false;
    handlebars_conf
        .register_templates_directory(
            std::path::PathBuf::from(&cli_args.assets_robocraft).parent().expect("Bad robocraft asset path").join("templates"),
            dir_conf,
        )
        .unwrap();
    let handlebars_ref = actix_web::web::Data::new(handlebars_conf);

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                use actix_web::dev::Service;
                log::debug!("Request {} {}", req.method(), req.path());
                srv.call(req)
            })
            .app_data(cli_args2.clone())
            .app_data(rc_preloaded.clone())
            .app_data(internal_auth.clone())
            .app_data(user_registry.clone())
            .app_data(handlebars_ref.clone())
            .app_data(server_settings.clone())
            .service(index)
            .service(robocraft::registration::form_submit)
            .service(robocraft::registration::form_load)
            .service(robocraft::registration::favicon)
            .service(robocraft::registration::favicon_standard)
            .service(robocraft::email::email_password_auth)
            .service(robocraft::steam::steam_auth)
            .service(robocraft::username::user_password_auth)
            .service(robocraft::displayname::displaye_password_auth)
            .service(robocraft::intercom::services_ws)
            .service(robocraft::intercom::service_msg)
            .service(robocraft::intercom::lobby_state_ws)
            .service(robocraft::intercom::lobby_state_msg)
            .service(robocraft::intercom::status_get)
            .service(robocraft::intercom::status_set)
            .service(oauth::openid_config::get_openid_configuration)
            .service(oauth::jwks::get_oauth_jwks)
            .service(oauth::auth::post_oauth_auth)
            .service(oauth::auth::get_oauth_auth)
            .service(oauth::token::post_oauth_token)
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}
