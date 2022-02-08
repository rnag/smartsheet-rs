use crate::models::{Attachment, Cell, Column, Discussion, User};
use crate::types::Result;

use core::option::Option;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    pub id: u64,
    #[serde(rename = "sheet_id")]
    pub sheet_id: Option<u64>,
    #[serde(rename = "rowNumber")]
    pub row_number: u64,
    pub expanded: bool,
    pub cells: Vec<Cell>,
    #[serde(default)]
    pub columns: Vec<Column>,
    pub attachments: Option<Vec<Attachment>>,
    pub discussions: Option<Vec<Discussion>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    pub created_by: Option<User>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "siblingId")]
    pub sibling_id: Option<u64>,
    #[serde(rename = "filteredOut")]
    pub filtered_out: Option<bool>,
    pub format: Option<String>,
    #[serde(rename = "modifiedBy")]
    pub modified_by: Option<User>,
    pub permalink: Option<String>,
    #[serde(rename = "accessLevel")]
    pub access_level: Option<String>,
    pub version: Option<u64>,
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
