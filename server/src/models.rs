use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::{
    form::FromForm,
    fs::TempFile,
    serde::{Deserialize, Serialize},
};

/*
 * These two Structs are meant to wrap every response.
 */

// ApiResponse struct acts as template for all responses.
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T> {
    pub data: T,
}

/*
 * These structs model our applications data items
 * for use in conversting w/ the database.
 */

// Ride Struct maps to DB.
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
#[derive(
    Deserialize, Serialize, Queryable, Selectable, Identifiable, Associations, Debug, PartialEq,
)]
#[diesel(table_name = crate::schema::ride_files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Ride))]
#[serde(crate = "rocket::serde")]
pub struct RideFile {
    pub id: i32,
    pub created_date: DateTime<Utc>,
    pub ride_id: i32,
    pub description: String,
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

// InsertableRideFile
#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::ride_files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct InsertableRideFile {
    pub description: String,
    pub ride_id: i32,
    pub file_name: String,
    pub file_type: String,
}

// RideWithFiles combines Ride and a Vec of RideFile structs.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct RideWithFiles {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
    pub ride_files: Vec<RideFile>,
}

// RideWithFiles used for submitting form with files.
#[derive(FromForm)]
pub struct RideData<'d> {
    pub title: String,
    pub description: String,
    pub data: Option<Vec<TempFile<'d>>>,
}
