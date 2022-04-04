use serde::{Deserialize, Serialize};

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
