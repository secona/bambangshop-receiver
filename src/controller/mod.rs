pub mod notification;

use rocket::fairing::AdHoc;

pub fn route_stage() -> AdHoc {
    AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/", routes![notification::subscribe, notification::unsubscribe, notification::receive])
    })
}
