// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "geometry"))]
    pub struct Geometry;
}

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
    use diesel::sql_types::*;
    use super::sql_types::Geometry;

    ride_geometries (id) {
        id -> Int4,
        rides_id -> Int4,
        geometry -> Nullable<Geometry>,
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

diesel::table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::joinable!(ride_files -> rides (ride_id));
diesel::joinable!(ride_geometries -> rides (rides_id));

diesel::allow_tables_to_appear_in_same_query!(
    ride_files,
    ride_geometries,
    rides,
    spatial_ref_sys,
);
