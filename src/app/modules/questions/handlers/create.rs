use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::questions::model::{NewQuestionWithContent, QuestionWithContent};
use crate::app::modules::questions::services::repository as questions_repository;

pub async fn post_create_admin(db: Db, _admin: UserInClaims, new_question: NewQuestionWithContent) -> Result<Json<QuestionWithContent>, Status> {
    let question = questions_repository::create(&db, new_question).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(_) => Err(Status::InternalServerError)
    }
}
