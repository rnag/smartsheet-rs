use smartsheet_rs::models::Row;
use smartsheet_rs::{CellGetter, ColumnMapper, RowGetter, SmartsheetApi};

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use log::error;
use serde_json::to_string_pretty;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO Update these values as needed
const SINGLE_FIND_CONDITION_EQ_COLUMN: &str = "Column2";
// value can be: String, Boolean, Number
const SINGLE_FIND_CONDITION_EQ_VALUE: f64 = 123.45;

const MULTI_FIND_CONDITION_NE_COLUMN: &str = "Column3";
const MULTI_FIND_CONDITION_NE_VALUE: &str = "Red";

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: rows_find <sheet_id>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let sheet_id = fetch_args().await?;

    let smart = SmartsheetApi::from_env()?;

    let sheet = smart.get_sheet(sheet_id).await?;

    // Create interchangeable name <-> id mappings for columns in the row
    let cols = ColumnMapper::new(&sheet.columns);

    // Create a `CellGetter` helper to find cells in a row by *Column Name*
    let _get_cell = CellGetter::from_mapper(&cols);

    // Create a `RowGetter` helper to find rows in a sheet by a condition
    // based on a *Column Name* and *Column Value*.
    let get_row = RowGetter::new(&sheet.rows, &cols);

    let default_row = Row::default();

    let start = Instant::now();

    let row = get_row
        .where_eq(
            SINGLE_FIND_CONDITION_EQ_COLUMN,
            SINGLE_FIND_CONDITION_EQ_VALUE,
        )?
        // Normally you could use `.first()?` here if you were certain about
        // finding it, but for example purposes, let's try to set a default
        // otherwise.
        .first()
        .unwrap_or(&default_row);

    let rows = get_row
        .where_ne(
            MULTI_FIND_CONDITION_NE_COLUMN,
            MULTI_FIND_CONDITION_NE_VALUE,
        )?
        .find_all()?;

    println!("Search Rows completed in {:.2?}", start.elapsed());

    println!(
        "-- FIRST ROW Where Value for {:?} = {:?} --",
        SINGLE_FIND_CONDITION_EQ_COLUMN, SINGLE_FIND_CONDITION_EQ_VALUE
    );
    println!("{}", to_string_pretty(&row)?);
    println!();

    println!(
        "-- ALL ROWS (Count: {}) Where Value for {:?} != {:?} --",
        rows.len(),
        MULTI_FIND_CONDITION_NE_COLUMN,
        MULTI_FIND_CONDITION_NE_VALUE
    );
    println!("{}", to_string_pretty(&rows)?);

    Ok(())
}
