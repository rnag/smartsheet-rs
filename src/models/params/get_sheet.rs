use crate::models::params::{RowExcludeFlags, RowIncludeFlags};
use crate::models::EnumStr;

use std::fmt;

/// Get Sheet Include Flags are documented here:
///   https://smartsheet-platform.github.io/api-docs/#get-sheet
///   https://stackoverflow.com/q/25214064/10237506
#[derive(Debug)]
pub enum SheetIncludeFlags {
    Base(RowIncludeFlags),
    // crossSheetReferences: includes the cross-sheet references
    CrossSheetReferences,
    // filterDefinitions: includes type of filter, operators used, and criteria
    FilterDefinitions,
    // ganttConfig: includes Gantt chart details
    GanttConfig,
    // ownerInfo: includes the workspace and the owner's email address and user Id
    OwnerInfo,
    // proofs: includes metadata for proofs at the row level
    Proofs,
    // source: adds the Source object indicating which dashboard, report,
    // sheet, or template the sheet was created from, if any.
    Source,
}

impl EnumStr for SheetIncludeFlags {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::Base(flags) => flags.as_str(),
            Self::CrossSheetReferences => "crossSheetReferences",
            Self::FilterDefinitions => "filterDefinitions",
            Self::GanttConfig => "ganttConfig",
            Self::OwnerInfo => "ownerInfo",
            Self::Proofs => "proofs",
            Self::Source => "source",
        }
    }
}

impl fmt::Display for SheetIncludeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Get Sheet Exclude Flags are documented here:
///   https://smartsheet-platform.github.io/api-docs/#get-sheet
#[derive(Debug)]
pub enum SheetExcludeFlags {
    Base(RowExcludeFlags),
    // filteredOutRows: excludes filtered out rows from response payload if a
    // sheet filter is applied; includes total number of filtered rows
    FilteredOutRows,
}

impl EnumStr for SheetExcludeFlags {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::Base(flags) => flags.as_str(),
            Self::FilteredOutRows => "filteredOutRows",
        }
    }
}

impl fmt::Display for SheetExcludeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
