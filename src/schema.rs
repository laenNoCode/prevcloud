// @generated automatically by Diesel CLI.

diesel::table! {
    cookies (id) {
        id -> Text,
        user_id -> Int4,
        expires -> Date,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
        salt -> Text,
    }
}

diesel::joinable!(cookies -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cookies,
    users,
);
