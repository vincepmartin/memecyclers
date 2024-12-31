use fitparser;
use fitparser::de::{from_reader, from_reader_with_options, DecodeOption};
use std::fs::File;
use std::io::Error;

// Convert into GeoJSON
pub fn storage_geo_json_in_db() {}

// Get GeoJSON from our fit file.
pub fn get_geo_json_from_fit(fit_file_path: String) -> Result<String, Error> {
    println!(
        "Parsing FIT files using Profile version: {}",
        fitparser::profile::VERSION
    );

    match File::open(fit_file_path) {
        Ok(mut fp) => {
            // match from_reader_with_options(&mut fp, &opts) {
            match from_reader(&mut fp) {
                Ok(fit_data_records) => {
                    println!(
                        "We have {} Vec records in our FitDataRecords",
                        fit_data_records.len()
                    );
                    for r in fit_data_records {
                        println!("{:#?}", r);
                    }
                }
                Err(e) => {
                    println!("Error found while processing fit file.");
                    println!("{}", e);
                }
            };
            Ok("BURRITOS".to_string())
        }
        Err(e) => Err(e),
    }
}
