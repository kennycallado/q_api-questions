use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::questions::model::QuestionWithContent;
use crate::app::modules::questions::services::repository as questions_repository;

pub async fn get_show_admin(db: Db, _admin: UserInClaims, id: i32, lang: String) -> Result<Json<QuestionWithContent>, Status> {
    let question =  questions_repository::get_by_id(&db, id, lang).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn get_multiple_admin(db: Db, _admin: UserInClaims, ids: Vec<i32>, lang: String) -> Result<Json<Vec<QuestionWithContent>>, Status> {
    let questions = questions_repository::get_questions_by_ids(&db, ids, lang).await;

    match questions {
        Ok(questions) => Ok(Json(questions)),
        Err(_) => Err(Status::InternalServerError)
    }
}
