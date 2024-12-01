pub mod routes {
    use crate::models::{InsertableRide, InsertableRideFile, Ride, RideData};
    use crate::rocket::{form::Form, serde::json::Json};
    use crate::schema;
    use crate::RidesDb;
    use diesel::{
        result::Error, ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl,
        SelectableHelper,
    };
    use uuid::Uuid;

    // Return a particular ride based on id.
    #[get("/ride/<ride_id>")]
    pub async fn get_ride(conn: RidesDb, ride_id: i32) -> Option<Json<Ride>> {
        use crate::schema::rides::dsl::*;
        let result = conn
            .run(move |conn| {
                rides
                    .filter(id.eq(ride_id))
                    .select(Ride::as_select())
                    .first(conn)
                    .optional()
            })
            .await;

        match result {
            Ok(Some(ride)) => Some(Json(ride)),
            _ => None,
        }
    }

    // Delete a particular ride based on id.
    #[delete("/ride/<ride_id>")]
    pub async fn delete_ride(conn: RidesDb, ride_id: i32) -> Json<String> {
        use schema::rides::dsl::*;

        let result = conn
            .run(move |conn| diesel::delete(rides.filter(id.eq(ride_id))).execute(conn))
            .await;

        match result {
            Ok(ok) => Json(format!("{ok} ride(s) with id {ride_id} deleted.").to_string()),
            Err(error) => Json(format!("Error deleting ride {}", error)),
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
    pub async fn post_ride(conn: RidesDb, ride: Json<InsertableRide>) -> Option<Json<Ride>> {
        let result = add_insertable_ride(&conn, ride.into_inner()).await;

        match result {
            Ok(ride) => Some(Json(ride)),
            Err(_) => None,
        }
    }

    // Create a new ride with an attached file.
    #[post("/ride_data", data = "<ride_form>")]
    pub async fn post_ride_data(conn: RidesDb, mut ride_form: Form<RideData<'_>>) -> Json<String> {
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

                            match file.persist_to(&full_file_path_and_name).await {
                                // We can use the '_' to basically ignore this value...
                                Ok(_) => {
                                    println!("Saved file to {}", full_file_path_and_name);
                                    let insertable_ride_file = InsertableRideFile {
                                        description: "temp_description".to_string(),
                                        rides_id: ride.id,
                                        file_name: full_file_path_and_name,
                                        file_type: "ride".to_string(),
                                    };

                                    // Persist the InsertableRideFile
                                    let result =
                                        add_insertable_ride_file(&conn, insertable_ride_file).await;

                                    match result {
                                        Ok(count) => {
                                            println!("{} InsertableRideFile Inserted", count);
                                        }
                                        Err(e) => {
                                            println!("Error Inserting InsertableRideFile!");
                                            println!("{}", e);
                                        }
                                    }
                                }
                                Err(error) => {
                                    println!("Failed to save file to {}", full_file_path_and_name);
                                    println!("{}", error.to_string());
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
                // return Err(Json("OK"));
            }
        };

        // TODO: Convert the form data into an InsertableRide and put it into the DB.
        // TODO: Save the file somewhere, generate a pointer, and then update the db.
        return Json("OK".to_string());
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
        use schema::ride_data::dsl::*;
        let result = conn
            .run(move |conn| {
                diesel::insert_into(ride_data)
                    .values(&insertable_ride_file)
                    .execute(conn)
            })
            .await;
        result
    }
}
