use crate::models::{Attachment, Cell, Column, Discussion, User};
use crate::types::Result;

use core::option::Option;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    #[serde(skip_serializing_if = "is_default")]
    pub id: u64,
    #[serde(rename = "sheet_id")]
    #[serde(skip_serializing)]
    pub sheet_id: Option<u64>,
    #[serde(rename = "rowNumber")]
    #[serde(skip_serializing)]
    pub row_number: u64,
    #[serde(skip_serializing)]
    pub expanded: bool,
    pub cells: Vec<Cell>,
    #[serde(default)]
    #[serde(skip_serializing)]
    pub columns: Vec<Column>,
    #[serde(skip_serializing)]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing)]
    pub discussions: Option<Vec<Discussion>>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing)]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing)]
    pub created_by: Option<User>,
    #[serde(rename = "modifiedAt")]
    #[serde(skip_serializing)]
    pub modified_at: String,
    #[serde(rename = "siblingId")]
    pub sibling_id: Option<u64>,
    #[serde(rename = "filteredOut")]
    pub filtered_out: Option<bool>,
    #[serde(skip_serializing)]
    pub format: Option<String>,
    #[serde(rename = "modifiedBy")]
    #[serde(skip_serializing)]
    pub modified_by: Option<User>,
    #[serde(skip_serializing)]
    pub permalink: Option<String>,
    #[serde(rename = "accessLevel")]
    #[serde(skip_serializing)]
    pub access_level: Option<String>,
    #[serde(skip_serializing)]
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
