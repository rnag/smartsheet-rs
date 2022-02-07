use crate::models::CellValue::{Boolean, Numeric, Text};
use crate::types::Result;

use core::fmt::Error;
use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result::{Err, Ok};
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Represents the `value` attribute in the `Cell` struct definition.
///
/// Will be transmitted as one of the following:
///     * text -- a `string` value
///     * number -- a numeric value, generally an `f64` type
///     * boolean -- a `bool` value
///
/// # Description
///
/// Per the docs, this value represents one of either a string, a number, or
/// a Boolean value -- depending on the cell type and the data in the cell.
/// Cell values larger than 4000 characters are silently truncated. An empty
/// cell returns no value.
///
/// # Docs
/// - https://smartsheet-platform.github.io/api-docs/#cell-object
/// - https://smartsheet-platform.github.io/api-docs/#cell-reference
///
#[derive(Clone, PartialEq, Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum CellValue {
    Text(String),
    Boolean(bool),
    Numeric(Number),
}

impl CellValue {
    pub fn as_str(&self) -> Result<&str> {
        if let Text(value) = &self {
            Ok(value)
        } else {
            Err(Box::new(Error::default()))
        }
    }

    pub fn as_str_safe(&self) -> Option<&str> {
        if let Text(value) = &self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Result<&Number> {
        if let Numeric(value) = &self {
            Ok(value)
        } else {
            Err(Box::new(Error::default()))
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        if let Some(value) = self.as_number()?.as_u64() {
            Ok(value)
        } else {
            Err(Box::new(Error::default()))
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        if let Some(value) = self.as_number()?.as_f64() {
            Ok(value)
        } else {
            Err(Box::new(Error::default()))
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        if let Boolean(v) = &self {
            Ok(*v)
        } else {
            Err(Box::new(Error::default()))
        }
    }
}
