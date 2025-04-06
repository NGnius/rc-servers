#![forbid(unsafe_code)]
mod robocraft;
mod cli;

#[rocket::get("/")]
fn index() -> String {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let license = env!("CARGO_PKG_LICENSE");
    let repo = env!("CARGO_PKG_REPOSITORY");
    format!("{} {} by [{}]\n{}\n{}", name, version, authors, license, repo)
}

#[rocket::launch]
fn rocket() -> _ {
    env_logger::init();
    let args = cli::CliArgs::get();
    rocket::build().mount("/", rocket::routes![index])
        .attach(robocraft::stage())
        .manage(args)
}

