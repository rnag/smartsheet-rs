use crate::models::Criteria;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub criteria: Vec<Criteria>,
    #[serde(rename = "includeParent")]
    pub include_parent: bool,
    pub operator: String,
}
