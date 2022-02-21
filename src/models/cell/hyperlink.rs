use core::option::Option;
use serde::{Deserialize, Serialize};

/// Represents a hyperlink to a dashboard, report, sheet, or URL.
///
/// https://smartsheet-platform.github.io/api-docs/#hyperlink-object
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hyperlink {
    /// When the hyperlink is a URL link, this property contains the URL value.
    pub url: String,
    /// If non-null, this hyperlink is a link to the report with this Id.
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<u64>,
    /// If non-null, this hyperlink is a link to the sheet with this Id.
    #[serde(rename = "sheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<u64>,
    /// If non-null, this hyperlink is a link to the dashboard with this Id.
    #[serde(rename = "sightId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sight_id: Option<u64>,
}
