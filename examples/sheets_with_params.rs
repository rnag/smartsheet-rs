#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::{ListSheetIncludeFlags, Sheet};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let start = Instant::now();

    let include = Some(vec![
        ListSheetIncludeFlags::SheetVersion,
        ListSheetIncludeFlags::Source,
    ]);

    let sheets = smart.list_sheets_with_params(include).await?;

    println!("List Sheets With Params completed in {:?}", start.elapsed());
    println!();

    println!("Sheet Count:  {}", sheets.total_count);
    println!("Total Pages:  {}", sheets.total_pages);
    println!("Page Number:  {}", sheets.page_number);
    println!("Page Size:    {}", sheets.page_size);

    if let Some(sheet) = sheets.data.first() {
        println!();
        println!("First Sheet:");
        println!("---");
        println!("{:#?}", sheet);

        // Assert that expected values are populated
        assert!(
            sheet.version.is_some(),
            "Expected `version` to be populated"
        );
        assert!(sheet.source.is_some(), "Expected `source` to be populated");
    }

    // Uncomment to also display the name of each sheet
    // print_sheet_names(&sheets.data).await?;

    Ok(())
}

/// Print sheet names, given a list of sheets
#[allow(dead_code)]
async fn print_sheet_names(sheets: &Vec<Sheet>) -> Result<()> {
    println!();
    println!("Sheet Names:");
    println!("---");

    for sheet in sheets {
        println!("  {}", sheet.name)
    }

    Ok(())
}
