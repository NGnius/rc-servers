mod cli;
mod robocraft;

use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/")]
async fn index() -> impl Responder {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let license = env!("CARGO_PKG_LICENSE");
    let repo = env!("CARGO_PKG_REPOSITORY");
    format!("{} {} by [{}]\n{}\n{}", name, version, authors, license, repo)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli_args = cli::CliArgs::get();
    let cli_args2 = cli_args.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(cli_args2.clone()))
            .service(index)
            .service(robocraft::live_data::live_data_json)
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}
