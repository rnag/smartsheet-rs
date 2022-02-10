//! Library-specific type definitions

/// A simple type alias so as to DRY.
pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
