//! Builder constructs
//!
use core::option::Option;
use std::collections::HashMap;

/// **Param Builder** - Utility to help build query parameters in GET requests
pub struct ParamBuilder<'a> {
    params: HashMap<&'a str, String>,
}

impl<'a> ParamBuilder<'a> {
    /// Create a new `ParamBuilder` object
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Insert a list of `T` objects - which can be `enum` types that
    /// implement `std::fmt::Display` for example - as a comma-separated
    /// string value for a query parameter named `key`.
    pub fn insert_comma_separated_values<T: std::fmt::Display>(
        &mut self,
        key: &'a str,
        values: Option<Vec<T>>,
    ) {
        if let Some(values) = values {
            let mut string_val = values
                .iter()
                .fold(String::new(), |accum, e| accum + &e.to_string() + ",");
            string_val.pop();
            self.params.insert(key, string_val);

            // params.insert("include", values.iter().join(","));
        }
    }

    /// Insert a single`T` object which implements `std::fmt::Display` - such
    /// as a *string* - as a value for a query parameter named `key`.
    pub fn insert_value<T: std::fmt::Display>(&mut self, key: &'a str, value: Option<T>) {
        if let Some(value) = value {
            self.params.insert(key, value.to_string());
        }
    }

    /// Add the *query parameters* to a provided `url`, if needed.
    pub fn add_query_to_url(&self, url: &mut String) {
        if !self.params.is_empty() {
            let params_str: String = self
                .params
                .iter()
                .map(|(k, v)| format!("{}={}", *k, v))
                .collect::<Vec<String>>()
                .join("&");
            url.push('?');
            url.push_str(&params_str);
        }
    }
}

impl<'a> Default for ParamBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
