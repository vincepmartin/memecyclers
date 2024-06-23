#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Serialize;

// TODO: Consider moving this to some other file.
// Use Serialize so that serde/rocket can serialize my struct into JSON.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]

// Ride struct.
struct Ride {
    id: usize,
    title: String,
    description: String,
}

// Return a list of all rides.
#[get("/ride")]
fn index() -> Json<Ride> {
    let ride = Ride {
        id: 1,
        title: String::from("First ride ever!"),
        description: String::from("Whoa, so great!  I can't believe it!"),
    };
    Json(ride)
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_ride])
}
