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
    report_id: Option<u64>,
    /// If non-null, this hyperlink is a link to the sheet with this Id.
    #[serde(rename = "sheetId")]
    sheet_id: Option<u64>,
    /// If non-null, this hyperlink is a link to the dashboard with this Id.
    #[serde(rename = "sightId")]
    sight_id: Option<u64>,
}
