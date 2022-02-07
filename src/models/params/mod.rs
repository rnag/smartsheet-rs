mod get_row;
mod get_sheet;
mod level;
mod list_sheets;

pub use self::get_row::*;
pub use self::get_sheet::*;
pub use self::level::*;
pub use self::list_sheets::*;

/// Allows enums to serialize themselves to a string value.
pub(crate) trait EnumStr {
    fn as_str<'a>(&self) -> &'a str;
}
