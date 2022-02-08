#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::time::Instant;

use smartsheet_rs;
use smartsheet_rs::models::{ListSheetIncludeFlags, Sheet};

use tabled::{Alignment, Footer, Header, Modify, Row, Style, TableIteratorExt, Tabled};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Sheet data, representing a row in the nicely formatted table that gets
/// printed to the console. For more info, check out the link below.
///
///   https://docs.rs/tabled
#[derive(Tabled)]
struct TableRow<'a> {
    #[header("Sheet ID")]
    id: u64,
    #[header("Sheet Name")]
    name: &'a str,
    #[header("Owner")]
    owner: &'a str,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let smart = smartsheet_rs::SmartsheetApi::from_env()?;

    let start = Instant::now();

    let include = Some(vec![
        ListSheetIncludeFlags::OwnerInfo,
        ListSheetIncludeFlags::SheetVersion,
        ListSheetIncludeFlags::Source,
    ]);

    let include_all = Some(true);

    let result = smart
        .list_sheets_with_params(include, include_all, None)
        .await?;

    println!(
        "List Sheets With Params completed in {:.2?}",
        start.elapsed()
    );
    println!();

    println!("Sheet Count:  {}", result.total_count);
    println!("Total Pages:  {}", result.total_pages);
    println!("Page Number:  {}", result.page_number);
    println!("Page Size:    {}", result.page_size);
    println!();

    // Print out the sheet data (IDs and names in this case) as a nicely
    // formatted table.

    let mut rows = Vec::with_capacity(result.total_count as usize);

    for sheet in &result.data {
        rows.push(TableRow {
            id: sheet.id,
            name: &sheet.name,
            owner: &sheet.owner.as_ref().unwrap(),
        });
    }

    //noinspection DuplicatedCode
    println!(
        "{}",
        rows.table()
            .with(Style::PSEUDO)
            .with(Modify::new(Row(1..)).with(Alignment::left()))
            .with(Header("Available Sheets"))
            .with(Footer(format!("{} Total Sheets", result.total_count)))
    );

    // Uncomment to print display on the first sheet
    // print_info_on_first_sheet(&result.data).await?;

    Ok(())
}

/// Print details on the first sheet in the response
// noinspection DuplicatedCode
#[allow(dead_code)]
async fn print_info_on_first_sheet(sheets: &Vec<Sheet>) -> Result<()> {
    if let Some(sheet) = sheets.first() {
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

    Ok(())
}
