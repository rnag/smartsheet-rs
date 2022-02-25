use crate::models::Row;

use serde::{Deserialize, Serialize};

/// Response returned when successfully *Adding* `Row`s to a Sheet. Represents
/// a [Result] object.
///
/// [Result]: https://smartsheet.redoc.ly/#section/Result-Object
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowResult {
    pub message: Message,
    pub result: Vec<Row>,
    pub result_code: i64,
    pub version: i64,
    pub failed_items: Option<Vec<Row>>,
}

/// Represents a `Message` enum, which can be one of:
///   * `SUCCESS`
///   * `PARTIAL_SUCCESS`
///
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Message {
    Success,
    PartialSuccess,
}
