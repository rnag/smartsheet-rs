use crate::models::EnumStr;

use std::fmt;

#[derive(Debug)]
/// Row Include Flags are documented here:
///   https://smartsheet-platform.github.io/api-docs/#row-cells
pub enum RowIncludeFlags {
    Attachments,
    Columns,
    ColumnType,
    Discussions,
    Filters,
    Format,
    ObjectValue,
    RowPermalink,
    RowWriterInfo,
    WriterInfo,
}

impl EnumStr for RowIncludeFlags {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::Attachments => "attachments",
            Self::Columns => "columns",
            Self::ColumnType => "columnType",
            Self::Discussions => "discussions",
            Self::Filters => "filters",
            Self::Format => "format",
            Self::ObjectValue => "objectValue",
            Self::RowPermalink => "rowPermalink",
            Self::RowWriterInfo => "rowWriterInfo",
            Self::WriterInfo => "writerInfo",
        }
    }
}

impl fmt::Display for RowIncludeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
/// Ref: https://smartsheet-platform.github.io/api-docs/#row-cells
pub enum RowExcludeFlags {
    /// excludes the following attributes from the cell.linkInFromCell object:
    ///   - columnId
    ///   - rowId
    ///   - status
    LinkInFromCellDetails,
    /// excludes the following attributes from the cell.linksOutToCells object:
    ///   - columnId
    ///   - rowId
    ///   - status
    LinkOutToCellDetails,
    /// excludes cells that have never contained any data
    NonExistentCells,
}

impl EnumStr for RowExcludeFlags {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::LinkInFromCellDetails => "linkInFromCellDetails",
            Self::LinkOutToCellDetails => "linksOutToCellsDetails",
            Self::NonExistentCells => "nonexistentCells",
        }
    }
}

impl fmt::Display for RowExcludeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
