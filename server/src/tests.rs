use crate::models::{ApiResponse, InsertableRide, Ride, RideData};
use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn check_health() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/health").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "\"OK\"");
}

// I know this is lazy...  However, I want to work on my app, not testing yet.
// TODO: Come back and make this better.
#[test]
fn test_everything() {
    // 1. Add a test ride to the database.
    let (client, response_from_input_ride) = add_ride();

    // 2. GET the added ride
    let get_returned_ride = get_added_ride(response_from_input_ride, &client);

    // 3. DELETE the added ride
    delete_ride(&client, get_returned_ride);

    // 4. Verify DELETE ride is actually gone from the DB.
    verify_deleted_ride(client);
}

// Helper functions used in testing.
fn verify_deleted_ride(client: Client) {
    let response = client
        .get("/api/ride/{get_returned_ride.id}")
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(
        response.status(),
        Status::BadRequest,
        "Deleted item still exists."
    );
}

fn delete_ride(client: &Client, get_returned_ride: ApiResponse<Ride>) {
    let response = client
        .delete(format!("/api/ride/{}", get_returned_ride.data.id))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok, "Delete failed.");
}

fn get_added_ride(
    response_from_input_ride: ApiResponse<Ride>,
    client: &Client,
) -> ApiResponse<Ride> {
    println!(
        "**** GETTING RIDE WITH ID {} ****",
        response_from_input_ride.data.id
    );
    let response = client
        .get(format!("/api/ride/{}", response_from_input_ride.data.id))
        .header(ContentType::JSON)
        .dispatch();

    let get_returned_ride_string = response.into_string().unwrap();

    println!("*** Returned ride string... {}", get_returned_ride_string);

    let get_returned_ride: ApiResponse<Ride> =
        rocket::serde::json::from_str(&get_returned_ride_string)
            .expect("Failed to deserialize response into a Ride struct.");

    assert_eq!(
        get_returned_ride.data, response_from_input_ride.data,
        "POST Ride and GET Ride are not equal!"
    );
    get_returned_ride
}

fn add_ride() -> (Client, ApiResponse<Ride>) {
    let input_ride = InsertableRide {
        title: "test_ride_title".to_string(),
        description: "test_ride_description.".to_string(),
    };

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client
        .post("/api/ride/")
        .header(ContentType::JSON)
        .body(rocket::serde::json::to_string(&input_ride).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_for_input_ride_string = response.into_string().unwrap();
    println!("{}", response_for_input_ride_string);

    let response_from_input_ride: ApiResponse<Ride> =
        rocket::serde::json::from_str(&response_for_input_ride_string)
            .expect("Failed to deserialise response into a Ride object.");

    assert_eq!(input_ride.title, response_from_input_ride.data.title);
    assert_eq!(
        input_ride.description,
        response_from_input_ride.data.description
    );
    (client, response_from_input_ride)
}

// #[test]
// fn test_multipart_form() {
//     // ********************
//     // 1. Test our multipart form.
//     // ********************
//     let ride_data_example = RideData {
//         title: "ride_data_test".to_string(),
//         description: "ride_data_description".to_string(),
//         data: //TODO: Figure out how to get binary file here.
//     };
//
//     let client = Client::tracked(rocket()).expect("valid rocket instance");
//     let response = client
//         .post("/api/ride/")
//         .header(ContentType::Form)
//         // TODO: The next bit is obviously not correct...
//         .body(rocket::serde::json::to_string(&ride_data_example).unwrap())
//         .dispatch();
//
//     assert_eq!(response.status(), Status::Ok);
// }
