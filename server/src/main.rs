#[macro_use]
extern crate rocket;

// Build in stuff...
use std::{env, vec};

// Get env vars from dot files.
use dotenvy::dotenv;

// Let us use some features from rocket to "custom"
// configure our DB connection in the launch method.
use rocket::{
    figment::{
        util::map,
        value::{Map, Value},
    },
    fs::FileServer,
};

use rocket_sync_db_pools::{database, diesel};
// My modules...
mod models;
mod routes;
mod schema;
#[cfg(test)]
mod tests;

use routes::routes::{delete_ride, get_health, get_ride, post_ride, post_ride_data};

// Create our DB struct...
#[database("rides_db")]
pub struct RidesDb(diesel::PgConnection);

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
        .mount("/", FileServer::from("../client"))
        .mount(
            "/api/",
            routes![get_ride, get_health, post_ride, post_ride_data, delete_ride],
        )
}
