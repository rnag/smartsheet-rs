use crate::models::{CellValue, Hyperlink, Image};
use crate::types::Result;

use core::fmt::Error;
use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result::{Err, Ok};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::value::Value;
use serde_json::Number;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    /// The Id of the column that the cell is located in.
    pub column_id: u64,
    /// Only returned if the include query string parameter contains
    /// `columnType`.
    // #[serde(skip_serializing)]
    pub column_type: Option<String>,
    /// The format descriptor describing this cell's conditional format. Only
    /// returned if the include query string parameter contains `format` and
    /// this cell has a conditional format applied.
    pub conditional_format: Option<String>,
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
    // #[serde(skip_serializing)]
    pub display_value: Option<String>,
    /// `Cell.objectValue` is an object representation of a cell's value and
    /// is currently used for adding or updating predecessor cell values, or
    /// for multi-contact details, such as email addresses.
    ///
    /// # More info
    ///
    /// - https://smartsheet-platform.github.io/api-docs/#cell-reference
    // #[serde(skip_serializing)]
    pub object_value: Option<Value>,
    /// The format descriptor. Only returned if the include query string
    /// parameter contains `format` and this cell has a non-default format
    /// applied.
    pub format: Option<String>,
    /// The formula for a cell, if set, for instance **=COUNTM([Assigned To]3)**.
    ///
    /// Note that calculation errors or problems with a formula do not cause
    /// the API call to return an error code. Instead, the response contains
    /// the same value as in the UI, such as
    /// `cell.value = "#CIRCULAR REFERENCE"`.
    pub formula: Option<String>,
    /// Cell Image object
    pub image: Option<Image>,
    /// (Admin only) Indicates whether the cell value can contain a value
    /// outside of the validation limits (value = **true**). When using this
    /// parameter, you must also set `strict` to **false** to bypass value
    /// type checking. This property is honored for POST or PUT actions that
    /// update rows.
    #[serde(skip_deserializing)]
    pub override_validation: Option<bool>,
    /// Set to false to enable lenient parsing. Defaults to true. You can
    /// specify this attribute in a request, but it is never present in a
    /// response.
    #[serde(skip_deserializing)]
    pub strict: Option<bool>,
    // TODO add below fields (low priority)
    // pub link_in_from_cell: Object,
    // pub links_out_to_cells: Object,
}

impl Serialize for Cell {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut cell = serializer.serialize_struct("Cell", 3)?;
        cell.serialize_field("columnId", &self.column_id)?;

        if let Some(link) = &self.hyperlink {
            cell.serialize_field("hyperlink", link)?;
        }

        if let Some(value) = &self.value {
            cell.serialize_field("value", value)?;
        }

        if let Some(object_value) = &self.object_value {
            cell.serialize_field("objectValue", object_value)?;
        }

        cell.end()
    }
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::CellValue::Text;
    use indoc::indoc;
    use serde_json::{json, to_string_pretty};

    #[test]
    fn test_it() {
        let c = Cell {
            column_id: 0,
            column_type: None,
            conditional_format: None,
            hyperlink: None,
            value: None,
            display_value: None,
            object_value: None,
            format: None,
            formula: None,
            image: None,
            override_validation: None,
            strict: None,
        };
        println!("{}", to_string_pretty(&c).unwrap());

        assert_eq!(
            to_string_pretty(&c).unwrap(),
            indoc! {r#"
                {
                  "columnId": 0
                }
            "#}
            .trim()
        );
    }

    #[test]
    fn test_another() {
        let c = Cell {
            column_id: 54321,
            column_type: Some("Testing".to_owned()),
            conditional_format: None,
            hyperlink: Some(Hyperlink {
                url: "abc".to_owned(),
                ..Default::default()
            }),
            value: Some(Text("My value".to_owned())),
            display_value: Some("Something".to_owned()),
            object_value: Some(json!(1.2)),
            format: Some("My format".to_owned()),
            formula: Some("My formula".to_owned()),
            image: None,
            override_validation: None,
            strict: None,
        };
        assert_eq!(
            to_string_pretty(&c).unwrap(),
            indoc! {r#"
                {
                  "columnId": 54321,
                  "hyperlink": {
                    "url": "abc"
                  },
                  "value": "My value",
                  "objectValue": 1.2
                }
            "#}
            .trim()
        );
    }
}
