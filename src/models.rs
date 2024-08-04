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
}

// InsertableRide Struct
// TODO: There has to be a way to make this and the Ride struct the same?
// Optionals or something?
#[derive(Insertable)]
#[diesel(table_name = crate::schema::rides)]
#[diesel(check_for_backend(diesel::pg::Pg))]
// TODO: Do we need both Deserialize and Serialize?
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InsertableRide {
    pub title: String,
    pub description: String,
}
