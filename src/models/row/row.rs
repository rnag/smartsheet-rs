use crate::models::{AccessLevel, AttachmentMeta, Cell, Column, Discussion, IndentEnabled, User};
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
    pub attachments: Option<Vec<AttachmentMeta>>,
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
    // TODO: Add Proof field
    // Proof Object
    // pub proofs: Proofs
    /// # Note
    ///
    /// The following are used in the [Row Location] specified attributes.
    ///
    /// [Row Location]: https://smartsheet.redoc.ly/#section/Specify-Row-Location

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
}

impl<const N: usize> From<&[Cell; N]> for Row {
    fn from(cells: &[Cell; N]) -> Self {
        Self {
            cells: cells.to_vec(),
            ..Default::default()
        }
    }
}

impl From<Vec<Cell>> for Row {
    fn from(cells: Vec<Cell>) -> Self {
        Self {
            cells,
            ..Default::default()
        }
    }
}

impl Row {
    pub fn with_cells<C: Into<Vec<Cell>>>(cells: C) -> Self {
        Row {
            cells: cells.into(),
            ..Default::default()
        }
    }

    pub fn with_id_and_cells<C: Into<Vec<Cell>>>(row_id: u64, cells: C) -> Self {
        Row {
            id: row_id,
            cells: cells.into(),
            ..Default::default()
        }
    }

    pub fn with_id_and_cells_slice<const N: usize>(row_id: u64, cells: &[Cell; N]) -> Self {
        Row {
            id: row_id,
            cells: cells.to_vec(),
            ..Default::default()
        }
    }

    /// Retrieve a specified `Cell` - for a given *column id* - from the `Row`
    pub fn get_cell_by_id(&self, column_id: u64) -> Result<&Cell> {
        return match self.cells.iter().find(|cell| cell.column_id == column_id) {
            Some(cell) => Ok(cell),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                "No cell found for the given Column ID or Name",
            ))),
        };
    }

    /// Fluent setter for the `id` attribute (Row Id)
    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    /// Fluent setter for the `expanded` attribute
    pub fn expanded<B: Into<Option<bool>>>(mut self, expanded: B) -> Self {
        self.expanded = expanded.into();
        self
    }

    /// Fluent setter for the `format` attribute
    pub fn format<S: Into<String>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Fluent setter for the `locked` attribute
    pub fn locked<B: Into<Option<bool>>>(mut self, locked: B) -> Self {
        self.locked = locked.into();
        self
    }

    /// Fluent setter for the `sibling_id` attribute
    pub fn sibling_id<U: Into<Option<u64>>>(mut self, sibling_id: U) -> Self {
        self.sibling_id = sibling_id.into();
        self
    }

    /// Fluent setter for the `parent_id` attribute
    pub fn parent_id<U: Into<Option<u64>>>(mut self, parent_id: U) -> Self {
        self.parent_id = parent_id.into();
        self
    }

    /// Fluent setter for the `to_top` attribute
    pub fn to_top<B: Into<Option<bool>>>(mut self, to_top: B) -> Self {
        self.to_top = to_top.into();
        self
    }

    /// Fluent setter for the `to_bottom` attribute
    pub fn to_bottom<B: Into<Option<bool>>>(mut self, to_bottom: B) -> Self {
        self.to_bottom = to_bottom.into();
        self
    }

    /// Fluent setter for the `indent` attribute
    pub fn indent<I: Into<Option<IndentEnabled>>>(mut self, indent: I) -> Self {
        self.indent = indent.into();
        self
    }

    /// Fluent setter for the `outdent` attribute
    pub fn outdent<I: Into<Option<IndentEnabled>>>(mut self, outdent: I) -> Self {
        self.outdent = outdent.into();
        self
    }
}

impl From<Row> for Vec<Row> {
    /// Useful when adding / updating row(s) to a sheet.
    fn from(row: Row) -> Self {
        vec![row]
    }
}

/// *Row Location Specifier* - A trait which allows us to define the
/// [Row Location] specified attributes for a collection (array or vector) of
/// `Row` objects.
///
/// [Row Location]: https://smartsheet.redoc.ly/#section/Specify-Row-Location
pub trait RowLocationSpecifier {
    /// Fluent setter for the `sibling_id` attribute
    fn sibling_id<U: Into<Option<u64>>>(&mut self, sibling_id: U) -> &mut Self;
    /// Fluent setter for the `parent_id` attribute
    fn parent_id<U: Into<Option<u64>>>(&mut self, parent_id: U) -> &mut Self;
    /// Fluent setter for the `to_top` attribute
    fn to_top<B: Into<Option<bool>>>(&mut self, to_top: B) -> &mut Self;
    /// Fluent setter for the `to_bottom` attribute
    fn to_bottom<B: Into<Option<bool>>>(&mut self, to_bottom: B) -> &mut Self;
    /// Fluent setter for the `indent` attribute
    fn indent<I: Into<Option<IndentEnabled>>>(&mut self, indent: I) -> &mut Self;
    /// Fluent setter for the `outdent` attribute
    fn outdent<I: Into<Option<IndentEnabled>>>(&mut self, outdent: I) -> &mut Self;
}

macro_rules! impl_row_location_specifier {
    (for $($t:ty),+) => {
        $(impl RowLocationSpecifier for $t {

            fn sibling_id<U: Into<Option<u64>>>(&mut self, sibling_id: U) -> &mut Self {
                let sibling_id = sibling_id.into();
                for row in self.iter_mut() {
                    row.sibling_id = sibling_id;
                }
                self
            }

            fn parent_id<U: Into<Option<u64>>>(&mut self, parent_id: U) -> &mut Self {
                let parent_id = parent_id.into();
                for row in self.iter_mut() {
                    row.parent_id = parent_id;
                }
                self
            }

            fn to_top<B: Into<Option<bool>>>(&mut self, to_top: B) -> &mut Self {
                let to_top = to_top.into();
                for row in self.iter_mut() {
                    row.to_top = to_top;
                }
                self
            }

            fn to_bottom<B: Into<Option<bool>>>(&mut self, to_bottom: B) -> &mut Self {
                let to_bottom = to_bottom.into();
                for row in self.iter_mut() {
                    row.to_bottom = to_bottom;
                }
                self
            }

            fn indent<I: Into<Option<IndentEnabled>>>(&mut self, indent: I) -> &mut Self {
                let indent = indent.into();
                for row in self.iter_mut() {
                    row.indent = indent;
                }
                self
            }

            fn outdent<I: Into<Option<IndentEnabled>>>(&mut self, outdent: I) -> &mut Self {
                let outdent = outdent.into();
                for row in self.iter_mut() {
                    row.outdent = outdent;
                }
                self
            }
        })*
    }
}

impl_row_location_specifier!(for Vec<Row>, [Row]);

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use serde_json::to_string_pretty;

    #[test]
    fn test_expanded() {
        let row = Row::default();
        assert_eq!(row.expanded, None);

        let row = row.expanded(true);

        assert_eq!(row.expanded, Some(true));
    }

    #[test]
    fn test_format() {
        let row = Row::default();
        assert_eq!(row.format, None);

        let row = row.format("test");
        assert_eq!(row.format, Some("test".to_owned()));
    }

    /// Test that only the [required fields] are populated when serializing
    /// a `Row` object.
    ///
    /// [required fields]: https://smartsheet.redoc.ly/#tag/rows
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
            indent: Some(IndentEnabled::TRUE),
            outdent: Some(IndentEnabled::TRUE),
        };

        let s = to_string_pretty(&row).unwrap();

        println!("{}", s);

        assert_eq!(
            s,
            indoc!(
                r#"
            {
              "id": 123,
              "cells": [],
              "expanded": false,
              "format": "my fmt",
              "locked": false,
              "siblingId": 123,
              "parentId": 321,
              "toTop": true,
              "toBottom": true,
              "indent": 1,
              "outdent": 1
            }
                "#
            )
            .trim()
        )
    }
}
