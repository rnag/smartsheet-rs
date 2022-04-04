#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use smartsheet_rs;

#[macro_use]
extern crate log;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// noinspection DuplicatedCode
async fn fetch_single_arg(arg_pos: usize) -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(arg_pos) {
        Some(value) => Ok(value.parse::<u64>()?),
        None => {
            let error_msg = "Usage: attachment <sheet_id> <attachment_id>";
            error!("{}", error_msg);
            Err(Box::new(Error::new(ErrorKind::InvalidInput, error_msg)))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let sheet_id = fetch_single_arg(1).await?;
    let attachment_id = fetch_single_arg(2).await?;

    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let start = Instant::now();

    let attachment = smart.get_attachment(sheet_id, attachment_id).await?;

    trace!("Get Attachment completed in {:.2?}", start.elapsed());
    println!();

    trace!("Attachment: {}", serde_json::to_string_pretty(&attachment)?);
    trace!("Attachment Download URL: {}", attachment.download_url());

    Ok(())
}
