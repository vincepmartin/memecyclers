use fitparser::FitDataRecord;
use std::fs::File;
use std::io::Error;

// Convert into GeoJSON
pub fn storage_geo_json_in_db() {}

// Get GeoJSON from our fit file.
pub fn get_geo_json_from_fit(fit_file_path: String) -> Result<Vec<FitDataRecord>, Error> {
    println!(
        "Parsing FIT files using Profile version: {}",
        fitparser::profile::VERSION
    );

    let mut fp = File::open(fit_file_path)?;

    // let fit_file_records = fitparser::from_reader(&mut fp).map_err
    let fit_data_records =
        fitparser::from_reader(&mut fp).map_err(|e| Error::other(e.to_string()))?;

    println!(
        "We have {} Vec records in our FitDataRecords",
        fit_data_records.len()
    );

    for r in &fit_data_records {
        println!("{:#?}", r);
    }

    Ok(fit_data_records)
}
