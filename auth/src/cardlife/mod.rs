#[allow(unused_variables)]
mod email;
mod steam;
mod temporary_get_user_id;
#[allow(unused_variables)]
mod token;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.attach(email::stage())
            .attach(steam::stage())
            .attach(temporary_get_user_id::stage())
            .attach(token::stage())
    })
}
