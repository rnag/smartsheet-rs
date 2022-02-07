/// Library-specific errors, including types and implementations.
///
use serde::{Deserialize, Serialize};

/// `RequestError` is raised when the Smartsheet API responds back with a
/// *non-* "OK" response.
///
/// More specifically, this error is raised when the status code of
/// a response is between 400 and 600, which indicates its either a client
/// error or a server error.
///
/// # Note
///
/// The `error` and `message` fields are mutually-exclusive; if we cannot
/// de-serialize `error`, the `message` will be populated instead with the
/// response data.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestError {
    pub status: u16,
    pub reason: String,
    pub message: Option<String>,
    pub error: Option<SmartsheetError>,
}

impl RequestError {
    /// Create a new `RequestError` object from a status code and reason.
    ///
    /// The `error` and `message` fields are mutually-exclusive, and so will
    /// both initially be unset.
    pub fn new(status: u16, reason: String) -> Self {
        Self {
            status,
            reason,
            message: None,
            error: None,
        }
    }
}
/// An error returned from the Smartsheet API, along with a custom error
/// code from the Smartsheet side.
///
/// # Docs
/// - https://smartsheet-platform.github.io/api-docs/#error-object
/// - https://smartsheet-platform.github.io/api-docs/#complete-error-code-list
///
#[derive(Debug, Deserialize, Serialize)]
pub struct SmartsheetError {
    pub message: String,
    #[serde(rename = "errorCode")]
    pub error_code: u16,
    #[serde(rename = "refId")]
    pub ref_id: Option<String>,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RequestError {}
