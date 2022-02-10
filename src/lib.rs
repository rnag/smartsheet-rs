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
//! - [Get Sheet](https://smartsheet-platform.github.io/api-docs/#get-sheet)
//! - [Get Row](https://smartsheet-platform.github.io/api-docs/#get-row)
//! - [Get Column](https://smartsheet-platform.github.io/api-docs/#get-column)
//!
//! You can check out sample usage of these API methods in the [examples/](https://github.com/rnag/smartsheet-rs/tree/main/examples)
//! folder in the project repo on GitHub.
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
//! smartsheet-rs = { version = "0.2", default-features = false, features = ["native-tls", "logging", "serde-std"] }
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
pub use helpers::{CellGetter, ColumnMapper};

mod api;
pub mod auth;
pub mod builders;
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
