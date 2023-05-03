use crate::app::modules::questions::controller::routes as questions_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
            .mount("/api/v1/question", questions_routes() )
    })
}
