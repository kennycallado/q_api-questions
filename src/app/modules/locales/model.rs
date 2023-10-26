
use diesel::{sql_types::Text, pg::Pg, row::Row, expression::AsExpression, helper_types::AsExprOf, deserialize::FromSqlRow};
use serde::{Deserialize, Serialize};

use crate::database::schema::question_translations;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[diesel(table_name = question_translations)]
#[serde(crate = "rocket::serde")]
pub struct QuestionContent {
    pub id: i32,
    pub question_id: i32,
    pub locale: String,
    pub question: String
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = question_translations)]
#[serde(crate = "rocket::serde")]
pub struct NewQuestionContent {
    pub question_id: Option<i32>,
    pub locale: String,
    pub question: String
}
