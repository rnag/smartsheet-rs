use crate::models::{AccessLevel, Attachment, Cell, Column, Discussion, User};
use crate::types::Result;
use crate::utils::is_default;

use core::option::Option;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    /// Row Id
    #[serde(skip_serializing_if = "is_default")]
    pub id: u64,
    /// Parent Sheet Id
    #[serde(skip_serializing)]
    pub sheet_id: Option<u64>,
    /// Enum: ADMIN, EDITOR, EDITOR_SHARE, OWNER, VIEWER
    #[serde(skip_serializing)]
    pub access_level: Option<AccessLevel>,
    /// Attachments on row. Only returned if the include query string
    /// parameter contains attachments.
    #[serde(skip_serializing)]
    pub attachments: Option<Vec<Attachment>>,
    /// Cells belonging to the row.
    pub cells: Vec<Cell>,
    /// Columns of row. Only returned if the include query string parameter
    /// contains columns.
    #[serde(default)]
    #[serde(skip_serializing)]
    pub columns: Vec<Column>,
    /// Describes this row's conditional format. Only returned if the include
    /// query string parameter contains format and this row has a conditional
    /// format applied.
    #[serde(skip_serializing)]
    pub conditional_format: Option<String>,
    /// string or number
    #[serde(skip_serializing)]
    pub created_at: String,
    /// User object containing name and email of the creator of this row.
    #[serde(skip_serializing)]
    pub created_by: Option<User>,
    /// Discussions on the row. Only returned if the include query string
    /// parameter contains discussions.
    #[serde(skip_serializing)]
    pub discussions: Option<Vec<Discussion>>,
    /// Indicates whether the row is expanded or collapsed.
    #[serde(skip_serializing)]
    pub expanded: bool,
    /// Indicates if the row is filtered out by a column filter. Only returned
    /// if the include query string parameter contains filters.
    pub filtered_out: Option<bool>,
    /// Format descriptor. Only returned if the include query string parameter
    /// contains format and this row has a non-default format applied.
    #[serde(skip_serializing)]
    pub format: Option<String>,
    /// Only returned, with a value of true, if the sheet is a project sheet
    /// with dependencies enabled and this row is in the critical path.
    pub in_critical_path: Option<bool>,
    /// Indicates whether the row is locked.
    pub locked: Option<bool>,
    /// Indicates whether the row is locked for the requesting user.
    pub locked_for_user: Option<bool>,
    /// string or number
    #[serde(skip_serializing)]
    pub modified_at: String,
    /// User object containing name and email of the last person to modify
    /// this row.
    #[serde(skip_serializing)]
    pub modified_by: Option<User>,
    /// URL that represents a direct link to the row in Smartsheet. Only
    /// returned if the include query string parameter contains `rowPermalink`.
    #[serde(skip_serializing)]
    pub permalink: Option<String>,
    /// Row number within the sheet.
    #[serde(skip_serializing)]
    pub row_number: u64,
    /// Sheet version number that is incremented every time a sheet is modified.
    #[serde(skip_serializing)]
    pub version: Option<u64>,
    /// Sibling Row Id
    pub sibling_id: Option<u64>,
    // TODO: Add Proof field
    // Proof Object
    // pub proofs: Proofs
}

impl Row {
    /// Retrieve a specified `Cell` - for a given *column id* - from the `Row`
    pub fn get_cell_by_id(&self, column_id: u64) -> Result<&Cell> {
        for cell in &self.cells {
            if cell.column_id == column_id {
                return Ok(cell);
            }
        }
        Err(Box::from(Error::new(
            ErrorKind::NotFound,
            "No cell found for the given Column ID or Name",
        )))
    }
}
