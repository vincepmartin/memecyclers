use crate::{
    models::{InsertableRide, InsertableRideFile, Ride, RideFile},
    schema, RidesDb,
};

use diesel::{
    result::Error, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper,
};

// TODO: Implement get_ride(ride_id)
// TODO: Use in routes.rs
// TODO: Test.
// Get the Ride via its ID from the db.
pub async fn get_ride(conn: &RidesDb, ride_id: i32) -> Result<Ride, Error> {
    use schema::rides::dsl::*;
    conn.run(move |conn| {
        rides
            .filter(id.eq(ride_id))
            .select(Ride::as_select())
            .first(conn)
    })
    .await
}

// TODO: Implement get_ride_file(ride_id)
// TODO: Use in routes.rs
// TODO: Test.
// Get files associated with a ride id.
pub async fn get_ride_file(conn: &RidesDb, for_ride_id: i32) -> Result<Vec<RideFile>, Error> {
    use schema::ride_files::dsl::*;
    conn.run(move |conn| {
        ride_files
            .filter(ride_id.eq(for_ride_id))
            .load::<RideFile>(conn)
    })
    .await
}

// Save an InsertableRide to the DB.
pub async fn add_insertable_ride(conn: &RidesDb, ride: &InsertableRide) -> QueryResult<Ride> {
    use schema::rides::dsl::*;
    let ride = ride.clone();
    conn.run(move |conn| {
        diesel::insert_into(rides)
            .values(&ride)
            .get_result::<Ride>(conn)
    })
    .await
}

// Save an InsertableRideFile to the DB.
pub async fn add_insertable_ride_file(
    conn: &RidesDb,
    insertable_ride_file: &InsertableRideFile,
) -> Result<usize, Error> {
    use schema::ride_files::dsl::*;
    let insertable_ride_file = insertable_ride_file.clone();
    conn.run(move |conn| {
        diesel::insert_into(ride_files)
            .values(insertable_ride_file)
            .execute(conn)
    })
    .await
}
