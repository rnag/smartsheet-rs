use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub id: u64,
    #[serde(rename = "type")]
    pub type_field: String,
}
