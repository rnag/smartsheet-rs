use serde::{Deserialize, Serialize};

/// Represents a valid [Attachment Sub Type] in Smartsheet.
///
/// [Attachment Sub Type]: https://smartsheet-platform.github.io/api-docs/#objects
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttachmentSubType {
    /// EGNYTE values: FOLDER
    Folder,
    /// GOOGLE_DRIVE values
    Document,
    Drawing,
    Pdf,
    Presentation,
    Spreadsheet,
}

impl Default for AttachmentSubType {
    fn default() -> Self {
        Self::Document
    }
}
