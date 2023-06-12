use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::questions::model::{Question, NewQuestion};
use crate::app::modules::questions::services::repository as questions_repository;

pub async fn put_update_admin(db: Db, _admin: UserInClaims, id: i32, new_question: NewQuestion) -> Result<Json<Question>, Status> {
    let question = questions_repository::update(&db, id, new_question).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(_) => {
            println!("Error: put_update_admin; Question not updated");
            Err(Status::InternalServerError)
        }
    }
}
