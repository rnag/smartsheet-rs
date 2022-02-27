//! This is an example which demonstrates how to work with cells that are for
//! `MULTI_CONTACT` columns, which are a bit tricky to work with.
//!
use smartsheet_rs::{CellGetter, ColumnMapper, SmartsheetApi};

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use log::error;
use smartsheet_rs::models::ContactEmailAddrs;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO Update these values as needed
const MULTI_CONTACT_COLUMN_NAME: &str = "My Email Column";
/// Row Numbers (Indexes on the left as they appear in the smartsheet)
///
/// # Note
/// These are *not* the same as the Row ID's.
///
const MULTI_CONTACT_ROW_NUMBERS: [usize; 3] = [9, 15, 43];

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: cell_multi_contact <sheet_id>";
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

    let start = Instant::now();

    // To get the `email` addresses for each *contact*, the docs mention we will
    // need to set the Row Include flags to include `objectValue`, and also set
    // the `level` parameter accordingly.
    //
    // We can use this convenience function to retrieve the rows with the full
    // `MULTI_CONTACT` details populated, since as email addresses for example.
    let sheet = smart.get_sheet_with_multi_contact_info(sheet_id).await?;

    println!(
        "Get Sheet with Multi Contact Info completed in {:.2?}",
        start.elapsed()
    );
    println!();

    let cols = ColumnMapper::from(&sheet);
    let get_cell = CellGetter::new(&cols);

    // Now, for each row we retrieve the cell for the specified `MULTI_CONTACT` column,
    // and print it out.

    println!("-- Printing the Cell for each Row --");

    for row_num in MULTI_CONTACT_ROW_NUMBERS {
        let row = &sheet.rows[row_num];
        let cell = get_cell.by_name(row, MULTI_CONTACT_COLUMN_NAME)?;

        println!("ROW NUMBER: {}", row_num);
        println!("  {:#?}", cell);
        println!();
    }

    println!("-- Printing Contact Info for each Row --");

    for row_num in MULTI_CONTACT_ROW_NUMBERS {
        // Retrieve the cell for the `MULTI_CONTACT` column from each row number.
        let cell = get_cell.by_name(&sheet.rows[row_num], MULTI_CONTACT_COLUMN_NAME)?;

        // Create a list of `Contact` objects from the cell details.
        let contacts = cell.contacts()?;

        // Get the contact emails, as a comma-delimited string in the format
        // *john1@example.com, john2@example.com*
        let emails = contacts.addrs_str();

        // Get a list of contact name addresses, where each one as indicated
        // in the RFC will be in the format `[display-name] angle-addr` --
        // that is, for example, *John Doe <john@example.com>*
        let names = contacts.name_addrs();

        println!("ROW NUMBER: {}", row_num);
        println!("  Emails: {:?}", emails);
        println!("  Names Addresses: {:#?}", names);

        println!();
    }

    Ok(())
}
