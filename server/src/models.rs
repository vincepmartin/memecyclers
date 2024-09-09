use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::{
    form::FromForm,
    fs::TempFile,
    serde::{Deserialize, Serialize},
};

/*
 * Define items that are returned to the user.
 */

// Returned data container.  All data returned will be
// encapsulated in this.
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum ApiResponse<T> {
    #[serde(rename = "success")]
    Success { data: T },
    #[serde(rename = "error")]
    Error { error: ApiError },
}

// Error
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub message: String,
}

// Ride Struct
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::rides)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Ride {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
}

// RideFile
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::ride_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct RideFile {
    pub id: i32,
    pub created_date: DateTime<Utc>,
    pub description: String,
    pub rides_id: i32,
    pub file_name: String,
    pub file_type: String,
}

// InsertableRideFile
#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::ride_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct InsertableRideFile {
    pub description: String,
    pub rides_id: i32,
    pub file_name: String,
    pub file_type: String,
}

// InsertableRide Struct
#[derive(Insertable, Deserialize, Serialize, FromForm)]
#[diesel(table_name = crate::schema::rides)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct InsertableRide {
    pub title: String,
    pub description: String,
}

// RideData Struct
// Used to add a new ride with includes binary files.
// Will mostly be converted to an InsertableRide for DB insertion.
// Additionally we will also probably have an InsertableRideFile and RideFile
// struct as well.
#[derive(FromForm)]
pub struct RideData<'d> {
    pub title: String,
    pub description: String,
    pub data: Option<Vec<TempFile<'d>>>,
}
