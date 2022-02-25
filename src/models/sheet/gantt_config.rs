use crate::models::sheet::Heading;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GanttConfig {
    pub first_day_of_week: String,
    pub fiscal_year_begins: String,
    pub holidays: Vec<Value>,
    pub hours_in_working_day: f64,
    pub primary_heading: Heading,
    pub secondary_heading: Heading,
    pub working_days: Vec<String>,
}
