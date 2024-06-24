mod libs;

mod models;
use models::Ride;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use std::vec;

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
// #[post("/ride", data = "<ride>")]
// fn post_ride(ride: Json<Ride>) -> Json<String> {
//     Json(format!("New ride: {}", ride.title))
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_ride])
}
