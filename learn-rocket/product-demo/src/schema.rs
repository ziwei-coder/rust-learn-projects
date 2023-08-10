// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        create_at -> Timestamp,
    }
}
