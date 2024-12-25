// @generated automatically by Diesel CLI.

diesel::table! {
    ride_files (id) {
        id -> Int4,
        created_date -> Timestamptz,
        ride_id -> Int4,
        description -> Text,
        #[max_length = 255]
        file_name -> Varchar,
        #[max_length = 8]
        file_type -> Varchar,
    }
}

diesel::table! {
    rides (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        created_date -> Timestamptz,
    }
}

diesel::joinable!(ride_files -> rides (ride_id));

diesel::allow_tables_to_appear_in_same_query!(
    ride_files,
    rides,
);
