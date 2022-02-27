use serde::{Deserialize, Serialize, Serializer};

/// Determines whether `indent` / `outdent` is enabled. When serializing, the
/// value will be set as a numeric "1". No other value is required to be set.
/// See [Specify Row Location] for more details.
///
/// [Specify Row Location]: https://smartsheet.redoc.ly/#section/Specify-Row-Location
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum IndentEnabled {
    TRUE = 1,
}

impl Serialize for IndentEnabled {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            IndentEnabled::TRUE => serializer.serialize_u8(1),
        }
    }
}
