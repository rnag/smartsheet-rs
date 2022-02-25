# smartsheet-rs

[<img alt="github" src="https://img.shields.io/badge/github-rnag/smartsheet--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/rnag/smartsheet-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/smartsheet-rs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/smartsheet-rs)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/smartsheet-rs/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/smartsheet-rs)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/rnag/smartsheet-rs/build/main?style=for-the-badge" height="22">](https://github.com/rnag/smartsheet-rs/actions?query=branch%3Amain)

**smartsheet-rs** is a *rust crate* which provides an `async` wrapper API that lets you easily interact
with the [Smartsheet API 2.0](https://smartsheet-platform.github.io/api-docs/).

This is an *unofficial* SDK I have made to learn Rust a little, but I hope you have fun with it --
I know that I certainly had quite a bit of fun in writing out the implementation for
this crate.

## Table of Contents

* [Getting Started](#getting-started)
* [Implemented Methods](#implemented-methods)
* [A Larger Example](#a-larger-example)
* [Dependencies and Features](#dependencies-and-features)
* [Contributing](#contributing)
* [License](#license)
* [Authors](#authors)

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
   smartsheet-rs = "0.3"
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

       println!("Printing sheet names:");
       for sheet in sheets.data {
           println!("  - {}", sheet.name);
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
- [Get Column](https://smartsheet-platform.github.io/api-docs/#get-column)

You can check out sample usage of these API methods in the [examples/](https://github.com/rnag/smartsheet-rs/tree/main/examples)
folder in the project repo on GitHub.

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
    let get_cell = CellGetter::new(&cols);

    // Get the first row in the sheet. We could also access
    // a row by index, like `&sheet.rows[i]` for example.
    let first_row = sheet.rows.first().unwrap();
   
    // Try to find a cell in the row by it's column name
    match get_cell.by_name(first_row, COLUMN_NAME) {
        Ok(cell) => println!("Here's the cell: {:#?}", *cell),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
```

The `CellGetter::by_name` method works by iterating over each cell in the row,
and then returning the first `Cell` where the *column ID* for the cell
matches the specified *column name*.

If the need arises to retrieve *multiple* `Cell` objects from a `Row` by their column names,
it might be a better idea to first build out a mapping of each *column name* to the
`Cell` object in the row for that column. The method `CellGetter::name_to_cell` can be used
for this purpose, as shown below.

```rust
let column_name_to_cell = get_cell.name_to_cell(row);

println!("{:#?}", column_name_to_cell);
// Prints:
// {
//     "Column 1": Cell {...},
//     "Column 2": Cell {...},
//      ...
```

## Dependencies and Features

This library uses only the minimum required dependencies, in order
to keep the overall size small. This crate uses [`hyper`][] and [`hyper-rustls`][]
internally, to make HTTPS requests to the Smartsheet API.

While `hyper-rustls` was chosen as the default TLS implementation
because it works without issue when cross-compiling for the
**x86_64-unknown-linux-musl** target as is common for [AWS Lambda][]
deployments, it is still possible to instead use the native [`hyper-tls`][]
implementation, which relies on OpenSSL.

To do this, disable the default "rust-tls" feature and enable the "native-tls" feature:

```toml
[dependencies]
smartsheet-rs = { version = "0.3", default-features = false, features = ["native-tls", "logging", "serde-std"] }
```

[`hyper`]: https://docs.rs/hyper
[`hyper-rustls`]: https://docs.rs/hyper-rustls
[`hyper-tls`]: https://docs.rs/hyper-tls
[AWS Lambda]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[Contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/rnag/smartsheet-rs/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`smartsheet-rs` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

* [Ritvik Nag](https://github.com/rnag)
