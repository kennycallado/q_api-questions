use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::questions::model::Question;
use crate::app::modules::questions::services::repository as questions_repository;

pub async fn get_index_admin(db: Db, _admin: UserInClaims) -> Result<Json<Vec<Question>>, Status> {
    let questions = questions_repository::get_all(&db).await;

    match questions {
        Ok(questions) => Ok(Json(questions)),
        Err(_) => Err(Status::InternalServerError),
    }
}

