use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criteria {
    #[serde(rename = "columnId")]
    pub column_id: u64,
    pub operator: String,
    #[serde(default)]
    pub values: Vec<Value>,
}
