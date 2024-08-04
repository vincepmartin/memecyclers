use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

// Ride Struct
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::rides)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ride {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
}

// InsertableRide Struct
#[derive(Insertable)]
#[diesel(table_name = crate::schema::rides)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InsertableRide {
    pub title: String,
    pub description: String,
}
