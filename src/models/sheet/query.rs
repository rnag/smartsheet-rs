use crate::models::Criteria;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub criteria: Vec<Criteria>,
    pub include_parent: bool,
    pub operator: String,
}
