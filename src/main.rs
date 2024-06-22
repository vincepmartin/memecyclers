#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Serialize;

use std::fmt;

// Use Serialize so that serde/rocket can serialize my struct into JSON.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
// Ride struct.
struct Ride {
    title: String,
    description: String,
}

// This is supposed to be my to string...
impl fmt::Display for Ride {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // What the heck is a Result!?
        write!(
            f,
            "title: {}, description: {}",
            self.title, self.description
        )
    }
}

#[get("/")]
fn index() -> Json<Ride> {
    let ride = Ride {
        title: String::from("First ride ever!"),
        description: String::from("Whoa, so great!  I can't believe it!"),
    };
    Json(ride)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
