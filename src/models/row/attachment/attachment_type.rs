use serde::{Deserialize, Serialize};

/// Represents a valid [Attachment Type] in Smartsheet.
///
/// [Attachment Type]: https://smartsheet-platform.github.io/api-docs/#objects
///
// noinspection SpellCheckingInspection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttachmentType {
    BoxCom,
    Dropbox,
    Egnyte,
    Evernote,
    File,
    GoogleDrive,
    Link,
    Onedrive,
}

impl Default for AttachmentType {
    fn default() -> Self {
        Self::File
    }
}
