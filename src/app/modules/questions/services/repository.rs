use diesel::prelude::*;
// use diesel::sql_types::Timestamptz;

use crate::app::modules::questions::model::QuestionType;
use crate::database::connection::Db;
use crate::database::schema::questions;
use crate::database::schema::question_translations;

use crate::app::modules::questions::model::{Question, NewQuestion};

pub async fn get_all(db: &Db, lang: String) -> Result<Vec<Question>, diesel::result::Error> {
    let result = db.run(move |conn| {
        questions::table
            .inner_join(question_translations::table.on(questions::id.eq(question_translations::question_id)))
            .filter(question_translations::locale.eq(lang))
            .select((questions::all_columns, question_translations::question))
            .load::<((i32, String), String)>(conn)
    }).await?;

    let questions = result.into_iter().map(|((id, question_type), question)|
        Question { id,
            question_type: match question_type.as_ref() {
                "checkbox" => QuestionType::Checkbox,
                "input" => QuestionType::Input,
                "radio" => QuestionType::Radio,
                "range" => QuestionType::Range,
                _ => panic!("Unknown question type"),
            },
            question }
    ).collect();


    Ok(questions)
}

pub async fn get_by_id(db: &Db, id: i32, lang: String) -> Result<Question, diesel::result::Error> {
    let result = db.run(move |conn| {
        questions::table
            .filter(questions::id.eq(id))
            .inner_join(question_translations::table.on(questions::id.eq(question_translations::question_id)))
            .filter(question_translations::locale.eq(lang))
            .select((questions::all_columns, question_translations::question))
            .first::<((i32, String), String)>(conn)
    }).await?;

    Ok(result.into())
}

pub async fn get_questions_by_ids(db: &Db, ids: Vec<i32>, lang: String) -> Result<Vec<Question>, diesel::result::Error> {
    // let result = db.run(move |conn| questions::table.filter(questions::id.eq_any(ids)).load::<(i32, String, String)>(conn)).await?;

    let result = db.run(move |conn| {
        questions::table
            .inner_join(question_translations::table.on(questions::id.eq(question_translations::question_id)))
            .filter(question_translations::locale.eq(lang))
            .select((questions::all_columns, question_translations::question))
            .filter(questions::id.eq_any(ids))
            .load::<((i32, String), String)>(conn)
    }).await?;

    let questions = result.into_iter().map(|q| q.into()).collect();

    Ok(questions)
}

pub async fn create(db: &Db, question: NewQuestion) -> Result<Question, diesel::result::Error> {
    // let result = db.run(move |conn| {
    //     diesel::insert_into(questions::table)
    //         .values(&question)
    //         // .get_result::<(i32, String, String)>(conn)
    //         .get_result::<(i32, String)>(conn)
    // }).await?;

    // Ok(result.into())
    
    unimplemented!()
}

pub async fn update(db: &Db, id: i32, question: NewQuestion) -> Result<Question, diesel::result::Error> {
    // let result = db.run(move |conn| {
    //     diesel::update(questions::table.find(id))
    //         .set(&question)
    //         // .get_result::<(i32, String, String)>(conn)
    //         .get_result::<(i32, String)>(conn)
    // }).await?;

    // Ok(result.into())

    unimplemented!()
}
