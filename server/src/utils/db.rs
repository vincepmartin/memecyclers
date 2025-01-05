use crate::{
    models::{InsertableRide, InsertableRideFile, Ride},
    schema, RidesDb,
};
use diesel::{result::Error, QueryResult, RunQueryDsl};

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
