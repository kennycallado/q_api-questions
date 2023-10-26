use diesel::prelude::*;
use diesel::query_builder::AsQuery;

use crate::app::modules::locales::model::{QuestionContent, NewQuestionContent};
use crate::app::modules::questions::model::{Question, NewQuestion, QuestionType, QuestionWithContent, NewQuestionWithContent};
use crate::database::connection::Db;
use crate::database::schema::questions;
use crate::database::schema::question_translations;

pub async fn get_all(db: &Db, lang: String) -> Result<Vec<QuestionWithContent>, diesel::result::Error> {
    let result: Vec<QuestionWithContent> = db.run(move |conn| {
        let blah = questions::table
            .inner_join(question_translations::table.on(questions::id.eq(question_translations::question_id)))
            .filter(question_translations::locale.eq(lang))
            .select((questions::all_columns, question_translations::question))
            .load::<((i32, String), String)>(conn);

        blah.map(|x| x.into_iter().map(|q| q.into()).collect())
    }).await?;

    Ok(result)
}

pub async fn get_by_id(db: &Db, id: i32, lang: String) -> Result<QuestionWithContent, diesel::result::Error> {
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

pub async fn get_questions_by_ids(db: &Db, ids: Vec<i32>, lang: String) -> Result<Vec<QuestionWithContent>, diesel::result::Error> {
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

pub async fn create(db: &Db, new_question: NewQuestionWithContent) -> Result<QuestionWithContent, diesel::result::Error> {
    let result = db.run(move |conn| {
        let q = NewQuestion { question_type: new_question.question_type };
        let question = diesel::insert_into(questions::table)
            .values(q)
            .get_result::<Question>(conn).unwrap();

        let q = NewQuestionContent {
            question_id: Some(question.id),
            locale: new_question.content.locale,
            question: new_question.content.question
        };

        let content = diesel::insert_into(question_translations::table) 
            .values(q)
            .get_result::<QuestionContent>(conn).unwrap();

        QuestionWithContent {
            id: question.id,
            question_type: question.question_type,
            question: content.question
        }
    }).await;

    Ok(result)
}

pub async fn update(db: &Db, id: i32, new_question: NewQuestionWithContent) -> Result<QuestionWithContent, diesel::result::Error> {
    let result = db.run(move |conn| {
        let q = NewQuestion { question_type: new_question.question_type };
        let question = diesel::update(questions::table.find(id))
            .set(q)
            .get_result::<Question>(conn).unwrap();

        let q = NewQuestionContent {
            question_id: Some(question.id),
            locale: new_question.content.locale.clone(),
            question: new_question.content.question,
        };

        let content = diesel::update(question_translations::table.filter(question_translations::question_id.eq(id)))
            .filter(question_translations::locale.eq(new_question.content.locale))
            .set(&q)
            .get_result::<QuestionContent>(conn);

        match content {
            Ok(content) => {
                QuestionWithContent {
                    id: question.id,
                    question_type: question.question_type,
                    question: content.question
                }
            },
            Err(_) => {
                let blah = diesel::insert_into(question_translations::table)
                    .values(q)
                    .get_result::<QuestionContent>(conn).unwrap();

                QuestionWithContent {
                    id: question.id,
                    question_type: question.question_type,
                    question: blah.question
                }
            }
        }

    }).await;

    Ok(result)
}
