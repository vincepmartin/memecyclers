#[macro_use]
extern crate rocket;

// Build in stuff...
use std::env;
use std::vec;

// Other peoples stuff...
// TODO: All this diesel stuff was just imported beacuse the compiler was complaining.
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

// Get env vars from dot files.
use dotenvy::dotenv;

// Let us use some features from rocket to "custom"
// configure our DB connection in the launch method.
use rocket::figment::{
    util::map,
    value::{Map, Value},
};

// use rocket::sentinel::resolution::DefaultSentinel;
use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel};

// My modules...
// mod libs;
mod models;
mod schema;
use models::Ride;

// Create our DB struct...
#[database("rides_db")]
// We are using the Diesel client.
// Before this I was confused, I was using the Rust-Postgres client and
// mistakenly trying ti implement the diesel one.
struct RidesDb(diesel::PgConnection);

// Return a particular ride based on id.
#[get("/ride/<ride_id>")]
async fn get_ride(conn: RidesDb, ride_id: i32) -> Option<Json<Ride>> {
    use schema::rides::dsl::*;
    // The move tells the closure below to BORROW all variables that it needs.
    // Since they borrow it, the higher level items can't destroy it until we are done with it.
    // If it's like this, it might solve it... conn.run(move |conn| {

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

// Get a list of all rides in the DB.
// #[get("/ride")]
// fn get_all_ride_ids() -> Json<Vec<Ride>> {}

// Create a new ride.
// #[post("/ride", data = "<ride>")]
// fn post_ride(ride: Json<Ride>) -> Json<String> {
//     Json(format!("New ride: {}", ride.title))
// }
//
// nachos

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
        .mount("/", routes![get_ride])
}
