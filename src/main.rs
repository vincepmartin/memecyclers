#[macro_use]
extern crate rocket;

use std::vec;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

// Note on serialization on the below structs.
// We use Serialize so that serde/rocket can serialize my struct into JSON.

// Ride struct.
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Ride {
    id: usize,
    title: String,
    description: String,
}

// Rides struct.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Rides {
    rides: Vec<Ride>,
}

// Return a list of all rides.
#[get("/ride")]
fn get_rides() -> Json<Rides> {
    let rides = Rides {
        rides: vec![
            Ride {
                id: 1,
                title: String::from("First ride ever!"),
                description: String::from("Whoa, so great! I can't believe it!"),
            },
            Ride {
                id: 2,
                title: String::from("Second ride ever!"),
                description: String::from("Whoa, so great! I can't believe it!"),
            },
            Ride {
                id: 3,
                title: String::from("Third ride ever!"),
                description: String::from("Whoa, so great! I can't believe it!"),
            },
            Ride {
                id: 4,
                title: String::from("Fourth ride ever!"),
                description: String::from("Whoa, so great! I can't believe it!"),
            },
        ],
    };
    Json(rides)
}

// Return a particular ride based on id.
#[get("/ride/<id>")]
fn get_ride(id: usize) -> Json<Ride> {
    let ride = Ride {
        id,
        title: String::from("First ride ever!"),
        description: String::from("Whoa, so great!  I can't believe it!"),
    };
    Json(ride)
}

// Create a new ride.
#[post("/ride", data = "<ride>")]
fn post_ride(ride: Json<Ride>) -> Json<String> {
    Json(format!("New ride: {}", ride.title))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_ride, get_rides, post_ride])
}
