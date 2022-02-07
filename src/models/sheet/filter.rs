use crate::models::Query;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "filterType")]
    pub filter_type: String,
    pub id: i64,
    pub name: String,
    pub query: Query,
    pub version: Option<i64>,
}
