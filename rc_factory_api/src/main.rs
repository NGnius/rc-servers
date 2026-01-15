mod cli;
mod robocraft;

use actix_web::{App, HttpServer, Responder};
use oj_rc_core::persist::config::{ConfigImpl, ConfigProvider};

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

fn io_error(e: impl ToString) -> std::io::Error {
    std::io::Error::other(e.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli_args = cli::CliArgs::get();

    let conf = ConfigImpl::load(&cli_args.assets).map_err(io_error)?;
    let factory_enum = <ConfigImpl as ConfigProvider<()>>::factory(&conf).await.map_err(io_error)?;

    let factory_data = actix_web::web::Data::new(factory_enum);

    HttpServer::new(move || {
        App::new()
            .app_data(factory_data.clone())
            .service(index)
            .service(robocraft::factory::crf_api::list)
            .service(robocraft::factory::crf_api::list_default)
            .service(robocraft::factory::crf_api::get)
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}
