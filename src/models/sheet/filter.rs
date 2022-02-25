use crate::models::Query;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub filter_type: String,
    pub id: i64,
    pub name: String,
    pub query: Query,
    pub version: Option<i64>,
}
