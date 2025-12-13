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
    format!("{} {}:{} by [{}]\n{}\n{}", name, version, git_version, authors, license, repo)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli_args = cli::CliArgs::get();
    let cli_args2 = actix_web::web::Data::new(cli_args.clone());
    let internal_auth = actix_web::web::Data::new(crate::robocraft::IntercomAuth::new(&cli_args.data_robocraft)?);
    HttpServer::new(move || {
        App::new()
            .app_data(cli_args2.clone())
            .app_data(internal_auth.clone())
            .service(index)
            .service(robocraft::live_data::live_data_json)
            .service(robocraft::user_avatar::get)
            .service(robocraft::user_avatar::post)
            .service(robocraft::clan_avatar::get)
            .service(robocraft::brawl_data::get)
            .service(robocraft::campaign_data::get)
            .service(robocraft::factory::arc::get)
            .service(robocraft::factory::thumbnail::get)
            .service(robocraft::factory::thumbnail::post)
            .service(robocraft::favicon::get)
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}
