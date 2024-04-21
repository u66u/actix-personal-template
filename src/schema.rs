// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
    }
}

diesel::table! {
    wordpairs (id) {
        id -> Int4,
        spanish_word -> Text,
        english_word -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(users, wordpairs,);
