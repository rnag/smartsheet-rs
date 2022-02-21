use crate::models::EnumStr;

use std::fmt;

#[derive(Debug)]
/// Row Include Flags are documented here:
///   https://smartsheet.redoc.ly/#section/Row-Include-Flags
pub enum RowIncludeFlags {
    /// Includes row `attachments` array.
    ///
    /// To include discussion attachments, both `attachments` and
    /// `discussions` must be present in the include list.
    Attachments,
    /// Adds a `columns` array that specifies all of the columns for the
    /// sheet. This enables you to have the full context of the cells in the
    /// row.
    Columns,
    /// Includes `columnType` attribute in the row's cells indicating the type
    /// of the column the cell resides in.
    ColumnType,
    /// Includes row `discussions` array.
    ///
    /// To include discussion attachments, both `attachments` and `discussions`
    /// must be present in the include list.
    Discussions,
    /// Includes `filteredOut` attribute indicating if the row should be
    /// displayed or hidden according to the sheet's filters.
    Filters,
    /// Includes `format` attribute on the row, its cells, or summary fields.
    /// See [Formatting](https://smartsheet.redoc.ly/#section/API-Basics/Formatting).
    Format,
    /// Includes `objectValue` attribute on cells containing values. For more
    /// information see [Cell Reference](https://smartsheet.redoc.ly/tag/cellsRelated#section/Cell-Reference).
    ObjectValue,
    /// Includes `permalink` attribute that represents a direct link to the
    /// row in the Smartsheet application.
    RowPermalink,
    /// **DEPRECATED** Includes `createdBy` and `modifiedBy` attributes on the
    /// row, indicating the row's creator, and last modifier.
    #[deprecated(note = "Use `RowIncludeFlags::WriterInfo` instead")]
    RowWriterInfo,
    /// Includes `createdBy` and `modifiedBy` attributes on the row or summary
    /// fields, indicating the row or summary field's creator, and last modifier.
    WriterInfo,
}

impl EnumStr for RowIncludeFlags {
    #[allow(deprecated)]
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
