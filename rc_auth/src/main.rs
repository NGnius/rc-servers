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
    let rc_preloaded = actix_web::web::Data::new(cli_args.clone().preloaded().await);

    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(
            std::path::PathBuf::from(&cli_args.assets_robocraft).parent().expect("Bad robocraft asset path").join("templates"),
            handlebars::DirectorySourceOptions {
                tpl_extension: ".html.hbs".to_owned(),
                hidden: false,
                temporary: false,
            },
        )
        .unwrap();
    let handlebars_ref = actix_web::web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(cli_args2.clone())
            .app_data(rc_preloaded.clone())
            .app_data(handlebars_ref.clone())
            .service(index)
            .service(robocraft::registration::form_submit)
            .service(robocraft::registration::form_load)
            .service(robocraft::registration::favicon)
            .service(robocraft::email::email_password_auth)
            .service(robocraft::steam::steam_auth)
            .service(robocraft::username::user_password_auth)
            /*.service(robocraft::live_data::live_data_json)
            .service(robocraft::user_avatar::get)
            .service(robocraft::clan_avatar::get)
            .service(robocraft::brawl_data::get)
            .service(robocraft::campaign_data::get)
            .service(robocraft::factory::arc::get)
            .service(robocraft::favicon::get)*/
    })
    .bind((cli_args.ip, cli_args.port))?
    .run()
    .await
}
