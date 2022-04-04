use serde::{Deserialize, Serialize};

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
