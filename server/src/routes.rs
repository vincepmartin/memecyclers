pub mod routes {
    use crate::models::{InsertableRide, Ride, RideData};
    use crate::rocket::{form::Form, serde::json::Json};
    use crate::schema;
    use crate::RidesDb;
    use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
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
        use schema::rides::dsl::*;
        let result = conn
            .run(move |conn| diesel::insert_into(rides).values(&*ride).get_result(conn))
            .await;

        match result {
            Ok(ride) => Some(Json(ride)),
            Err(_) => None,
        }
    }

    // Create a new ride with an attached file.
    #[post("/ride_data", data = "<ride_form>")]
    pub async fn post_ride_data(conn: RidesDb, mut ride_form: Form<RideData<'_>>) -> Json<String> {
        println!("**** POSTING RIDE WITH DATA ****");
        println!("{}", ride_form.title);
        println!("{}", ride_form.description);

        match &mut ride_form.data {
            Some(d) => {
                println!("We have a file attachment...");
                let tmp_file_path = "storage";
                let tmp_file_name = Uuid::new_v4();
                let tmp_file_ext = "jpg";
                let full_file_path_and_name = if let Some(form_file_name) = d.name() {
                    println!("We have a file name in the submitted form!");
                    format!(
                        "{}/{}_{}.{}",
                        tmp_file_path,
                        tmp_file_name.to_string(),
                        form_file_name,
                        tmp_file_ext
                    )
                } else {
                    println!("We do not have a file name in the submitted form!");
                    format!(
                        "{}/{}.{}",
                        tmp_file_path,
                        tmp_file_name.to_string(),
                        tmp_file_ext
                    )
                };

                match d.persist_to(&full_file_path_and_name).await {
                    // We can use the '_' to basically ignore this value...
                    Ok(_) => {
                        println!("Saved file to {}", full_file_path_and_name);
                    }
                    Err(error) => {
                        println!("Failed to save file to {}", full_file_path_and_name);
                        println!("{}", error.to_string());
                    }
                }
            }
            None => {
                println!("We have no file attachment...");
            }
        }

        // TODO: Convert the form data into an InsertableRide and put it into the DB.
        // TODO: Save the file somewhere, generate a pointer, and then update the db.
        return Json("OK".to_string());
    }
}
