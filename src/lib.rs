// #![deny(warnings)]
// #![warn(rust_2018_idioms)]

//! [![github]](https://github.com/rnag/smartsheet-rs)&ensp;[![crates-io]](https://crates.io/crates/smartsheet-rs)&ensp;[![docs-rs]](https://docs.rs/smartsheet-rs)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! An async Rust library implementation to interact with the
//! [Smartsheet API v2](https://smartsheet-platform.github.io/api-docs/).
//!
//! <br>
//!
//! ## Example
//!
//! ```no_run
//! use smartsheet_rs::SmartsheetApi;
//!
//! #[tokio::main]
//! async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let smart = SmartsheetApi::from_env()?;
//!     println!("Created a Smartsheet API client");
//!
//!     let sheets = smart.list_sheets().await?;
//!
//!     println!("Printing sheet IDs and names:");
//!     for sheet in sheets.data {
//!         println!(
//!             "{sep}{id:<20}|{sep}{name}",
//!             sep = '\t',
//!             id = sheet.id,
//!             name = sheet.name
//!         );
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
//! - [List Attachments](https://smartsheet-platform.github.io/api-docs/#list-attachments)
//! - [Get Sheet](https://smartsheet-platform.github.io/api-docs/#get-sheet)
//! - [Get Column](https://smartsheet-platform.github.io/api-docs/#get-column)
//! - [Get Attachment](https://smartsheet-platform.github.io/api-docs/#get-attachment)
//! - [Get Row](https://smartsheet-platform.github.io/api-docs/#get-row)
//! - [Add Rows](https://smartsheet-platform.github.io/api-docs/#add-rows)
//! - [Update Rows](https://smartsheet-platform.github.io/api-docs/#update-rows)
//! - [Delete Rows](https://smartsheet-platform.github.io/api-docs/#delete-rows)
//!
//! You can check out sample usage of these API methods in the [examples/](https://github.com/rnag/smartsheet-rs/tree/main/examples)
//! folder in the project repo on GitHub.
//!
//! ## A Larger Example
//!
//! This section contains more examples of usage. You can find it in the readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/smartsheet-rs#a-larger-example
//! [`README.md`]: https://github.com/rnag/smartsheet-rs#a-larger-example
//!
//! ## Dependencies and Features
//!
//! This library uses only the minimum required dependencies, in order
//! to keep the overall size small. This crate uses [hyper][] and
//! [hyper-rustls][] internally, to make HTTPS requests to the Smartsheet API.
//!
//! While `hyper-rustls` was chosen as the default TLS implementation
//! because it works without issue when cross-compiling for the
//! **x86_64-unknown-linux-musl** target as is common for [AWS Lambda][]
//! deployments, it is still possible to instead use the native [`hyper-tls`][]
//! implementation, which relies on OpenSSL.
//!
//! To do this, disable the default "rust-tls" feature and enable the "native-tls" feature:
//!
//! ```toml
//! [dependencies]
//! smartsheet-rs = { version = "0.6.1", default-features = false, features = ["native-tls", "logging", "serde-std"] }
//! ```
//!
//! [hyper]: https://docs.rs/hyper
//! [hyper-rustls]: https://docs.rs/hyper-rustls
//! [`hyper-tls`]: https://docs.rs/hyper-tls
//! [AWS Lambda]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html
//!

// #![warn(missing_docs)]

mod features_check;

#[cfg(feature = "logging")]
mod log {
    pub use log::{debug, error, trace, warn};
}

#[cfg(not(feature = "logging"))]
mod log {
    macro_rules! debug      ( ($($tt:tt)*) => {{}} );
    macro_rules! error      ( ($($tt:tt)*) => {{}} );
    macro_rules! trace      ( ($($tt:tt)*) => {{}} );
    macro_rules! warning    ( ($($tt:tt)*) => {{}} );
    pub(crate) use {debug, error, trace, warning as warn};
}

pub use api::SmartsheetApi;
pub use cell_factory::CellFactory;
pub use helpers::{CellGetter, ColumnMapper, RowGetter};

mod api;
pub mod auth;
pub mod builders;
mod cell_factory;
pub mod constants;
pub mod helpers;
mod https;
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
