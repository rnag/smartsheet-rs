use core::option::Option;
use serde::{Deserialize, Serialize};

/// Represents a hyperlink to a dashboard, report, sheet, or URL.
///
/// https://smartsheet-platform.github.io/api-docs/#hyperlink-object
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hyperlink {
    /// When the hyperlink is a URL link, this property contains the URL value.
    ///
    /// When the hyperlink is a dashboard/report/sheet link (that is,
    /// dashboardId, reportId, or sheetId is non-null), this property contains
    /// the permalink to the dashboard, report, or sheet.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub url: String,
    /// If non-null, this hyperlink is a link to the report with this Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<u64>,
    /// If non-null, this hyperlink is a link to the sheet with this Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<u64>,
    /// If non-null, this hyperlink is a link to the dashboard with this Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sight_id: Option<u64>,
}

impl Hyperlink {
    /// Create a new `Hyperlink` object with a **url**
    pub fn with_url<S: Into<String>>(url: S) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }
}

impl From<&str> for Hyperlink {
    fn from(value: &str) -> Self {
        Self::with_url(value)
    }
}

impl From<String> for Hyperlink {
    fn from(value: String) -> Self {
        Self::with_url(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_link() {
        let link = Hyperlink {
            sheet_id: Some(12345),
            ..Default::default()
        };

        assert_eq!(
            serde_json::to_string_pretty(&link).unwrap(),
            indoc! {r#"
                {
                  "sheetId": 12345
                }
            "#}
            .trim()
        )
    }
}
