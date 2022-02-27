#![allow(warnings)]
#![warn(rust_2018_idioms)]

use smartsheet_rs::models::CellValue::{Numeric, Text};
use smartsheet_rs::models::{Cell, Contact, LightPicker, Row, RowLocationSpecifier};
use smartsheet_rs::{CellFactory, ColumnMapper, SmartsheetApi};

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use log::error;
use serde_json::{json, to_string_pretty};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: rows_add <sheet_id>";
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

    let index_result = smart.list_columns(sheet_id).await?;

    let cols = ColumnMapper::new(&index_result.data);
    let make = CellFactory::new(&cols);

    // println!("Column Name to ID: {:#?}", cols.name_to_id);

    // Create the `Cell` objects to add here.

    let start = Instant::now();

    // Column 1 - Type: TEXT / NUMBER
    let mut c1 = make
        .cell("Primary Column", "My Test Value")?
        // This disables validation in the case of a dropdown, so it's not ideal.
        .with_strict(false);

    // Column 2 - Type: TEXT / NUMBER
    let c2 = make.cell("Column2", 123.45)?;

    // Column 3 - Type: SYMBOLS -> LIGHT PICKER (Red, Yellow, Green, Blue, Gray)
    let c3 = make.cell("Column3", LightPicker::Yellow)?;

    // Column 4 - Type: CHECKBOX *or* SYMBOLS -> STAR/FLAG
    let c4 = make.cell("Column4", true)?;

    // Column 5 - Type: DROPDOWN (MULTI SELECT)
    let c5 = make.multi_picklist_cell("Column5", &["One", "Two", "Hello, world!"])?;

    // Here's the *hard* way to achieve the above:
    // let c5 = Cell {
    //     column_id: cols.name_to_id["Column5"],
    //     object_value: Some(
    //         json!({"objectType": "MULTI_PICKLIST", "values": ["One", "Two", "Hello, world!"]}),
    //     ),
    //     ..Default::default()
    // };

    // Column 6 - Type: CONTACT LIST (MULTI SELECT)
    let c6 = make.multi_contact_cell(
        "Column6",
        &[
            Contact::from("user1.email@smartsheet.com").name("Contact Name"),
            "user2.email@smartsheet.com".into(),
        ],
    )?;

    // Again, here's the *hard* way to achieve the above:
    // let c6 = Cell {
    //     column_id: cols.name_to_id["Column6"],
    //     object_value: Some(json!(
    //     {"objectType": "MULTI_CONTACT",
    //     "values": [
    //         { "objectType": "CONTACT", "email": "user1.email@smartsheet.com" },
    //         { "objectType": "CONTACT", "email": "user2.mail@smartsheet.com" }
    //     ]})),
    //     ..Default::default()
    // };

    // Column 7 - Type: HYPERLINK, /w URL
    let c7 = make.url_hyperlink_cell("Column7", "My Link", "https://google.com")?;

    let cells = [c1, c2, c3, c4, c5, c6, c7];

    let row1 = Row::from(&cells);
    let row2 = row1.clone();

    println!("Created cells in {:.2?}", start.elapsed());

    println!("INPUT Object: {}\n", to_string_pretty(&row1).unwrap());

    let start = Instant::now();

    let rows = smart
        // alternatively:
        //   .add_rows(sheet_id, [row1, row2].to_bottom(true))
        .add_rows_with_params(sheet_id, [row1, row2].to_bottom(true), false, true)
        .await?;

    println!("Added Rows in {:.2?}", start.elapsed());
    println!();

    // Print out the IDs of each Row that were added.
    for row in rows.result {
        println!("  - Row ID: {}", row.id);
    }

    Ok(())
}
