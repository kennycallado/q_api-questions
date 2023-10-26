use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::questions::model::QuestionWithContent;
use crate::app::modules::questions::services::repository as questions_repository;

pub async fn get_index_admin(db: Db, _admin: UserInClaims, lang: String) -> Result<Json<Vec<QuestionWithContent>>, Status> {
    let questions = questions_repository::get_all(&db, lang).await;

    match questions {
        Ok(questions) => Ok(Json(questions)),
        Err(_) => Err(Status::InternalServerError),
    }
}

