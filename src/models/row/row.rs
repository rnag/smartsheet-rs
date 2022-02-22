use crate::models::{AccessLevel, Attachment, Cell, Column, Discussion, IndentEnabled, User};
use crate::types::Result;
use crate::utils::is_default;

use core::option::Option;
use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    /// Indicates if the row is filtered out by a column filter. Only returned
    /// if the include query string parameter contains filters.
    #[serde(skip_serializing)]
    pub filtered_out: Option<bool>,
    /// Format descriptor. Only returned if the include query string parameter
    /// contains format and this row has a non-default format applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Only returned, with a value of true, if the sheet is a project sheet
    /// with dependencies enabled and this row is in the critical path.
    #[serde(skip_serializing)]
    pub in_critical_path: Option<bool>,
    /// Indicates whether the row is locked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Indicates whether the row is locked for the requesting user.
    #[serde(skip_serializing)]
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
    ///
    /// Also used to [specify row location] when adding/updating rows.
    ///
    /// [specify row location]: https://smartsheet.redoc.ly/#section/Specify-Row-Location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sibling_id: Option<u64>,
    /// Parent Id, used to [specify row location] when adding/updating rows.
    ///
    /// [specify row location]: https://smartsheet.redoc.ly/#section/Specify-Row-Location
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,
    /// Row Location Specifier: Top of a sheet
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_top: Option<bool>,
    /// Row Location Specifier: Bottom of a sheet
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_bottom: Option<bool>,
    /// Row Location Specifier: Indent one existing row, must have a value of "1"
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<IndentEnabled>,
    /// Row Location Specifier: Outdent one existing row, must have a value of "1"
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outdent: Option<IndentEnabled>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string_pretty;

    #[test]
    fn test_serialize_row() {
        let row = Row {
            id: 123,
            sheet_id: Some(321),
            access_level: Some(AccessLevel::Admin),
            attachments: Some(vec![]),
            cells: vec![],
            columns: vec![],
            conditional_format: Some("conditional fmt".to_owned()),
            created_at: "abc".to_owned(),
            created_by: Some(User {
                email: "a@b.com".to_owned(),
                name: Some("Test".to_owned()),
            }),
            discussions: Some(vec![]),
            expanded: Some(false),
            filtered_out: Some(true),
            format: Some("my fmt".to_owned()),
            in_critical_path: Some(true),
            locked: Some(false),
            locked_for_user: Some(true),
            modified_at: "abc".to_owned(),
            modified_by: Some(User {
                email: "z@a.com".to_owned(),
                name: Some("My Name".to_owned()),
            }),
            permalink: Some("test link".to_owned()),
            row_number: 123,
            version: Some(111),
            sibling_id: Some(123),
            parent_id: Some(321),
            to_top: Some(true),
            to_bottom: Some(true),
            indent: None,
            outdent: Some(IndentEnabled::TRUE),
        };

        let s = to_string_pretty(&row).unwrap();

        println!("{}", s);
    }
}
