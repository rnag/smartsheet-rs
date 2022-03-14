#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::Column;

use log::error;
use tabled::{Alignment, Footer, Header, Modify, Row, Style, TableIteratorExt, Tabled};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Column data, representing a row in the nicely formatted table that gets
/// printed to the console. For more info, check out the link below.
///
///   https://docs.rs/tabled
#[derive(Tabled)]
struct TableRow<'a> {
    #[header("Column ID")]
    id: u64,
    #[header("Column Name")]
    name: &'a str,
}

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: columns <sheet_id>";
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

    let result = smart.list_columns(sheet_id).await?;

    println!("List Columns completed in {:.2?}", start.elapsed());
    println!();

    println!("Column Count: {}", result.total_count);
    println!("Total Pages:  {}", result.total_pages);
    println!("Page Number:  {}", result.page_number);
    println!("Page Size:    {}", result.page_size);
    println!();

    // Print out the sheet data (IDs and names in this case) as a nicely
    // formatted table.

    let mut rows = Vec::with_capacity(result.total_count as usize);

    for column in &result.data {
        rows.push(TableRow {
            id: column.id,
            name: &column.title,
        });
    }

    //noinspection DuplicatedCode
    println!(
        "{}",
        rows.table()
            .with(Style::PSEUDO)
            .with(Modify::new(Row(1..)).with(Alignment::left()))
            .with(Header("Available Columns"))
            .with(Footer(format!("{} Total Columns", result.total_count)))
    );

    // Uncomment to print display on the first column
    // print_info_on_first_column(&result.data).await?;

    Ok(())
}

/// Print details on the first column in the response
// noinspection DuplicatedCode
#[allow(dead_code)]
async fn print_info_on_first_column(columns: &Vec<Column>) -> Result<()> {
    if let Some(column) = columns.first() {
        println!();
        println!("First Column:");
        println!("---");
        println!("{:#?}", column);
    }

    Ok(())
}
