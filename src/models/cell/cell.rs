use crate::models::{CellValue, Hyperlink};
use crate::types::Result;

use core::fmt::Error;
use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result::{Err, Ok};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use serde_json::Number;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    #[serde(rename = "columnId")]
    pub column_id: u64,
    #[serde(rename = "columnType")]
    pub column_type: Option<String>,
    /// Represents a hyperlink to a dashboard, report, sheet, or URL.
    ///
    /// In the most common scenario, the hyperlink is a URL link, and the `url`
    /// property contains the URL value. This can more easily be retrieved
    /// via the `Cell.link_url` method.
    ///
    /// # More info
    ///
    /// - https://smartsheet-platform.github.io/api-docs/#hyperlinks
    /// - https://smartsheet-platform.github.io/api-docs/#hyperlink-object
    pub hyperlink: Option<Hyperlink>,
    /// `Cell.value` represents a cell's raw value and can be one of the
    /// following primitive JSON types: string, number, or Boolean, depending
    /// on the column type. An empty cell returns no value.
    ///
    /// # More info
    ///
    /// - https://smartsheet-platform.github.io/api-docs/#cell-reference
    pub value: Option<CellValue>,
    /// `Cell.displayValue` is always a string and is only returned for certain
    /// column types. It represents the formatted value as it should be
    /// displayed to an end-user.
    ///
    /// # Examples
    ///
    /// If a TEXT_NUMBER column is formatted as a US Dollar currency, its
    /// value may be a number like 1234.5678, but its displayValue
    /// is "$1,234.57".
    ///
    /// # More info
    ///
    /// - https://smartsheet-platform.github.io/api-docs/#cell-reference
    #[serde(rename = "displayValue")]
    pub display_value: Option<String>,
    /// `Cell.objectValue` is an object representation of a cell's value and
    /// is currently used for adding or updating predecessor cell values, or
    /// for multi-contact details, such as email addresses.
    ///
    /// # More info
    ///
    /// - https://smartsheet-platform.github.io/api-docs/#cell-reference
    #[serde(rename = "objectValue")]
    pub object_value: Option<Value>,
    pub format: Option<String>,
    pub formula: Option<String>,
}

impl Cell {
    /// Retrieve the Cell `value` as a *string*
    pub fn value_as_str(&self) -> Result<&str> {
        if let Some(value) = &self.value {
            value.as_str()
        } else {
            Err(Box::new(Error::default()))
        }
    }

    /// Retrieve the Cell `value` as a *string*, but prefer to use an
    /// `Option` implementation instead, which can be `unwrap`ped.
    pub fn value_as_str_safe(&self) -> Option<&str> {
        if let Some(value) = &self.value {
            value.as_str_safe()
        } else {
            None
        }
    }

    /// Retrieve the Cell `value` as a *Number*
    pub fn value_as_number(&self) -> Result<&Number> {
        if let Some(value) = &self.value {
            value.as_number()
        } else {
            Err(Box::new(Error::default()))
        }
    }

    /// Retrieve the Cell `value` as a *boolean*
    pub fn value_as_bool(&self) -> Result<bool> {
        if let Some(value) = &self.value {
            value.as_bool()
        } else {
            Err(Box::new(Error::default()))
        }
    }

    /// Retrieve the Cell `value` as an *unsigned integer*
    pub fn value_as_u64(&self) -> Result<u64> {
        if let Some(value) = &self.value {
            value.as_u64()
        } else {
            Err(Box::new(Error::default()))
        }
    }

    /// Retrieve the Cell `value` as a *float*
    pub fn value_as_f64(&self) -> Result<f64> {
        if let Some(value) = &self.value {
            value.as_f64()
        } else {
            Err(Box::new(Error::default()))
        }
    }

    /// Retrieve the Cell `display_value` as a *string*
    pub fn display_value_as_str(&self) -> Result<&str> {
        if let Some(value) = &self.display_value {
            Ok(value)
        } else {
            Err(Box::new(Error::default()))
        }
    }

    /// Retrieve the Cell `display_value` as a *string*, but prefer to use an
    /// `Option` implementation instead, which can be `unwrap`ped.
    pub fn display_value_as_str_safe(&self) -> Option<&str> {
        match &self.display_value {
            Some(v) => Some(v),
            None => None,
        }
    }

    /// Retrieve the Cell `hyperlink` URL as a *string*
    pub fn link_url(&self) -> Result<&str> {
        if let Some(link) = &self.hyperlink {
            Ok(&link.url)
        } else {
            Err(Box::new(Error::default()))
        }
    }

    /// Retrieve the Cell `hyperlink` URL as a *string*, but prefer to use an
    /// `Option` implementation instead, which can be `unwrap`ped.
    pub fn link_url_safe(&self) -> Option<&str> {
        if let Some(link) = &self.hyperlink {
            Some(&link.url)
        } else {
            None
        }
    }
}
