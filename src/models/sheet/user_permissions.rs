use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPermissions {
    #[serde(rename = "summaryPermissions")]
    pub summary_permissions: String,
}
