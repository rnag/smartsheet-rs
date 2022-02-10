#![allow(warnings)]
#![warn(rust_2018_idioms)]

use log::error;
use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::CellValue::Text;
use smartsheet_rs::models::{Cell, Row};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: sheet <sheet_id>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let sheet_id = fetch_args().await?;

    let smart = smartsheet_rs::SmartsheetApi::from_token("");

    let start = Instant::now();

    let c1 = Cell {
        column_id: COL_ID,
        value: Some(Text("MY TEST".parse().unwrap())),
        ..Default::default()
    };
    let c2 = Cell {
        column_id: COL_ID_2,
        value: Some(Text("Hello World".parse().unwrap())),
        ..Default::default()
    };

    // let r = Row {
    //     cells: vec![c1, c2],
    //     ..Default::default()
    // };

    // let row = smart.add_row(sheet_id, r).await?;

    // println!("Add Row completed in {:.2?}", start.elapsed());
    // println!();

    // Print out some basic info about the sheet
    // println!("Row:   {:#?}", row);

    println!("Cell Serialize: {}", serde_json::to_string_pretty(&c1)?);
    Ok(())
}
