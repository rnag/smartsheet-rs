# smartsheet-rs

[![Crates.io](https://img.shields.io/crates/v/smartsheet-rs.svg)](https://crates.io/crates/smartsheet-rs)
[![Docs](https://docs.rs/smartsheet-rs/badge.svg)](https://docs.rs/smartsheet-rs)
[![MIT](https://img.shields.io/crates/l/smartsheet-rs.svg)](https://crates.io/crates/smartsheet-rs)

**smartsheet-rs** is a *rust crate* which provides an `async` wrapper API that lets you easily interact
with the [Smartsheet API 2.0](https://smartsheet-platform.github.io/api-docs/).

This is an *unofficial* SDK I have made to learn Rust a little, but I hope you have fun with it --
I know that I certainly had quite a bit of fun in writing out the implementation for
this crate.

## Getting Started

Getting started with the `smartsheet-rs` library is easy:

1. Set **SMARTSHEET_ACCESS_TOKEN** in your environment; you can
   also use the `SmartsheetApi::from_token` constructor
   to explicitly set the token value.
   Find out more  about [Authentication and Access Tokens](https://smartsheet-platform.github.io/api-docs/#authentication-and-access-tokens)
   in the Smartsheet API Documentation.

3. Add these dependencies to your `Cargo.toml`:

   ```toml
   [dependencies]
   smartsheet-rs = "0.1"
   tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
   ```

3. Add some usage to your application:

   ```rust
   use smartsheet_rs::SmartsheetApi;

   #[tokio::main]
   async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
       let smart = SmartsheetApi::from_env()?;
       println!("Created a Smartsheet API client");

       let sheets = smart.list_sheets().await?;

       println!("Printing sheet IDs and names:");
       for sheet in sheets.data {
           println!("\t{id:<20}|\t{name}", id = sheet.id, name = sheet.name);
       }

       Ok(())
   }
   ```

## A Larger Example

When working with rows and cells in the SmartSheet API, one thing that
stands out is that the API purposefully identifies columns by their *ID*,
rather than their title or *column name*.

However, as humans it's much more natural and convenient to refer to *column names*
when working with the data.
Towards that end, the **smartsheet-rs** crate provides helper *struct* implementations
such as the `ColumnMapper` and `CellGetter` in order to simplify interaction
with the Smartsheet API.

Here's a quick example of how that would work:

```rust
use smartsheet_rs::{CellGetter, ColumnMapper, SmartsheetApi};

// TODO update these values as needed
const SHEET_ID: u64 = 1234567890;
const COLUMN_NAME: &str = "My Column";

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let smart = SmartsheetApi::from_env()?;

    let sheet = smart.get_sheet(SHEET_ID).await?;

    // Create interchangeable name <-> id mappings for columns in the row
    let cols = ColumnMapper::new(&sheet.columns);

    // Create a `CellGetter` helper to find cells in a row by *Column Name*
    let get_cell = CellGetter::from_mapper(&cols);

    // Get the first row in the sheet. We could also access
    // a row by index, like `&sheet.rows[i]` for example.
    let first_row = sheet.rows.first().unwrap();
   
   // Try to find a cell in the row by it's column name
    if let Some(cell) = get_cell.by_name(first_row, COLUMN_NAME) {
        println!("Here's the cell: {:#?}", *cell);
    } else {
        println!("No such cell for the specified column!")
    }

    Ok(())
}
```

## Implemented Methods

The following API methods from the [official documentation](https://smartsheet-platform.github.io/api-docs)
have been implemented currently:

- [List Sheets](https://smartsheet-platform.github.io/api-docs/#list-sheets)
- [List Columns](https://smartsheet-platform.github.io/api-docs/#list-columns)
- [Get Sheet](https://smartsheet-platform.github.io/api-docs/#get-sheet)
- [Get Row](https://smartsheet-platform.github.io/api-docs/#get-row)

You can check out sample usage of these API methods in the [examples/](https://github.com/rnag/smartsheet-rs/tree/main/examples)
folder in the project repo on GitHub.

## Dependencies

This library uses only the minimum required dependencies, in order
to keep the overall size small. This crate uses [`hyper`][] and [`hyper-tls`][]
internally, to make HTTPS requests to the Smartsheet API.

[`hyper`]: https://docs.rs/hyper
[`hyper-tls`]: https://docs.rs/hyper-tls
