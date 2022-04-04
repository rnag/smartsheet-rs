use serde::{Deserialize, Serialize};

/// The type of object the attachment belongs to.
///
/// # Docs
/// <https://smartsheet-platform.github.io/api-docs/#objects>
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParentType {
    Comment,
    Row,
    Sheet,
}

impl Default for ParentType {
    fn default() -> Self {
        Self::Row
    }
}
