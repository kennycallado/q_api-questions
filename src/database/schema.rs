// @generated automatically by Diesel CLI.

diesel::table! {
    question_translations (id) {
        id -> Int4,
        question_id -> Int4,
        locale -> Varchar,
        question -> Varchar,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        question_type -> Varchar,
    }
}

diesel::joinable!(question_translations -> questions (question_id));

diesel::allow_tables_to_appear_in_same_query!(
    question_translations,
    questions,
);
