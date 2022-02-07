use crate::models::sheet::Heading;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GanttConfig {
    #[serde(rename = "firstDayOfWeek")]
    pub first_day_of_week: String,
    #[serde(rename = "fiscalYearBegins")]
    pub fiscal_year_begins: String,
    pub holidays: Vec<Value>,
    #[serde(rename = "hoursInWorkingDay")]
    pub hours_in_working_day: f64,
    #[serde(rename = "primaryHeading")]
    pub primary_heading: Heading,
    #[serde(rename = "secondaryHeading")]
    pub secondary_heading: Heading,
    #[serde(rename = "workingDays")]
    pub working_days: Vec<String>,
}
