#![forbid(unsafe_code)]
mod common;

#[cfg(feature = "cardlife")]
mod cardlife;

#[rocket::get("/")]
fn index() -> String {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let git_version = git_version::git_version!(args = ["--always", "--dirty=+"]);
    let authors = env!("CARGO_PKG_AUTHORS");
    let license = env!("CARGO_PKG_LICENSE");
    let repo = env!("CARGO_PKG_REPOSITORY");
    format!("{} {}:{} by [{}]\n{}\n{}", name, version, git_version, authors, license, repo)
}

#[rocket::launch]
async fn rocket() -> _ {
    env_logger::init();
    let args = common::cli::CliArgs::get();
    #[allow(unused_mut)]
    let mut builder = rocket::build().mount("/", rocket::routes![index])
        .manage(args.preloaded().await);

    #[cfg(feature = "cardlife")]
    {builder = builder.attach(cardlife::stage());}

    builder
}

