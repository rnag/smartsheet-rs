#![allow(warnings)]
#![warn(rust_2018_idioms)]

use smartsheet_rs::models::CellValue::{Numeric, Text};
use smartsheet_rs::models::{Cell, LightPicker, Row, RowLocationSpecifier};
use smartsheet_rs::{CellFactory, ColumnMapper, SmartsheetApi};

use std::env;
use std::io::{Error, ErrorKind};
use std::ops::Deref;
use std::time::Instant;

use log::error;
use serde_json::{json, to_string_pretty};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO Update these as needed
const ROW_IDS: [u64; 2] = [3337773114124164, 7841372741494660];

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: rows_update <sheet_id>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let sheet_id = fetch_args().await?;

    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let index_result = smart.list_columns(sheet_id).await?;

    let cols = ColumnMapper::new(&index_result.data);
    let make = CellFactory::new(&cols);

    // println!("Column Name to ID: {:#?}", cols.name_to_id);

    // Create the `Cell` objects to update here.

    let start = Instant::now();

    // Column 1 - Type: TEXT / NUMBER
    let mut c1 = make
        .cell("Primary Column", "New Value")?
        // This disables validation in the case of a dropdown, so it's not ideal.
        .with_strict(false);

    // Column 2 - Type: TEXT / NUMBER
    let c2 = make.cell("Column2", "")?;

    // Column 3 - Type: SYMBOLS -> LIGHT PICKER (Red, Yellow, Green, Blue, Gray)
    let c3 = make.cell("Column3", LightPicker::Green)?;

    // Column 4 - Type: CHECKBOX *or* SYMBOLS -> STAR/FLAG
    let c4 = make.cell("Column4", false)?;

    // Column 5 - Type: DROPDOWN (MULTI SELECT)
    let c5 = make.multi_picklist_cell("Column5", &["Three...", "Two...", "And ONE"])?;

    // Column 6 - Type: CONTACT LIST (MULTI SELECT)
    let c6 = make.multi_contact_cell("Column6", &["abc@xyz.org".into()])?;

    // Column 7 - Type: HYPERLINK, /w URL
    let c7 = make.url_hyperlink_cell("Column7", "UPDATED Link", "https://google-world.com")?;

    let cells = [c1, c2, c3, c4, c5, c6, c7];

    // We can create a Row with ID and Cells like so.
    let row1 = Row::with_id_and_cells_slice(ROW_IDS[0], &cells);
    // Or we could also create one like this...
    let row2 = Row::from(&cells).id(ROW_IDS[1]);

    println!("Created cells in {:.2?}", start.elapsed());

    let start = Instant::now();

    println!("INPUT Object: {}\n", to_string_pretty(&row1).unwrap());

    let mut rows = [row1, row2];
    // Location specifier: this is optional
    rows.to_top(true);

    let rows = smart
        // alternatively:
        //   .update_rows(sheet_id, [row1, row2].to_bottom(true))
        .update_rows_with_params(sheet_id, rows, false, true)
        .await?;

    println!("Updated Rows in {:.2?}", start.elapsed());
    println!();

    // Print out the IDs of each Row that were updated.
    for row in rows.result {
        println!("  - Row ID: {}", row.id);
    }

    Ok(())
}
