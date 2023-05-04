use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::claims::AccessClaims;
use crate::config::database::Db;

use crate::app::modules::questions::handlers::{create, index, show, update};

use crate::app::modules::questions::model::{Question, NewQuestion};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        option_index,
        option_show,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
    ]
}

#[options("/")]
pub async fn option_index() -> Status {
    Status::Ok
}

#[options("/<_id>")]
pub async fn option_show(_id: i32) -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<Question>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(db, claims.0.user).await,
        _ => {
            println!("Error: get_index; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 101)]
pub async fn get_show(db: Db, claims: AccessClaims, id: i32) ->  Result<Json<Question>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(db, claims.0.user, id).await,
        _ => {
            println!("Error: get_show; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>", rank = 102)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_question>", rank = 1)]
pub async fn post_create(db: Db, claims: AccessClaims, new_question: Json<NewQuestion>) -> Result<Json<Question>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_admin(db, claims.0.user, new_question.into_inner()).await,
        _ => {
            println!("Error: post_create; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[post("/", data = "<_new_question>", rank = 2)]
pub async fn post_create_none(_new_question: Json<NewQuestion>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<question>", rank = 101)]
pub async fn put_update(db: Db, claims: AccessClaims, id: i32, question: Json<NewQuestion>) -> Result<Json<Question>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(db, claims.0.user, id, question.into_inner()).await,
        _ => {
            println!("Error: put_update; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[put("/<_id>", data = "<_question>", rank = 102)]
pub async fn put_update_none(_id: i32, _question: Json<NewQuestion>) -> Status {
    Status::Unauthorized
}
