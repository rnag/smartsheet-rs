#![deny(warnings)]
#![warn(rust_2018_idioms)]

use log::error;
use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::{Column, Row};
use smartsheet_rs::{CellGetter, ColumnMapper};
#[macro_use]
extern crate log;

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
    sensible_env_logger::init!();

    let sheet_id = fetch_args().await?;

    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let start = Instant::now();

    let sheet = smart.get_sheet(sheet_id).await?;

    trace!("Get Sheet completed in {:.2?}", start.elapsed());
    println!();

    // Print out some basic info about the sheet
    debug!("Sheet Name:   {}", sheet.name);
    debug!("Sheet ID:     {}", sheet.id);
    debug!("Row Count:    {}", sheet.total_row_count);
    debug!("Columns:      {}", sheet.columns.len());
    debug!("Created At:   {}", sheet.created_at);
    debug!("Read Only:    {:?}", sheet.read_only);

    // Assert some sheet properties are *not* populated in the response
    // by default.
    assert!(sheet.owner.is_none(), "Expected `owner` to be omitted");
    assert!(
        sheet.owner_id.is_none(),
        "Expected `owner_id` to be omitted"
    );
    assert!(sheet.source.is_none(), "Expected `source` to be omitted");

    let first_row = sheet.rows.first();

    if let Some(row) = first_row {
        println!("First Row ID: {}", row.id);
    }

    // Uncomment to display the column name + values of each cell in the row
    // print_column_names_and_cell_values(first_row, &sheet.columns).await?;

    Ok(())
}

/// For each cell in the row, print out columns name and cell values
// noinspection DuplicatedCode
#[allow(dead_code)]
async fn print_column_names_and_cell_values(row: Option<&Row>, cols: &Vec<Column>) -> Result<()> {
    if row.is_none() {
        return Ok(());
    }

    let row = row.unwrap();

    // Create `name` <-> `id` mappings for columns in the sheet
    let cols = ColumnMapper::new(cols);

    // Create a `CellGetter` helper to find cells in a row by `name`
    let get_cell = CellGetter::new(&cols);

    let start = Instant::now();

    println!();
    debug!("Column Names and Values");
    debug!("---");

    for (col_name, _col_id) in &cols.name_to_id {
        if let Ok(cell) = get_cell.by_name(row, col_name) {
            debug!("Column Name: {}", col_name);
            // Print out the cell value
            if let Ok(value) = cell.value_as_str() {
                debug!("Value (STRING): {:#?}", value);
            } else if let Ok(value) = cell.value_as_bool() {
                debug!("Value (BOOL):   {:#?}", value);
            } else if let Ok(value) = cell.value_as_f64() {
                debug!("Value (NUMBER): {:#?}", value);
            }

            // Print out the cell display value
            if let Ok(display_value) = cell.display_value_as_str() {
                debug!("Display Value: {:#?}", display_value);
            } else {
                debug!("Display Value: {:?}", cell.display_value);
            }

            // Print out the cell link, if it's set
            if let Ok(link) = cell.link_url() {
                debug!("Hyperlink URL: {}", link);
            }

            println!();
        }
    }

    debug!("Print cell values completed in {:?}", start.elapsed());

    Ok(())
}
