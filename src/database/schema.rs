// @generated automatically by Diesel CLI.

diesel::table! {
    questions (id) {
        id -> Int4,
        question_type -> Varchar,
        question -> Varchar,
    }
}
