use std::fmt;

#[derive(Copy, Clone, Debug)]
/// The levels are documented here:
///   https://smartsheet-platform.github.io/api-docs/?python#working-with-complex-objects-multi-contact-or-multi-picklist
pub enum Level {
    /// LEVEL=0
    ///
    ///   Note: this is the default
    Text,
    /// LEVEL=1
    MultiContact,
    /// LEVEL=2
    MultiPicklist,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
