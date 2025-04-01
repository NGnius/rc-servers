mod common;

#[cfg(feature = "cardlife")]
mod cardlife;
#[cfg(feature = "robocraft")]
mod robocraft;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> _ {
    env_logger::init();
    let args = common::cli::CliArgs::get();
    #[allow(unused_mut)]
    let mut builder = rocket::build().mount("/", rocket::routes![index])
        .manage(args.preloaded());

    #[cfg(feature = "cardlife")]
    {builder = builder.attach(cardlife::stage());}
    #[cfg(feature = "robocraft")]
    {builder = builder.attach(robocraft::stage());}

    builder
}

#[cfg(all(feature = "steam", feature = "robocraft", feature = "cardlife"))]
compile_error!("Feature \"steam\" cannot work with features \"cardlife\" and \"robocraft\" at the same time");

