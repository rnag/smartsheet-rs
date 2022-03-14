#![deny(warnings)]
#![allow(deprecated)]
#![warn(rust_2018_idioms)]

use log::error;
use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::{Column, Row};
use smartsheet_rs::{CellGetter, ColumnMapper};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_args() -> Result<String> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_name) => Ok(sheet_name),
        None => {
            let error_msg = "Usage: sheet_by_name <sheet_name>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let sheet_name = fetch_args().await?;

    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let start = Instant::now();

    let sheet = smart.get_sheet_by_name(&sheet_name).await?;

    println!("Get Sheet By Name completed in {:.2?}", start.elapsed());
    println!();

    // Print out some basic info about the sheet
    println!("Sheet Name:   {}", sheet.name);
    println!("Sheet ID:     {}", sheet.id);
    println!("Row Count:    {}", sheet.total_row_count);
    println!("Columns:      {}", sheet.columns.len());
    println!("Created At:   {}", sheet.created_at);
    println!("Read Only:    {:?}", sheet.read_only);

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
    println!("Column Names and Values");
    println!("---");

    for (col_name, _col_id) in &cols.name_to_id {
        if let Ok(cell) = get_cell.by_name(row, col_name) {
            println!("Column Name: {}", col_name);
            // Print out the cell value
            if let Ok(value) = cell.value_as_str() {
                println!("Value (STRING): {:#?}", value);
            } else if let Ok(value) = cell.value_as_bool() {
                println!("Value (BOOL):   {:#?}", value);
            } else if let Ok(value) = cell.value_as_f64() {
                println!("Value (NUMBER): {:#?}", value);
            }

            // Print out the cell display value
            if let Ok(display_value) = cell.display_value_as_str() {
                println!("Display Value: {:#?}", display_value);
            } else {
                println!("Display Value: {:?}", cell.display_value);
            }

            // Print out the cell link, if it's set
            if let Ok(link) = cell.link_url() {
                println!("Hyperlink URL: {}", link);
            }

            println!();
        }
    }

    println!("Print cell values completed in {:?}", start.elapsed());

    Ok(())
}
