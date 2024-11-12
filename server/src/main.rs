#[macro_use]
extern crate rocket;

// Build in stuff...
use std::{env, fs, path::Path, vec};

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

    // Establish our config figment.
    let figment = rocket::Config::figment().merge(("databases", map!["rides_db" => db]));

    // Check to make sure our temp directory exists, if it does not, create it!
    match figment.find_value("temp_dir") {
        Ok(val) => {
            if let Some(val_path) = val.as_str() {
                let path = Path::new(val_path);
                if !path.exists() {
                    match fs::create_dir_all(path) {
                        Ok(_) => {
                            println!("Creating temp_dir at {}", val_path);
                        }
                        Err(e) => {
                            println!("Error temp_dir path at {}", val_path);
                            println!("{}", e);
                        }
                    }
                }
            } else {
            }
        }
        Err(e) => {
            println!("Error temp_dir value not found in config files.");
            println!("{}", e);
        }
    }

    rocket::custom(figment)
        .attach(RidesDb::fairing())
        .mount("/", FileServer::from("../client"))
        .mount(
            "/api/",
            routes![get_ride, get_health, post_ride, post_ride_data, delete_ride],
        )
}
