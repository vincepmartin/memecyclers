pub mod routes {
    use crate::models::{InsertableRide, Ride};
    use crate::rocket::serde::json::Json;
    use crate::schema;
    use crate::RidesDb;
    use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

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
        return Json(String::from("OK"));
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
}
