pub mod routes {
    use crate::models::{
        ApiResponse, InsertableRide, InsertableRideFile, Ride, RideData, RideFile, RideWithFiles,
    };
    use crate::rocket::{form::Form, http::Status, serde::json::Json};
    use crate::schema;
    use crate::RidesDb;
    use diesel::{
        result::Error, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper,
    };
    use uuid::Uuid;

    // Return a particular ride based on id.
    #[get("/ride/<ride_id>")]
    pub async fn get_ride(
        conn: RidesDb,
        ride_id: i32,
    ) -> Result<Json<ApiResponse<RideWithFiles>>, Status> {
        use schema::rides::dsl::*;

        // Our first query gets the ride itself from the DB.
        let ride_query = conn
            .run(move |conn| {
                rides
                    .filter(id.eq(ride_id))
                    .select(Ride::as_select())
                    .first(conn)
            })
            .await;

        match ride_query {
            Ok(ride) => {
                // Our second query returns the ride_files that are associated with the item
                // returned in the first query.
                use schema::ride_files::dsl::*;
                let ride_files_query = conn
                    .run(move |conn| {
                        ride_files
                            .filter(ride_id.eq(ride.id))
                            .load::<RideFile>(conn)
                    })
                    .await;
                match ride_files_query {
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
    pub async fn delete_ride(
        conn: RidesDb,
        ride_id: i32,
    ) -> Result<Json<ApiResponse<String>>, Status> {
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
        return Json("OK".to_string());
    }

    // TODO: Implement this.
    // Get a list of all rides in the DB.
    // #[get("/ride")]
    // fn get_all_ride_ids() -> Json<Vec<Ride>> {}

    // Create a new ride.
    #[post("/ride", format = "json", data = "<ride>")]
    pub async fn post_ride(
        conn: RidesDb,
        ride: Json<InsertableRide>,
    ) -> Result<Json<ApiResponse<Ride>>, Status> {
        let result = add_insertable_ride(&conn, ride.into_inner()).await;

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

        // Handle creation of the InsertableRide.
        let temp_insertable_ride = InsertableRide {
            title: ride_form.title.clone(),
            description: ride_form.description.clone(),
        };

        let ride_result = add_insertable_ride(&conn, temp_insertable_ride).await;
        let _ride = match ride_result {
            Ok(ride) => {
                println!("Added a ride.");
                println!("{:?}", ride);

                // Handle file attachments.
                match &mut ride_form.data {
                    Some(data_files) => {
                        for file in data_files {
                            let tmp_file_path = "storage";
                            let tmp_file_name = Uuid::new_v4().to_string();
                            let tmp_file_ext = "jpg";

                            let full_file_path_and_name = if let Some(form_file_name) = &file.name()
                            {
                                format!(
                                    "{}/{}_{}.{}",
                                    tmp_file_path, tmp_file_name, form_file_name, tmp_file_ext
                                )
                            } else {
                                format!("{}/{}.{}", tmp_file_path, tmp_file_name, tmp_file_ext)
                            };

                            // We can use the '_' to basically ignore this value...  As we don't
                            // handle anything from this persist_to function.
                            let _ = match file.persist_to(&full_file_path_and_name).await {
                                Ok(_) => {
                                    println!("Saved file to {}", full_file_path_and_name);
                                    let insertable_ride_file = InsertableRideFile {
                                        description: "temp_description".to_string(),
                                        ride_id: ride.id,
                                        file_name: full_file_path_and_name,
                                        file_type: "ride".to_string(),
                                    };

                                    // Persist the InsertableRideFile
                                    let result =
                                        add_insertable_ride_file(&conn, insertable_ride_file).await;

                                    match result {
                                        Ok(count) => {
                                            println!("{} InsertableRideFile Inserted", count);
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
                                    println!("{}", e.to_string());
                                    return Err(Status::InternalServerError);
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

    // Save an InsertableRide to the DB.
    async fn add_insertable_ride(conn: &RidesDb, ride: InsertableRide) -> QueryResult<Ride> {
        use schema::rides::dsl::*;
        let result = conn
            .run(move |conn| {
                diesel::insert_into(rides)
                    .values(&ride)
                    .get_result::<Ride>(conn)
            })
            .await;

        return result;
    }

    // Save an InsertableRideFile to the DB.
    async fn add_insertable_ride_file(
        conn: &RidesDb,
        insertable_ride_file: InsertableRideFile,
    ) -> Result<usize, Error> {
        use schema::ride_files::dsl::*;
        let result = conn
            .run(move |conn| {
                diesel::insert_into(ride_files)
                    .values(&insertable_ride_file)
                    .execute(conn)
            })
            .await;
        result
    }
}
