use super::models::{InsertableRide, Ride};
use super::rocket;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn check_health() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::get_health)).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "\"OK\"");
}

// I know this is lazy...  However, I want to work on my app, not testing yet.
// TODO: Come back and make this better.
#[test]
fn test_everything() {
    // ********************
    // 1. Add a test ride.
    // ********************

    let insertable_example_ride = InsertableRide {
        title: "test_ride_title".to_string(),
        description: "test_ride_description.".to_string(),
    };

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client
        .post(uri!(super::post_ride))
        .header(ContentType::JSON)
        .body(rocket::serde::json::to_string(&insertable_example_ride).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let post_returned_ride_string = response.into_string().unwrap();
    let post_returned_ride: Ride = rocket::serde::json::from_str(&post_returned_ride_string)
        .expect("Failed to deserialise response into a Ride object.");

    assert_eq!(insertable_example_ride.title, post_returned_ride.title);
    assert_eq!(
        insertable_example_ride.description,
        post_returned_ride.description
    );

    // ********************
    // 2. GET the added ride
    // ********************
    println!("**** GETTING RIDE WITH ID {} ****", post_returned_ride.id);
    let response = client
        .get(format!("/ride/{}", post_returned_ride.id))
        .header(ContentType::JSON)
        .dispatch();

    let get_returned_ride_string = response.into_string().unwrap();
    println!("{}", get_returned_ride_string);

    let get_returned_ride: Ride = rocket::serde::json::from_str(&get_returned_ride_string)
        .expect("Failed to deserialize response into a Ride struct.");

    assert_eq!(
        get_returned_ride, post_returned_ride,
        "POST Ride and GET Ride are not equal!"
    );

    // ********************
    // 3. DELETE the added ride
    // ********************

    let response = client
        .delete(format!("/ride/{}", get_returned_ride.id))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok, "Delete failed.");

    // ********************
    // 4. Verify DELETE ride is actually gone from the DB.
    // ********************
    let response = client
        .get("/ride/{get_returned_ride.id}")
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(
        response.status(),
        Status::BadRequest,
        "Deleted item still exists."
    );
}
