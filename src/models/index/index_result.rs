use serde::{Deserialize, Serialize};

/// **Index Result** - Object returned for all GET operations against index
/// endpoints. This object provides metadata which can be used to perform
/// paging on potentially large data sets.
///
/// Docs:
/// - https://smartsheet-platform.github.io/api-docs/?python#indexresult-object
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexResult<T> {
    /// Will be one of:
    ///   - Sheet
    ///   - Column
    pub data: Vec<T>,
    pub page_number: u64,
    // Page size of the operation
    //
    // *Note*: this value can be omitted in the response, for example
    // when passing `include_all=true` in a **List Columns** request
    #[serde(default)]
    pub page_size: u64,
    pub total_count: u64,
    pub total_pages: u64,
}
