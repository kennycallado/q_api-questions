use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::questions::model::Question;
use crate::app::modules::questions::services::repository as questions_repository;

pub async fn get_show_admin(db: Db, _admin: UserInClaims, id: i32) -> Result<Json<Question>, Status> {
    let question =  questions_repository::get_by_id(&db, id).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn get_multiple_admin(db: Db, _admin: UserInClaims, ids: Vec<i32>) -> Result<Json<Vec<Question>>, Status> {
    let questions = questions_repository::get_questions_by_ids(&db, ids).await;

    match questions {
        Ok(questions) => Ok(Json(questions)),
        Err(_) => Err(Status::InternalServerError)
    }
}
