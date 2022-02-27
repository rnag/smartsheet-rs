use crate::models::EnumStr;
use core::fmt;

/// **Symbol Cell** - Represents a *Decision* dropdown cell.
///
pub enum Decision {
    Yes,
    Hold,
    No,
}

impl EnumStr for Decision {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::Yes => "Yes",
            Self::Hold => "Hold",
            Self::No => "No",
        }
    }
}

impl fmt::Display for Decision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
