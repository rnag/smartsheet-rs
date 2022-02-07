#![deny(warnings)]
#![warn(rust_2018_idioms)]

use log::error;
use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::{Column, Level};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: columns_with_level <sheet_id>";
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

    let start = Instant::now();

    let target_level = Level::MultiPicklist;

    let columns = smart
        .list_columns_with_level(sheet_id, Some(target_level))
        .await?;

    println!("List Columns with Level completed in {:?}", start.elapsed());
    println!();

    println!("Column Count: {}", columns.total_count);
    println!("Total Pages:  {}", columns.total_pages);
    println!("Page Number:  {}", columns.page_number);
    println!("Page Size:    {}", columns.page_size);

    if let Some(column) = columns.data.first() {
        println!();
        println!("Column Names");
        println!("---");

        for c in &columns.data {
            println!("  Id: {} | Name: {}", c.id, c.title);
        }

        println!();
        println!("First Column:");
        println!("---");
        println!("{:#?}", column);
    }

    // Uncomment to also display the name + id of each column
    // print_column_id_and_names(&columns.data).await?;

    Ok(())
}

/// Print column id and names, given a list of columns
#[allow(dead_code)]
async fn print_column_id_and_names(columns: &Vec<Column>) -> Result<()> {
    println!();
    println!("Column ID and Names");
    println!("---");

    for c in columns {
        println!("  Id: {} | Name: {}", c.id, c.title);
    }

    Ok(())
}
