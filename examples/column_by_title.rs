#![deny(warnings)]
#![allow(deprecated)]
#![warn(rust_2018_idioms)]

use log::error;
use std::env;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::time::Instant;

use smartsheet_rs;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// noinspection DuplicatedCode
async fn fetch_single_arg<T: FromStr>(arg_pos: usize) -> Result<T>
where
    <T as FromStr>::Err: 'static + std::error::Error + Send + Sync,
{
    // Some simple CLI args requirements...
    match env::args().nth(arg_pos) {
        Some(value) => Ok(value.parse::<T>()?),
        None => {
            let error_msg = "Usage: column_by_title <sheet_id> <column_title>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

async fn fetch_args() -> Result<(u64, String)> {
    // Some simple CLI args requirements...
    let sheet_id: u64 = fetch_single_arg(1).await?;
    let column_title: String = fetch_single_arg(2).await?;

    Ok((sheet_id, column_title))
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let (sheet_id, column_title) = fetch_args().await?;

    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let start = Instant::now();

    let column = smart.get_column_by_title(sheet_id, &column_title).await?;

    println!("Get Column By Name completed in {:.2?}", start.elapsed());
    println!();

    // Print out some basic info about the column
    println!("Column Title: {}", column.title);
    println!("Column ID:    {}", column.id);
    println!("Column Index: {}", column.index);
    println!("Type:         {}", column.type_field);
    println!("Width:        {}", column.width);
    println!("Version:      {:?}", column.version);

    Ok(())
}
