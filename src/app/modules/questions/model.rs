use std::fmt;

// use chrono::NaiveDateTime;

use diesel::{sql_types::Text, pg::Pg, row::Row, expression::AsExpression, helper_types::AsExprOf, deserialize::FromSqlRow};
use serde::{Deserialize, Serialize};

use crate::database::schema::questions;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Question {
    pub id: i32,
    pub question_type: QuestionType,
    pub question: String,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

impl From<(i32, String, String)> for Question {
    fn from(value: (i32, String, String)) -> Self {
        Question {
            id: value.0,
            question_type: match value.1.as_ref() {
                "checkbox" => QuestionType::Checkbox,
                "input" => QuestionType::Input,
                "radio" => QuestionType::Radio,
                "range" => QuestionType::Range,
                _ => panic!("Unknown question type"),
            },
            question: value.2,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = questions)]
#[serde(crate = "rocket::serde")]
pub struct NewQuestion {
    pub question_type: QuestionType,
    pub question: String,
}

impl From<Question> for NewQuestion {
    fn from(question: Question) -> Self {
        NewQuestion {
            question_type: QuestionType::from(question.question_type),
            question: question.question,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QuestionType {
    Checkbox,
    Input,
    Radio,
    Range,
}

impl fmt::Display for QuestionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                QuestionType::Checkbox => "checkbox",
                QuestionType::Input => "input",
                QuestionType::Radio => "radio",
                QuestionType::Range => "range",
            }
        )
    }
}

impl FromSqlRow<Text, Pg> for QuestionType {
    fn build_from_row<'a>(row: &impl Row<'a, Pg>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match String::build_from_row(row)?.as_ref() {
            "checkbox" => Ok(QuestionType::Checkbox),
            "input" => Ok(QuestionType::Input),
            "radio" => Ok(QuestionType::Radio),
            "range" => Ok(QuestionType::Range),
            v => Err(format!("Unknown value {} for QuestionType found", v).into()),
        }
    }
}

impl AsExpression<Text> for QuestionType {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

impl<'a> AsExpression<Text> for &'a QuestionType {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}
