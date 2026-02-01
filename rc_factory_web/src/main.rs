mod cli;
mod robocraft;

use actix_web::{App, HttpServer, Responder};
use oj_rc_core::persist::config::{ConfigImpl, ConfigProvider};

#[actix_web::get("/version")]
async fn index() -> impl Responder {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let git_version = git_version::git_version!(args = ["--always", "--dirty=+"]);
    let authors = env!("CARGO_PKG_AUTHORS");
    let license = env!("CARGO_PKG_LICENSE");
    let repo = env!("CARGO_PKG_REPOSITORY");
    format!("{} {}:{} by [{}]\n{}\n{}", name, version, git_version, authors, license, repo)
}

fn io_error(e: impl ToString) -> std::io::Error {
    std::io::Error::other(e.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli_args = cli::CliArgs::get();

    let conf = ConfigImpl::load(&cli_args.assets).map_err(io_error)?;
    let users = oj_rc_core::persist::user::UserImpl::load(&cli_args.data, &conf).await.map_err(io_error)?;
    let factory_enum = <ConfigImpl as ConfigProvider<()>>::factory(&conf, &|| users.factory_impl()).await.map_err(io_error)?;
    let factory_data = actix_web::web::Data::new(factory_enum);

    let mut handlebars_conf = handlebars::Handlebars::new();
    let mut dir_conf = handlebars::DirectorySourceOptions::default();
    dir_conf.tpl_extension = ".html.hbs".to_owned();
    dir_conf.hidden = false;
    dir_conf.temporary = false;
    handlebars_conf
        .register_templates_directory(
            std::path::PathBuf::from(&cli_args.assets).parent().expect("Bad asset path").join("templates"),
            dir_conf,
        )
        .unwrap();
    let handlebars_ref = actix_web::web::Data::new(handlebars_conf);

    let assets_root = actix_web::web::Data::new(std::path::PathBuf::from(&cli_args.assets));

    HttpServer::new(move || {
        App::new()
            .app_data(factory_data.clone())
            .app_data(assets_root.clone())
            .app_data(handlebars_ref.clone())
            .service(index)
            .service(robocraft::web_ui::index)
            .service(robocraft::web_ui::app_js)
            .service(robocraft::web_ui::favicon)
            .service(robocraft::web_ui::favicon_standard)
            .service(robocraft::factory::crf_api::list)
            .service(robocraft::factory::crf_api::list_default)
            .service(robocraft::factory::crf_api::get)
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}
