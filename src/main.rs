#[macro_use]
extern crate rocket;

// Build in stuff...
use std::env;
use std::vec;

// Other peoples stuff...
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

// Get env vars from dot files.
use dotenvy::dotenv;

// Let us use some features from rocket to "custom"
// configure our DB connection in the launch method.
use rocket::figment::{
    util::map,
    value::{Map, Value},
};

use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel};

// My modules...
// mod libs;
mod models;
mod schema;
use models::{InsertableRide, Ride};

// Create our DB struct...
#[database("rides_db")]
struct RidesDb(diesel::PgConnection);

// Return a particular ride based on id.
#[get("/ride/<ride_id>")]
async fn get_ride(conn: RidesDb, ride_id: i32) -> Option<Json<Ride>> {
    use schema::rides::dsl::*;
    // The move tells the closure below to BORROW all variables that it needs.
    // Since they borrow it, the higher level items can't destroy it until we are done with it.

    conn.run(move |conn| {
        rides
            .filter(id.eq(ride_id))
            .select(Ride::as_select())
            .first(conn)
            .optional()
    })
    .await
    .ok()
    .flatten()
    .map(Json)
}

// TODO: Implement this.
// Get a list of all rides in the DB.
// #[get("/ride")]
// fn get_all_ride_ids() -> Json<Vec<Ride>> {}

// Create a new ride.
#[post("/ride", format = "json", data = "<ride>")]
async fn post_ride(conn: RidesDb, ride: Json<InsertableRide>) -> Json<String> {
    use schema::rides::dsl::*;
    conn.run(move |conn| diesel::insert_into(rides).values(&*ride).execute(conn))
        .await
        .ok();

    return Json("OK".to_string());
}

#[launch]
fn rocket() -> _ {
    // Configure database connection via .env files.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: Map<_, Value> = map! {
        "url" => database_url.into(),
        "pool_size" => 10.into(),
        "timeout" => 5.into()
    };

    let figment = rocket::Config::figment().merge(("databases", map!["rides_db" => db]));
    rocket::custom(figment)
        .attach(RidesDb::fairing())
        .mount("/", routes![get_ride, post_ride])
}
