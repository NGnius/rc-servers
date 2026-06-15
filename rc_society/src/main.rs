#![forbid(unsafe_code)]
mod cli;
mod api;
mod web;

use actix_web::{App, HttpServer, Responder};

pub static START_TIME: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(-1);

#[actix_web::get("/version")]
async fn version_info() -> impl Responder {
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

    let config = oj_rc_core::ConfigImpl::load(&cli_args.assets_robocraft)?;

    let server_settings = actix_web::web::Data::new(<oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::server_config(&config));

    let parsers = oj_rc_core::cubes::CubeParsers::new(&config);

    let vehicle_importers = crate::api::garage::plugins::ImportPlugins::standard(&cli_args.assets_robocraft, &parsers);
    log::info!("Loaded {} vehicle import/export plugins", vehicle_importers.plugin_names().count());
    let importers_ref = actix_web::web::Data::new(vehicle_importers);

    let parsers_ref = actix_web::web::Data::new(parsers);

    let users = oj_rc_core::UserImpl::load(&cli_args.data_robocraft, &config).await.expect("Bad user data");
    let auth_ref = actix_web::web::Data::new(Box::new(users));

    let cli_args2 = actix_web::web::Data::new(cli_args.clone());
    let loadeds_args = actix_web::web::Data::new(cli_args.loaded());

    let mut handlebars_conf = handlebars::Handlebars::new();
    let mut dir_conf = handlebars::DirectorySourceOptions::default();
    dir_conf.tpl_extension = ".html.hbs".to_owned();
    dir_conf.hidden = false;
    dir_conf.temporary = false;
    handlebars_conf
        .register_templates_directory(
            std::path::PathBuf::from(&cli_args.assets_robocraft).parent().expect("Bad robocraft asset path").join("templates/rc_society"),
            dir_conf,
        )
        .unwrap();
    let handlebars_ref = actix_web::web::Data::new(handlebars_conf);

    START_TIME.store(chrono::Utc::now().timestamp(), std::sync::atomic::Ordering::SeqCst);

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                use actix_web::dev::Service;
                log::trace!("Request {} {}", req.method(), req.path());
                srv.call(req)
            })
            .wrap(actix_identity::IdentityMiddleware::default())
            .wrap(actix_session::SessionMiddleware::new(
                actix_session::storage::CookieSessionStore::default(),
                loadeds_args.cookie_key.clone(),
            ))
            .app_data(cli_args2.clone())
            .app_data(loadeds_args.clone())
            .app_data(handlebars_ref.clone())
            .app_data(server_settings.clone())
            .app_data(auth_ref.clone())
            .app_data(importers_ref.clone())
            .app_data(parsers_ref.clone())
            .service(version_info)
            .service(web::login::form_submit)
            .service(web::login::form_load)
            .service(web::favicon::favicon_standard)
            .service(web::dashboard::get)
            .service(web::dashboard::post) // for login redirect
            .service(web::index::get)
            .service(web::garage::list::get)
            .service(web::garage::info::get)
            .service(web::garage::export::get)
            .service(web::garage::import::get_existing)
            .service(web::garage::import::get_new)
            .service(web::garage::import::post)
            .service(web::garage::selected::get)
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}
