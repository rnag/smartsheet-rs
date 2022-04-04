#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use serde_json::to_string_pretty;

use smartsheet_rs;
use smartsheet_rs::models::{AttachmentMeta, IndexResult};

#[macro_use]
extern crate log;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: attachments <sheet_id>";
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

    let attachments = smart.list_attachments(sheet_id).await?;

    trace!("List Attachments completed in {:.2?}", start.elapsed());
    trace!("Found {} attachments", attachments.total_count);
    println!();

    // Uncomment to show *all* attachments in a sheet
    // print_attachments(sheet_id, &attachments)?;

    // Uncomment to show only the *first* attachment in a sheet
    print_first_attachment(sheet_id, &attachments)?;

    Ok(())
}

/// Show **all** attachments from a list of *attachments* in a sheet
#[allow(unused)]
fn print_attachments(sheet_id: u64, attachments: &IndexResult<AttachmentMeta>) -> Result<()> {
    let default_attach = Default::default();
    let first_attach = attachments.data.first().unwrap_or(&default_attach);

    trace!("Attachments: {}", to_string_pretty(&attachments.data)?);
    println!();

    trace!("Sheet ID: {}", sheet_id);
    trace!("First Attachment ID: {}", first_attach.id);

    Ok(())
}

/// Show the **first** attachment from a list of *attachments* in a sheet
#[allow(unused)]
fn print_first_attachment(sheet_id: u64, attachments: &IndexResult<AttachmentMeta>) -> Result<()> {
    let default_attach = Default::default();
    let first_attach = attachments.data.first().unwrap_or(&default_attach);

    trace!("First Attachment: {}", to_string_pretty(&first_attach)?);
    println!();

    trace!("Sheet ID: {}", sheet_id);
    trace!("First Attachment ID: {}", first_attach.id);

    Ok(())
}
