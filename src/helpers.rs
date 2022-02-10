//! Public helper utilities
//!
use crate::models::{Cell, Column, Row};
use crate::types::Result;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};

/// Define type aliases for the column mappings so that we can DRY.
type ColumnNameToId<'a> = HashMap<&'a str, u64>;
type ColumnIdToName<'a> = HashMap<u64, &'a str>;
type ColumnNameToCell<'a> = HashMap<&'a str, &'a Cell>;

/// **Column Mapper** - Utility to generate the `name` <-> `id` mappings for
/// columns in a sheet.
///
pub struct ColumnMapper<'a> {
    /// Represents a mapping of *Column Name* to *Column ID*
    ///
    /// Note that the ID value is unique, internal, and used mainly in the
    /// Smartsheet API.
    pub name_to_id: ColumnNameToId<'a>,
    /// Represents a mapping of *Column ID* to *Column Name*
    ///
    /// Note that the ID value is unique, internal, and used mainly in the
    /// Smartsheet API.
    pub id_to_name: ColumnIdToName<'a>,
}

impl<'a> ColumnMapper<'a> {
    /// Create a new `ColumnMapper` object from a list of `columns`.
    pub fn new(columns: &'a [Column]) -> Self {
        let (name_to_id, id_to_name) = Self::get_mappings(columns);

        Self {
            name_to_id,
            id_to_name,
        }
    }

    /// Retrieve the `name` <-> `id` mappings for *columns* in a sheet.
    fn get_mappings(columns: &'a [Column]) -> (ColumnNameToId, ColumnIdToName) {
        let num_columns = columns.len();
        if num_columns == 0 {
            // TODO maybe don't panic
            panic!("No column data for the Row - please ensure you call `get_row_with_column_data()` *or* \
        pass `RowIncludeFlags::Columns` as an `include` argument to `get_row_with_params()`")
        }

        let mut name_to_id: ColumnNameToId<'a> = HashMap::with_capacity(num_columns);
        let mut id_to_name: ColumnIdToName<'a> = HashMap::with_capacity(num_columns);

        for c in columns {
            let title = &c.title;

            name_to_id.insert(title, c.id);
            id_to_name.insert(c.id, title);
        }

        (name_to_id, id_to_name)
    }
}

/// **Cell Getter** - Utility to make it easier to retrieve a `Cell` from a
/// `Row` object.
///
pub struct CellGetter<'a> {
    column_name_to_id: &'a ColumnNameToId<'a>,
    id_to_column_name: &'a ColumnIdToName<'a>,
}

impl<'a> CellGetter<'a> {
    /// Create a new `CellGetter` from a reference to a `ColumnMapper` object
    pub fn new(columns: &'a ColumnMapper<'a>) -> Self {
        Self {
            column_name_to_id: &columns.name_to_id,
            id_to_column_name: &columns.id_to_name,
        }
    }

    /// Create a new `CellGetter` from a reference to a `ColumnMapper` object
    pub fn from_mapper(columns: &'a ColumnMapper<'a>) -> Self {
        Self::new(columns)
    }

    /// Create a new `CellGetter` from a reference to a mapping of *column name*
    /// to *column id*
    ///
    /// NOTE: Disabling this for now, because I can't get the lifetimes to work.
    // pub fn from_name_to_id<'b>(column_name_to_id: &'b ColumnNameToId<'b>) -> CellGetter<'a> {
    //     let mut id_to_column_name: ColumnIdToName<'b> =
    //         HashMap::with_capacity(column_name_to_id.len());
    //     for (&name, &id) in column_name_to_id {
    //         id_to_column_name.insert(id, name);
    //     }
    //
    //     Self {
    //         column_name_to_id,
    //         id_to_column_name: &id_to_column_name,
    //     }
    // }

    /// Find a `cell` in a `row` by its *column name*.
    pub fn by_name<'b>(&'a self, row: &'a Row, name: &'b str) -> Result<&Cell> {
        match self.column_name_to_id.get(name) {
            Some(&col_id) => row.get_cell_by_id(col_id),
            None => Err(Box::from(Error::new(
                ErrorKind::InvalidInput,
                format!("A column named `{}` was not found in the Sheet", name),
            ))),
        }
    }

    /// Find a `cell` in a `row` by its *column id*.
    pub fn by_id(&'a self, row: &'a Row, column_id: u64) -> Result<&Cell> {
        row.get_cell_by_id(column_id)
    }

    /// Retrieve a mapping of *column name* to `Cell` object.
    ///
    /// Note: this is likely more efficient when multiple `Cell`s are to be
    /// retrieved from an individual `Row`.
    pub fn name_to_cell(&'a self, row: &'a Row) -> ColumnNameToCell<'a> {
        let mut col_name_to_cell: ColumnNameToCell<'a> = HashMap::with_capacity(row.cells.len());

        for cell in &row.cells {
            if let Some(&col_name) = self.id_to_column_name.get(&cell.column_id) {
                col_name_to_cell.insert(col_name, cell);
            }
        }

        col_name_to_cell
    }
}
