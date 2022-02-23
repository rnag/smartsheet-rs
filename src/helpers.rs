//! Public helper utilities
//!
use crate::models::{Cell, CellValue, Column, Row};
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

/// **Row Getter** - Utility to make it easier to find and retrieve row(s)
/// in an array of `Row` objects, based on a predicate or a pre-defined
/// filter condition.
///
/// For example, a common use case is finding a `Row` where a cell is
/// *equal* to a specific value.
///
pub struct RowGetter<'a> {
    /// Represents an array of *Row* objects that we want to run a search on.
    pub rows: &'a [Row],
    /// Represents a mapping of *Column Name* to *Column ID*
    ///
    /// Note that the ID value is unique, internal, and used mainly in the
    /// Smartsheet API.
    pub column_name_to_id: &'a ColumnNameToId<'a>,
}

impl<'a> RowGetter<'a> {
    /// Create a new `RowGetter` from a reference to a `ColumnMapper` object
    pub fn new(rows: &'a [Row], columns: &'a ColumnMapper<'a>) -> RowGetter<'a> {
        Self {
            rows,
            column_name_to_id: &columns.name_to_id,
        }
    }

    /// Uses an **equals (eq)** condition to compare a cell for a *Column
    /// Name* against a specified *Value*.
    pub fn where_eq<V: Into<CellValue>>(
        &'a self,
        column_name: &'a str,
        value: V,
    ) -> Result<RowFinder<'a>> {
        RowFinder::new(
            self.rows,
            self.column_name_to_id,
            column_name,
            value.into(),
            Comp::EQ,
        )
    }

    /// Uses an **equals (eq)** condition to compare a cell for a *Column
    /// ID* against a specified *Value*.
    pub fn where_eq_by_id<V: Into<CellValue>>(&'a self, column_id: u64, value: V) -> RowFinder<'a> {
        RowFinder::new_by_id(self.rows, column_id, value.into(), Comp::EQ)
    }

    /// Uses a **not equals (ne)** condition to compare a cell for a *Column
    /// Name* against a specified *Value*.
    pub fn where_ne<V: Into<CellValue>>(
        &'a self,
        column_name: &'a str,
        value: V,
    ) -> Result<RowFinder<'a>> {
        RowFinder::new(
            self.rows,
            self.column_name_to_id,
            column_name,
            value.into(),
            Comp::NE,
        )
    }

    /// Uses a **not equals (ne)** condition to compare a cell for a *Column
    /// ID* against a specified *Value*.
    pub fn where_ne_by_id<V: Into<CellValue>>(&'a self, column_id: u64, value: V) -> RowFinder<'a> {
        RowFinder::new_by_id(self.rows, column_id, value.into(), Comp::NE)
    }
}

/// Enum which represents a Comparison operator or a Search Criteria
pub enum Comp {
    EQ,
    NE,
}

impl Comp {
    /// Return a closure which compares two `CellValue`s to determine
    /// equality.
    pub fn get_cell_comparator<'a>(&'a self) -> fn(&'a CellValue, &'a CellValue) -> bool {
        match self {
            Comp::EQ => |v1: &'a CellValue, v2: &'a CellValue| v1 == v2,
            Comp::NE => |v1: &'a CellValue, v2: &'a CellValue| v1 != v2,
        }
    }
}

/// **Row Finder**: Find row(s) in an array of `Row`s that match a pre-defined
/// condition.
///
/// # Note
/// It's preferable to use the [`RowGetter`] implementation instead, as
/// that's a little easier to work with.
///
pub struct RowFinder<'a> {
    /// Represents an array of *Row* objects that we want to run a search on.
    pub rows: &'a [Row],
    /// Column Id to filter the value by.
    column_id: u64,
    /// Value to filter by.
    value: CellValue,
    /// Determines how we intend to compare cell value against `value`.
    cmp: Comp,
}

impl<'a> RowFinder<'a> {
    /// Create a new `RowFinder`.
    ///
    /// # Note
    /// It's preferable to use the [`RowGetter`] implementation instead, as
    /// that's a little easier to work with.
    ///
    pub fn new(
        rows: &'a [Row],
        column_name_to_id: &'a ColumnNameToId<'a>,
        column_name: &'a str,
        value: CellValue,
        cmp: Comp,
    ) -> Result<Self> {
        let column_id = match column_name_to_id.get(column_name) {
            Some(&v) => v,
            None => {
                return Err(Box::from(Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "The column name `{}` does not exist in the sheet",
                        column_name
                    ),
                )));
            }
        };

        Ok(Self::new_by_id(rows, column_id, value, cmp))
    }

    /// Create a new `RowFinder` by *Column Id* instead of *Column Name*.
    pub fn new_by_id(rows: &'a [Row], column_id: u64, value: CellValue, cmp: Comp) -> Self {
        Self {
            rows,
            column_id,
            value,
            cmp,
        }
    }

    /// Find the *first* `Row` matching a specified condition.
    pub fn first<'b>(&'b self) -> Result<&'a Row> {
        let cmp = self.cmp.get_cell_comparator();

        return match self.rows.iter().find(|row| {
            if let Ok(cell) = row.get_cell_by_id(self.column_id) {
                matches!(&cell.value, Some(cv) if cmp(&self.value, cv))
            } else {
                false
            }
        }) {
            Some(row) => Ok(row),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                "No matching row for the condition",
            ))),
        };
    }

    /// Find *all* `Row`s matching a specified condition.
    pub fn find_all<'b>(&'b self) -> Result<Vec<&'a Row>> {
        let cmp = self.cmp.get_cell_comparator();

        Ok(self
            .rows
            .iter()
            .filter(|row| {
                if let Ok(cell) = row.get_cell_by_id(self.column_id) {
                    matches!(&cell.value, Some(cv) if cmp(&self.value, cv))
                } else {
                    false
                }
            })
            .collect::<Vec<_>>())
    }
}
