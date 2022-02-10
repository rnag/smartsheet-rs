#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::Row;
use smartsheet_rs::{CellGetter, ColumnMapper};

use log::error;
use tabled::{Header, Style, TableIteratorExt, Tabled};

/// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Tabled)]
struct TableRow<'a> {
    #[header("Row ID")]
    row_id: u64,
    #[header("Row #")]
    row_number: u64,
    #[header("Created At")]
    created_at: &'a str,
    #[header("Modified At")]
    modified_at: &'a str,
}

// noinspection DuplicatedCode
async fn fetch_single_arg(arg_pos: usize) -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(arg_pos) {
        Some(value) => Ok(value.parse::<u64>()?),
        None => {
            let error_msg = "Usage: row <sheet_id> <row_id>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

async fn fetch_args() -> Result<(u64, u64)> {
    // Some simple CLI args requirements...
    let sheet_id = fetch_single_arg(1).await?;
    let row_id = fetch_single_arg(2).await?;

    Ok((sheet_id, row_id))
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let (sheet_id, row_id) = fetch_args().await?;

    // Create Smartsheet client
    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let start = Instant::now();

    // Get row data, and pass `include=columns` in the request
    let row = smart.get_row_with_column_data(sheet_id, row_id).await?;

    println!("Get Row completed in {:.2?}", start.elapsed());
    println!();

    // Print out some basic info about the row

    let tr = TableRow {
        row_id: row.id,
        row_number: row.row_number,
        created_at: &row.created_at,
        modified_at: &row.modified_at,
    };

    println!(
        "{}",
        [tr].table()
            .with(Style::PSEUDO_CLEAN)
            // .with(Modify::new(Row(1..)).with(Alignment::left()))
            .with(Header("Row Info"))
    );

    // Assert some row properties are not populated in the response
    // by default.
    assert!(
        row.permalink.is_none(),
        "Expected `permalink` to be omitted"
    );
    assert!(
        row.attachments.is_none(),
        "Expected `attachments` to be omitted"
    );
    assert!(
        row.created_by.is_none(),
        "Expected `created_by` to be omitted"
    );

    // Create `name` <-> `id` mappings for columns in the row
    let cols = ColumnMapper::new(&row.columns);

    // Create a `CellGetter` helper to find cells in a row by `name`
    let _get_cell = CellGetter::new(&cols);

    // Uncomment to display a mapping of column name to the cell for that column
    // print_column_name_to_cell(&row, _get_cell).await?;

    // Uncomment to display the column name + values of each cell in the row
    // print_column_names_and_cell_values(&row, &cols, _get_cell).await?;

    Ok(())
}

/// For each cell in the row, print out columns name and cell values
// noinspection DuplicatedCode
#[allow(dead_code)]
async fn print_column_names_and_cell_values<'a>(
    row: &'a Row,
    cols: &ColumnMapper<'a>,
    get_cell: CellGetter<'a>,
) -> Result<()> {
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

/// Print out a mapping of *column title* to the `Cell` object in that column
// noinspection DuplicatedCode
#[allow(dead_code)]
async fn print_column_name_to_cell<'a>(row: &'a Row, get_cell: CellGetter<'a>) -> Result<()> {
    let start = Instant::now();
    let col_name_to_cell = get_cell.name_to_cell(row);
    println!("Get cell mappings completed in {:?}", start.elapsed());

    println!();
    println!("Column Name to Cell");
    println!("---");
    println!("{:#?}", col_name_to_cell);

    Ok(())
}
