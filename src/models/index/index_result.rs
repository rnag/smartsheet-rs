use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// **Index Result** - Object returned for all GET operations against index
/// endpoints. This object provides metadata which can be used to perform
/// paging on potentially large data sets.
///
/// Docs:
/// - https://smartsheet-platform.github.io/api-docs/?python#indexresult-object
pub struct IndexResult<T> {
    /// Will be one of:
    ///   - Sheet
    ///   - Column
    pub data: Vec<T>,
    #[serde(rename = "pageNumber")]
    pub page_number: u64,
    // Page size of the operation
    //
    // *Note*: this value can be omitted in the response, for example
    // when passing `include_all=true` in a **List Columns** request
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: u64,
    #[serde(rename = "totalCount")]
    pub total_count: u64,
    #[serde(rename = "totalPages")]
    pub total_pages: u64,
}
