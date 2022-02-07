#![deny(warnings)]
#![warn(rust_2018_idioms)]

use log::error;
use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::{Level, Row, RowExcludeFlags, RowIncludeFlags};
use smartsheet_rs::{CellGetter, ColumnMapper};

/// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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

    let include = vec![
        RowIncludeFlags::Columns,
        RowIncludeFlags::ColumnType,
        RowIncludeFlags::Attachments,
        RowIncludeFlags::Discussions,
        RowIncludeFlags::ObjectValue,
        RowIncludeFlags::RowPermalink,
        RowIncludeFlags::RowWriterInfo,
    ];

    let exclude = vec![
        RowExcludeFlags::NonExistentCells,
        RowExcludeFlags::LinkInFromCellDetails,
        RowExcludeFlags::LinkOutToCellDetails,
    ];

    let level = Level::MultiPicklist;

    // Get row data, and pass `include=columns` in the request
    let row = smart
        .get_row_with_params(sheet_id, row_id, Some(include), Some(exclude), Some(level))
        .await?;

    println!("Get Row with Params completed in {:?}", start.elapsed());
    println!();

    // Print out some basic info about the row
    println!("Row ID:     {}", row.id);
    println!("Row Number: {}", row.row_number);
    println!("Created At: {}", row.created_at);

    // Assert desired row properties are populated in the response
    assert!(
        row.permalink.as_ref().is_some(),
        "Expected `permalink` to be populated"
    );
    assert!(
        row.attachments.is_some(),
        "Expected `attachments` to be populated"
    );

    println!("Permalink:  {}", row.permalink.as_ref().unwrap());
    if let Some(attachments) = &row.attachments {
        println!("Attachments: {:#?}", attachments);
    }

    // Create `name` <-> `id` mappings for columns in the row
    let cols = ColumnMapper::new(&row.columns);

    // Create a `CellGetter` helper to find cells in a row by `name`
    let _get_cell = CellGetter::from_mapper(&cols);

    // Uncomment to display the column name + values of each cell in the row
    // print_column_names_and_cell_values(&row, &cols, _get_cell).await?;

    Ok(())
}

/// For each cell in the row, print out columns name and cell values
#[allow(dead_code)]
async fn print_column_names_and_cell_values<'a>(
    row: &Row,
    cols: &ColumnMapper<'a>,
    get_cell: CellGetter<'a>,
) -> Result<()> {
    let start = Instant::now();

    println!();
    println!("Column Names and Values");
    println!("---");

    for (col_name, _col_id) in &cols.name_to_id {
        if let Some(cell) = get_cell.by_name(row, col_name) {
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

            if let Some(obj_value) = &cell.object_value {
                println!("Object Value: {:#?}", obj_value);
            }

            println!();
        }
    }

    println!("Print cell values completed in {:?}", start.elapsed());

    Ok(())
}
