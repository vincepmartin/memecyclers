use crate::models::{ApiResponse, InsertableRide, Ride};
use crate::rocket;
use crate::utils::get_geo_json_from_fit;
use std::fs;
use std::io::Write;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn check_health() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/health").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "\"OK\"");
}

#[test]
fn test_get_all_rides() {
    // 0. We have to make sure we have a ride in the DB.
    let input_ride = InsertableRide {
        title: "test_ride_title".to_string(),
        description: "test_ride_description.".to_string(),
    };

    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // Add item 1.
    client
        .post("/api/ride/")
        .header(ContentType::JSON)
        .body(rocket::serde::json::to_string(&input_ride).unwrap())
        .dispatch();

    // Add item 2.
    client
        .post("/api/ride/")
        .header(ContentType::JSON)
        .body(rocket::serde::json::to_string(&input_ride).unwrap())
        .dispatch();

    // 1. Make our request to get all rides.
    let response = client
        .get("/api/rides/")
        .header(ContentType::JSON)
        .dispatch();

    match response.into_string() {
        Some(response_string) => {
            let returned_rides: ApiResponse<Vec<Ride>> =
                rocket::serde::json::from_str(&response_string).unwrap();

            println!(
                "test_get_all_rides(): Number or rides returned: {}",
                returned_rides.data.len()
            );
            // 2. Ensure that at least one of them exists...
            assert!(returned_rides.data.len() > 1);
        }
        None => {
            // No clue what to do here.
            println!("test_get_all_rides(): No rides returned from DB.");
        }
    }
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

#[test]
fn test_multipart_without_files() {
    let mut form_data: Vec<u8> = Vec::new();
    let boundary = "GADGET";

    // Add our fields.
    add_form_field("title", "Test ride without data", boundary, &mut form_data);
    add_form_field(
        "description",
        "This is the description of the test ride without data attached.  We have no files!",
        boundary,
        &mut form_data,
    );

    // End of our form.
    write!(form_data, "--{}--\r\n", boundary).unwrap();

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let request = client
        .post("/api/ride_data/")
        .header(ContentType::new("multipart", "form-data; boundary=GADGET"))
        .body(&form_data);

    let response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_multipart_form_with_files() {
    let mut form_data: Vec<u8> = Vec::new();
    let boundary = "GADGET";

    // Add our fields.
    add_form_field("title", "Test ride without data", boundary, &mut form_data);
    add_form_field(
        "description",
        "This is the description of the test ride without data attached.  We have no files!",
        boundary,
        &mut form_data,
    );

    let test_file_path = "./storage/test_file.jpg";
    let image_data = fs::read(test_file_path).expect("Failed to load test file.");

    add_form_field_binary("image_1.jpg", &image_data, boundary, &mut form_data);

    // End of our form.
    write!(form_data, "--{}--\r\n", boundary).unwrap();

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let request = client
        .post("/api/ride_data/")
        .header(ContentType::new("multipart", "form-data; boundary=GADGET"))
        .body(&form_data);

    let response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn fit_file_converts_to_json() {
    let test_file_path = "./storage/test.fit".to_string();
    let results = get_geo_json_from_fit(test_file_path).expect("Can't load test fit file.");
    assert!(results.len() > 1);
}

// *** Helper functions used in testing. ***
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

fn delete_ride(client: &Client, get_returned_ride: ApiResponse<Ride>) {
    let response = client
        .delete(format!("/api/ride/{}", get_returned_ride.data.id))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok, "Delete failed.");
}

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

// TODO: Consider creating a Struct for this and implement some fields to create values?
// Maybe you can store the name and values in a Vec.
fn add_form_field(name: &str, value: &str, boundary: &str, form_data: &mut Vec<u8>) {
    // Create our boundary
    write!(form_data, "--{}\r\n", boundary).unwrap();
    // Name
    write!(
        form_data,
        "Content-Disposition: form-data; name=\"{}\"\r\n",
        name
    )
    .unwrap();
    write!(form_data, "\r\n").unwrap();
    // Value
    write!(form_data, "{}", value).unwrap();
    write!(form_data, "\r\n").unwrap();
}

fn add_form_field_binary(name: &str, value: &[u8], boundary: &str, form_data: &mut Vec<u8>) {
    // Create our boundary
    write!(form_data, "--{}\r\n", boundary).unwrap();
    // Name
    write!(
        form_data,
        "Content-Disposition: form-data; name=\"{}\"\r\n",
        name
    )
    .unwrap();
    write!(form_data, "Content-Type: image/jpeg\r\n").unwrap();
    write!(form_data, "\r\n").unwrap();
    // Value
    write!(form_data, "{}", String::from_utf8_lossy(value)).unwrap();
    write!(form_data, "\r\n").unwrap();
}
