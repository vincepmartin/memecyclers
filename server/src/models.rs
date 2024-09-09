use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::{
    form::FromForm,
    fs::TempFile,
    serde::{Deserialize, Serialize},
};

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
// Will mostly be convered to a InsertableRide for DB insertion.
#[derive(FromForm)]
pub struct RideData<'d> {
    pub title: String,
    pub description: String,
    pub data: Option<TempFile<'d>>,
}
