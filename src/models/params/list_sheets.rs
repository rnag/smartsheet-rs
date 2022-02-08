use crate::models::EnumStr;

use core::fmt;

/// List Sheet Include Flags are documented here:
///   https://smartsheet-platform.github.io/api-docs/#list-sheets
#[derive(Debug)]
pub enum ListSheetIncludeFlags {
    OwnerInfo,
    SheetVersion,
    Source,
}

impl EnumStr for ListSheetIncludeFlags {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::OwnerInfo => "ownerInfo",
            Self::SheetVersion => "sheetVersion",
            Self::Source => "source",
        }
    }
}

impl fmt::Display for ListSheetIncludeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
