// #![deny(warnings)]
// #![warn(rust_2018_idioms)]

//! An async Rust library implementation to interact with the
//! [Smartsheet API v2](https://smartsheet-platform.github.io/api-docs/).
//!
//! ## Example
//!
//! ```no_run
//! use smartsheet_rs::SmartsheetApi;
//!
//! #[tokio::main]
//! async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     // Note: replace the value with your token, or alternatively
//!     // ensure that the `SMARTSHEET_ACCESS_TOKEN` env variable is
//!     // set, and uncomment the following.
//!     // let smart = SmartsheetApi::from_env()?;
//!     let smart = SmartsheetApi::from_token("MY-TOKEN");
//!     println!("Created a Smartsheet API client");
//!
//!     let sheets = smart.list_sheets().await?;
//!
//!     println!("Printing sheet IDs and names:");
//!     for sheet in sheets.data {
//!         println!("\t{id:<20}|\t{name}", id = sheet.id, name = sheet.name);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Implemented Methods
//!
//! The following API methods from the [official documentation](https://smartsheet-platform.github.io/api-docs)
//! have been implemented currently:
//!
//! - [List Sheets](https://smartsheet-platform.github.io/api-docs/#list-sheets)
//! - [List Columns](https://smartsheet-platform.github.io/api-docs/#list-columns)
//! - [Get Sheet](https://smartsheet-platform.github.io/api-docs/#get-sheet)
//! - [Get Row](https://smartsheet-platform.github.io/api-docs/#get-row)
//!
//! You can check out sample usage of these API methods in the [examples/](https://github.com/rnag/smartsheet-rs/tree/main/examples)
//! folder in the project repo on GitHub.
//!
//! ## Dependencies
//!
//! This library uses only the minimum required dependencies, in order
//! to keep the overall size small. This crate uses [hyper][] and
//! [hyper-tls][] internally, to make HTTPS requests to the Smartsheet API.
//!
//! [hyper]: https://docs.rs/hyper
//! [hyper-tls]: https://docs.rs/hyper-tls

pub use api::SmartsheetApi;
pub use helpers::{CellGetter, ColumnMapper};

mod api;
pub mod auth;
pub mod builders;
pub mod constants;
pub mod helpers;
pub mod models;
pub mod status;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
