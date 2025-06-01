mod cli;
mod robocraft;

use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/")]
async fn index() -> impl Responder {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let git_version = git_version::git_version!(args = ["--always", "--dirty=+"]);
    let authors = env!("CARGO_PKG_AUTHORS");
    let license = env!("CARGO_PKG_LICENSE");
    let repo = env!("CARGO_PKG_REPOSITORY");
    format!("{} {}{} by [{}]\n{}\n{}", name, version, git_version, authors, license, repo)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli_args = cli::CliArgs::get();
    let cli_args2 = cli_args.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(cli_args2.clone()))
            .service(index)
            .service(robocraft::live_data::live_data_json)
            .service(robocraft::user_avatar::get)
            .service(robocraft::clan_avatar::get)
            .service(robocraft::brawl_data::get)
            .service(robocraft::campaign_data::get)
            .service(robocraft::factory::arc::get)
            .service(robocraft::favicon::get)
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}
