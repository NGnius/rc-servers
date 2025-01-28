mod debug;
mod email;
mod steam;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("robocraft", |rocket| async {
        rocket.attach(email::stage())
            .attach(steam::stage())
            .attach(debug::stage())
    })
}
