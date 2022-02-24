use crate::models::Row;

use serde::{Deserialize, Serialize};

/// Response returned when *Adding* `Row`s
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRowResult {
    pub message: String,
    pub result: Row,
    pub result_code: i64,
    pub version: i64,
}
