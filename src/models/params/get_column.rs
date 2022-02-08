use crate::models::EnumStr;

use std::fmt;

#[derive(Debug)]
/// Column Include Flags are documented here:
///   http://smartsheet-platform.github.io/smartsheet-csharp-sdk/html/a3a70465-3261-c76a-9b20-5eae99c0e4be.htm
pub enum ColumnIncludeFlags {
    Filters,
}

impl EnumStr for ColumnIncludeFlags {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::Filters => "filters",
        }
    }
}

impl fmt::Display for ColumnIncludeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
