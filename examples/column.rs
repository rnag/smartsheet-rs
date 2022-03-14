#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;

use log::error;
use tabled::{Header, Style, TableIteratorExt, Tabled};

/// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Tabled)]
struct TableRow<'a> {
    #[header("Column ID")]
    id: u64,
    #[header("Index")]
    index: u64,
    #[header("Title")]
    title: &'a str,
    #[header("Field Type")]
    field_type: &'a str,
}

// noinspection DuplicatedCode
async fn fetch_single_arg(arg_pos: usize) -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(arg_pos) {
        Some(value) => Ok(value.parse::<u64>()?),
        None => {
            let error_msg = "Usage: column <sheet_id> <column_id>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

async fn fetch_args() -> Result<(u64, u64)> {
    // Some simple CLI args requirements...
    let sheet_id = fetch_single_arg(1).await?;
    let column_id = fetch_single_arg(2).await?;

    Ok((sheet_id, column_id))
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let (sheet_id, column_id) = fetch_args().await?;

    // Create Smartsheet client
    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let start = Instant::now();

    // Get info on the sheet column
    let col = smart.get_column(sheet_id, column_id).await?;

    println!("Get Column completed in {:.2?}", start.elapsed());
    println!();

    // Print out some basic info about the row

    let tr = TableRow {
        id: col.id,
        index: col.index,
        title: &col.title,
        field_type: &col.type_field,
    };

    println!(
        "{}",
        [tr].table()
            .with(Style::PSEUDO_CLEAN)
            // .with(Modify::new(Row(1..)).with(Alignment::left()))
            .with(Header("Column Info"))
    );

    Ok(())
}
