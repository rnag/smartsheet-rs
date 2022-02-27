use crate::models::Row;

use serde::{Deserialize, Serialize};

/// Response returned when successfully *Adding* `Row`s to a Sheet. Represents
/// a [Result] object.
///
/// [Result]: https://smartsheet.redoc.ly/#section/Result-Object
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowResult<R = Row> {
    pub message: Message,
    pub result: Vec<R>,
    pub result_code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<R>>,
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
