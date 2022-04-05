#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::collections::HashMap;
use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

use serde_json::to_string_pretty;

use smartsheet_rs;
use smartsheet_rs::models::RowIncludeFlags::Attachments;
use smartsheet_rs::models::{AttachmentMeta, SheetIncludeFlags};

#[macro_use]
extern crate log;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_args() -> Result<u64> {
    // Some simple CLI args requirements...
    match env::args().nth(1) {
        Some(sheet_id) => Ok(sheet_id.parse::<u64>()?),
        None => {
            let error_msg = "Usage: sheet_with_attachments <sheet_id>";
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

    let sheet = smart
        .get_sheet_with_params(
            sheet_id,
            vec![SheetIncludeFlags::Base(Attachments)],
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;

    trace!(
        "Get Sheet with Attachments completed in {:.2?}",
        start.elapsed()
    );

    let row_to_attachments: HashMap<_, _> = sheet
        .rows
        .into_iter()
        .filter_map(|r| match r.attachments {
            Some(attachments) if !attachments.is_empty() => Some((r.id, attachments)),
            _ => None,
        })
        .collect();

    trace!("Found {} rows with attachments", row_to_attachments.len());
    println!();

    // Uncomment to show *all* row attachments in a sheet
    // print_row_attachments(sheet_id, &row_to_attachments)?;

    // Uncomment to show only the *first* row attachment in a sheet
    print_first_row_attachment(sheet_id, &row_to_attachments)?;

    Ok(())
}

/// Show **all** attachments from a list of *row attachments* in a sheet
#[allow(unused)]
fn print_row_attachments(
    sheet_id: u64,
    row_to_attach: &HashMap<u64, Vec<AttachmentMeta>>,
) -> Result<()> {
    trace!(
        "Row ID to Attachments: {}",
        to_string_pretty(&row_to_attach)?
    );
    println!();

    trace!("Sheet ID: {}", sheet_id);

    let first_row_attachments = match row_to_attach.iter().nth(0) {
        Some((_row_id, attachments)) => attachments,
        None => return Ok(()),
    };
    let default_attach = Default::default();
    let first_attach = first_row_attachments.first().unwrap_or(&default_attach);

    trace!("First Attachment ID: {}", first_attach.id);

    Ok(())
}

/// Show the **first** attachment from a list of *row attachments* in a sheet
#[allow(unused)]
fn print_first_row_attachment(
    sheet_id: u64,
    row_to_attach: &HashMap<u64, Vec<AttachmentMeta>>,
) -> Result<()> {
    let first_row_attachments = match row_to_attach.iter().nth(0) {
        Some((_row_id, attachments)) => attachments,
        None => return Ok(()),
    };

    trace!(
        "First Row Attachments: {}",
        to_string_pretty(&first_row_attachments)?
    );
    println!();

    trace!("Sheet ID: {}", sheet_id);
    trace!(
        "First Attachment ID: {}",
        first_row_attachments
            .first()
            .unwrap_or(&AttachmentMeta::default())
            .id
    );

    Ok(())
}
