mod robocraft;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> _ {
    env_logger::init();
    rocket::build().mount("/", rocket::routes![index])
        .attach(robocraft::stage())
}

