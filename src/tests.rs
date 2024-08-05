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

#[test]
fn check_add_ride() {
    let example_ride = InsertableRide {
        title: "Test Ride".to_string(),
        description: "This is a test ride created by our internal tests.".to_string(),
    };

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client
        .post(uri!(super::post_ride))
        .header(ContentType::JSON)
        .body(rocket::serde::json::to_string(&example_ride).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let returned_ride_string = response.into_string().unwrap();
    let returned_ride: Ride = rocket::serde::json::from_str(&returned_ride_string)
        .expect("Failed to deserialise response into a Ride object.");

    assert_eq!(example_ride.title, returned_ride.title);
    assert_eq!(example_ride.description, returned_ride.description);
}
