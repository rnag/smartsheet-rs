use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Criteria {
    pub column_id: u64,
    pub operator: String,
    #[serde(default)]
    pub values: Vec<Value>,
}
