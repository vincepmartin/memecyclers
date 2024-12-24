use crate::models::{ApiResponse, InsertableRide, Ride};
use crate::rocket;
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

    // start
    write!(form_data, "--{}\r\n", boundary).unwrap();

    // Title
    write!(
        form_data,
        "Content-Disposition: form-data; name=\"title\"\r\n"
    )
    .unwrap();
    write!(form_data, "\r\n").unwrap();
    write!(form_data, "Test form no data").unwrap();
    write!(form_data, "\r\n").unwrap();

    // Description
    write!(form_data, "--{}\r\n", boundary).unwrap();
    write!(
        form_data,
        "Content-Disposition: form-data; name=\"description\"\r\n"
    )
    .unwrap();
    write!(form_data, "\r\n").unwrap();
    write!(form_data, "Description test information.").unwrap();
    write!(form_data, "\r\n").unwrap();

    write!(form_data, "--{}--\r\n", boundary).unwrap();

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let request = client
        .post("/api/ride_data/")
        .header(ContentType::new("multipart", "form-data; boundary=GADGET"))
        .body(&form_data);

    let response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
}

// #[test]
// fn test_multipart_form_with_files() {
//     // **************************
//     // Test our multipart form.
//     // **************************
//     // let ride_data_example = RideData {
//     //     title: "ride_data_test".to_string(),
//     //     description: "ride_data_description".to_string(),
//     //     data: None, // We are not using data here...
//     // };
//
//     // // We are instead using it here...
//     // // Load our test file.
//     // let test_file_path = "./storage/test_file.jpg";
//     // let image_data = fs::read(test_file_path).expect("Failed to load test file.");
//
//     // // Craft a string that represents a Form with our test ride.
//     // // This feels a little gross, but hopefully it works.
//     // let form_boundary = "--boundary_rando_123";
//     // let form_data = format!(
//     //     "{}\nContent-Disposition: form-data; name=\"title\"\n\n{}\n{}\
//     //     \nContent-Disposition: form-data; name=\"description\"\n\n{}\n{}\
//     //     \nContent-Disposition: form-data; name=\"data\"; filename=\"test_file.jpg\"\n\
//     //     Content-Type: image/jpeg\n\n{}\n{}",
//     //     form_boundary,
//     //     "ride_data_test", // title
//     //     form_boundary,
//     //     "ride_data_description", // description
//     //     form_boundary,
//     //     String::from_utf8_lossy(&image_data), // fake file data
//     //     form_boundary
//     // );
//     // println!("*** Here is our stupid form that we are going to send...");
//     // println!("{}", form_data);
//     // println!("***");
//
//     let test_file_path = "./test/request_two_files.txt";
//     let form_data = fs::read(test_file_path).expect("Failed to load test request file.");
//
//     let client = Client::tracked(rocket()).expect("valid rocket instance");
//     let request = client
//         .post("/api/ride_data/")
//         // TODO Consider changing me.
//         .header(ContentType::new(
//             "multipart",
//             "form-data; boundary=boundary_rando_123",
//         ))
//         .body(&form_data);
//
//     println!("Our request!!!");
//
//     let response = request.dispatch();
//     println!(
//         "********************\n
//         ***** RESPONSE *****\n{}",
//         String::from_utf8(form_data).expect("Invalid UTF-8 in test data file.")
//     );
//     assert_eq!(response.status(), Status::Ok);
// }

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
