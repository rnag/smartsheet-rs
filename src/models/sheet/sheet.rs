use crate::models::{
    Column, Filter, GanttConfig, Row, Source, UserPermissions, UserSettings, Workspace,
};
use core::option::Option;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sheet {
    pub id: u64,
    pub name: String,
    pub owner: Option<String>,
    #[serde(rename = "ownerId")]
    pub owner_id: Option<u64>,
    #[serde(default)]
    pub rows: Vec<Row>,
    #[serde(default)]
    pub columns: Vec<Column>,
    #[serde(rename = "totalRowCount")]
    #[serde(default)]
    pub total_row_count: u64,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "accessLevel")]
    pub access_level: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub permalink: String,
    pub source: Option<Source>,
    pub favorite: Option<bool>,
    pub filters: Option<Vec<Filter>>,
    pub version: Option<u64>,
    #[serde(rename = "cellImageUploadEnabled")]
    pub cell_image_upload_enabled: Option<bool>,
    #[serde(rename = "dependenciesEnabled")]
    pub dependencies_enabled: Option<bool>,
    #[serde(rename = "effectiveAttachmentOptions")]
    #[serde(default)]
    pub effective_attachment_options: Vec<String>,
    #[serde(rename = "ganttConfig")]
    pub gantt_config: Option<GanttConfig>,
    #[serde(rename = "ganttEnabled")]
    pub gantt_enabled: Option<bool>,
    #[serde(rename = "hasSummaryFields")]
    pub has_summary_fields: Option<bool>,
    #[serde(rename = "isMultiPicklistEnabled")]
    pub is_multi_picklist_enabled: Option<bool>,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "resourceManagementEnabled")]
    pub resource_management_enabled: Option<bool>,
    #[serde(rename = "resourceManagementType")]
    pub resource_management_type: Option<String>,
    #[serde(rename = "userPermissions")]
    #[serde(default)]
    pub user_permissions: UserPermissions,
    #[serde(rename = "userSettings")]
    #[serde(default)]
    pub user_settings: UserSettings,
    #[serde(rename = "workspace")]
    pub workspace: Option<Workspace>,
}
