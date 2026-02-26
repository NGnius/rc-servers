#![forbid(unsafe_code)]
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
    let token_secret = actix_web::web::Data::new(robocraft::TokenSecret::load(&cli_args.data_robocraft).await?);

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                use actix_web::dev::Service;
                log::trace!("Request {} {}", req.method(), req.path());
                srv.call(req)
            })
            .app_data(cli_args2.clone())
            .app_data(token_secret.clone())
            .service(index)
            .service(robocraft::robopay_store)
            .service(robocraft::robopay_token)
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}

