#![allow(warnings)]
#![warn(rust_2018_idioms)]

use smartsheet_rs::SmartsheetApi;

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use log::error;
use serde_json::to_string_pretty;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO Update these as needed
const ROW_IDS: [u64; 2] = [3337773114124164, 7841372741494660];

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: rows_delete <sheet_id>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let sheet_id = fetch_args().await?;

    let smart = SmartsheetApi::from_env()?;

    // println!("Column Name to ID: {:#?}", cols.name_to_id);

    let start = Instant::now();

    println!("INPUT Object: {}\n", to_string_pretty(&ROW_IDS).unwrap());

    let row_ids = smart.delete_rows(sheet_id, ROW_IDS).await?;

    println!("Deleted Rows in {:.2?}", start.elapsed());
    println!();

    // Print out the IDs of each Row that were updated.
    if row_ids.result.is_empty() {
        println!("  No Rows were deleted.");
    } else {
        for row_id in row_ids.result {
            println!("  - Row ID: {}", row_id);
        }
    }

    Ok(())
}
