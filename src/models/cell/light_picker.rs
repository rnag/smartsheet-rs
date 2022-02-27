use crate::models::EnumStr;

use core::fmt;

/// **Symbol Cell** - Represents a *Light Picker* dropdown cell.
///
pub enum LightPicker {
    Red,
    Yellow,
    Green,
    Blue,
    Gray,
}

impl EnumStr for LightPicker {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::Red => "Red",
            Self::Yellow => "Yellow",
            Self::Green => "Green",
            Self::Blue => "Blue",
            Self::Gray => "Gray",
        }
    }
}

impl fmt::Display for LightPicker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
