// @generated automatically by Diesel CLI.

diesel::table! {
    rides (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        created_date -> Timestamptz,
    }
}
