use crate::models::EnumStr;
use core::fmt;

/// Represents a value for the `objectType` field within an
/// [ObjectValue Object].
///
/// [ObjectValue Object]: https://smartsheet.redoc.ly/#section/ObjectValue-Object
///
pub enum ObjectType {
    AbstractDateTime,
    Contact,
    Date,
    DateTime,
    Duration,
    MultiContact,
    MultiPicklist,
    PredecessorList,
}

impl EnumStr for ObjectType {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::AbstractDateTime => "ABSTRACT_DATETIME",
            Self::Contact => "CONTACT",
            Self::Date => "DATE",
            Self::DateTime => "DATETIME",
            Self::Duration => "DURATION",
            Self::MultiContact => "MULTI_CONTACT",
            Self::MultiPicklist => "MULTI_PICKLIST",
            Self::PredecessorList => "PREDECESSOR_LIST",
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
