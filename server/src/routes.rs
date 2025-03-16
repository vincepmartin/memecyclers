use crate::models::{
    ApiResponse, InsertableRide, InsertableRideFile, Ride, RideData, RideWithFiles,
};
use crate::rocket::{form::Form, http::Status, serde::json::Json};
use crate::schema;
use crate::utils::db::{add_insertable_ride, add_insertable_ride_file, get_ride, get_ride_file};
use crate::utils::helpers::{get_geo_json_from_fit, write_vec_to_file};
use crate::RidesDb;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

// Return a particular ride based on id.
#[get("/ride/<ride_id>")]
pub async fn fetch_ride(
    conn: RidesDb,
    ride_id: i32,
) -> Result<Json<ApiResponse<RideWithFiles>>, Status> {
    // Our first query gets the ride itself from the DB.
    match get_ride(&conn, ride_id).await {
        Ok(ride) => {
            // Our second query returns the ride_files that are associated with the item
            // returned in the first query.
            match get_ride_file(&conn, ride_id).await {
                Ok(ride_files_result) => Ok(Json(ApiResponse {
                    data: RideWithFiles {
                        id: ride.id,
                        title: ride.title,
                        description: ride.description,
                        created_date: ride.created_date,
                        ride_files: ride_files_result,
                    },
                })),
                Err(_) => Err(Status::NotFound),
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}

// Delete a particular ride based on id.
#[delete("/ride/<ride_id>")]
pub async fn delete_ride(conn: RidesDb, ride_id: i32) -> Result<Json<ApiResponse<String>>, Status> {
    use schema::rides::dsl::*;

    let result = conn
        .run(move |conn| diesel::delete(rides.filter(id.eq(ride_id))).execute(conn))
        .await;

    match result {
        Ok(ok) => Ok(Json(ApiResponse {
            data: format!("{ok} ride(s) with id {ride_id} deleted.").to_string(),
        })),
        Err(_) => Err(Status::ServiceUnavailable),
    }
}

// Health check returns OK if everything is OK.
#[get("/health")]
pub async fn get_health() -> Json<String> {
    Json("OK".to_string())
}

// TODO: Implement this.
// Get a list of all rides in the DB.
#[get("/rides")]
pub async fn get_all_rides(conn: RidesDb) -> Result<Json<ApiResponse<Vec<Ride>>>, Status> {
    // Get a list of all of our rides from the DB.
    use schema::rides::dsl::*;

    // Our first query gets the ride itself from the DB.
    let ride_query = conn
        .run(move |conn| rides.select(Ride::as_select()).load(conn))
        .await;

    match ride_query {
        Ok(all_rides) => Ok(Json(ApiResponse { data: all_rides })),
        Err(_) => Err(Status::InternalServerError),
    }
}

// Create a new ride.
#[post("/ride", format = "json", data = "<ride>")]
pub async fn post_ride(
    conn: RidesDb,
    ride: Json<InsertableRide>,
) -> Result<Json<ApiResponse<Ride>>, Status> {
    let result = add_insertable_ride(&conn, &ride.into_inner()).await;

    match result {
        Ok(ride) => Ok(Json(ApiResponse { data: ride })),
        Err(_) => Err(Status::ServiceUnavailable),
    }
}

// Create a new ride with an attached file.
#[post("/ride_data", data = "<ride_form>")]
pub async fn post_ride_data(
    conn: RidesDb,
    mut ride_form: Form<RideData<'_>>,
) -> Result<Status, Status> {
    println!("POST: RIDE WITH DATA");
    println!("{}", ride_form.title);
    println!("{}", ride_form.description);
    println!("Data field debug: {:?}", ride_form.data.is_some());

    match add_insertable_ride(
        &conn,
        &InsertableRide {
            title: ride_form.title.clone(),
            description: ride_form.description.clone(),
        },
    )
    .await
    {
        Ok(ride) => {
            println!("Added a ride.");
            println!("{:?}", ride);

            // Handle file attachments.
            match &mut ride_form.data {
                Some(data_files) => {
                    for file in data_files {
                        let tmp_file_path = "storage";
                        let tmp_file_name = Uuid::new_v4().to_string();
                        let (tmp_file_ext, tmp_file_type) = match file.content_type() {
                            Some(content_type) => {
                                let file_ext = content_type.extension().map_or("unk", |ext| {
                                    // TODO: This feels a bit janky...
                                    // ContentType will give my uploaded .fit files a .bin ext,
                                    // change this back to .fit.
                                    if ext == "bin" {
                                        "fit"
                                    } else {
                                        ext.as_str()
                                    }
                                });
                                let file_type = if content_type.is_png() || content_type.is_jpeg() {
                                    "image"
                                } else {
                                    "ride"
                                };
                                (file_ext, file_type)
                            }
                            None => ("unk", "ride"),
                        };

                        let full_file_path_and_name = if let Some(form_file_name) = &file.name() {
                            format!(
                                "{}/{}_{}.{}",
                                tmp_file_path, tmp_file_name, form_file_name, tmp_file_ext
                            )
                        } else {
                            format!("{}/{}.{}", tmp_file_path, tmp_file_name, tmp_file_ext)
                        };

                        // TODO: Write function to store the file in the DB as JSON after
                        // converting it to geoJSON.
                        // We can use the '_' to basically ignore this value...  As we don't
                        // handle anything from this persist_to function.
                        let _ = match file.persist_to(&full_file_path_and_name).await {
                            Ok(_) => {
                                println!("Saved file to {}", full_file_path_and_name);
                                // TODO: Add logic here to handle .fit files.
                                // - figure out the file type, with both options probably  being
                                let insertable_ride_file = InsertableRideFile {
                                    description: "temp_description".to_string(),
                                    ride_id: ride.id,
                                    file_name: full_file_path_and_name,
                                    file_type: tmp_file_type.to_string(),
                                };

                                // Persist the InsertableRideFile
                                match add_insertable_ride_file(&conn, &insertable_ride_file).await {
                                    Ok(count) => {
                                        println!("{} InsertableRideFile Inserted", count);
                                        if insertable_ride_file.file_type.eq("ride") {
                                            match get_geo_json_from_fit(
                                                insertable_ride_file.file_name,
                                            ) {
                                                Ok(fit_data_records) => {
                                                    // Print out all of our logs, why not?
                                                    println!("Fit file data records!");
                                                    for r in &fit_data_records {
                                                        println!("{:#?}", &r);
                                                    }

                                                    // TODO: For now just write this to a file so
                                                    // that you can investigate it to figure out
                                                    // what the heck you actually need to pull out
                                                    // of it.
                                                    match write_vec_to_file(
                                                        fit_data_records,
                                                        "output.txt",
                                                    ) {
                                                        Ok(_) => {
                                                            println!("File written!")
                                                        }
                                                        Err(e) => {
                                                            println!("We have an error {}", e);
                                                        }
                                                    }
                                                    // Insert the data you got from this into the
                                                    // DB.
                                                }
                                                Err(e) => {
                                                    println!("Error parsing FIT file!");
                                                    println!("{}", e);
                                                }
                                            };
                                        }
                                        Ok(Json(Status::Ok))
                                    }
                                    Err(e) => {
                                        println!("Error Inserting InsertableRideFile!");
                                        println!("{}", e);
                                        Err(Status::InternalServerError)
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Failed to save file to {}", full_file_path_and_name);
                                println!("{}", e);
                                Err(Status::InternalServerError)
                            }
                        };
                    }
                }
                None => {
                    println!("Creating new ride without attachment.");
                }
            }
        }
        // TODO: Handle this error, here you can pass the error back via a Responder
        // https://rocket.rs/guide/v0.5/responses/#responder
        Err(_) => {
            println!("Error adding a ride!");
        }
    };

    // Ride added?
    Ok(Status::Ok)
}
