use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn check_health() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::get_health)).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "\"OK\"");
}
