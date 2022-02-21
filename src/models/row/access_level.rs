use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccessLevel {
    Admin,
    Editor,
    EditorShare,
    Owner,
    Viewer,
}
