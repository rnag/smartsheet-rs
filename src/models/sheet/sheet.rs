use crate::models::{
    Column, Filter, GanttConfig, Row, Source, UserPermissions, UserSettings, Workspace,
};
use crate::types::Result;

use core::option::Option;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

pub type RowIdToRow<'a> = HashMap<u64, &'a Row>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    pub id: u64,
    pub name: String,
    pub owner: Option<String>,
    pub owner_id: Option<u64>,
    #[serde(default)]
    pub rows: Vec<Row>,
    #[serde(default)]
    pub columns: Vec<Column>,
    #[serde(default)]
    pub total_row_count: u64,
    pub modified_at: String,
    pub access_level: String,
    pub created_at: String,
    pub permalink: String,
    pub source: Option<Source>,
    pub favorite: Option<bool>,
    pub filters: Option<Vec<Filter>>,
    pub version: Option<u64>,
    pub cell_image_upload_enabled: Option<bool>,
    pub dependencies_enabled: Option<bool>,
    #[serde(default)]
    pub effective_attachment_options: Vec<String>,
    pub gantt_config: Option<GanttConfig>,
    pub gantt_enabled: Option<bool>,
    pub has_summary_fields: Option<bool>,
    pub is_multi_picklist_enabled: Option<bool>,
    pub read_only: Option<bool>,
    pub resource_management_enabled: Option<bool>,
    pub resource_management_type: Option<String>,
    #[serde(default)]
    pub user_permissions: UserPermissions,
    #[serde(default)]
    pub user_settings: UserSettings,
    pub workspace: Option<Workspace>,
}

impl Sheet {
    /// Retrieve a specified `Row` - for a given *row id* - from the `Sheet`
    pub fn get_row_by_id(&self, row_id: u64) -> Result<&Row> {
        return match self.rows.iter().find(|row| row.id == row_id) {
            Some(row) => Ok(row),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                "No row found for the given Row ID",
            ))),
        };
    }

    /// Retrieve a mapping of *row id* to `Row` object.
    ///
    /// Note: this is likely more efficient when multiple `Row`s are to be
    /// retrieved based on their *row id*.
    pub fn id_to_row(&self) -> RowIdToRow {
        self.rows.iter().map(|row| (row.id, row)).collect()
    }
}
