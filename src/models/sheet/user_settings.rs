use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    #[serde(rename = "criticalPathEnabled")]
    pub critical_path_enabled: bool,
    #[serde(rename = "displaySummaryTasks")]
    pub display_summary_tasks: bool,
}
