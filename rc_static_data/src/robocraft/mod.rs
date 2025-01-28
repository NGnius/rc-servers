mod live_data;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.attach(live_data::stage())
    })
}
