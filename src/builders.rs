//! Builder constructs
//!
use core::option::Option;
use std::collections::HashMap;

/// **Param Builder** - Utility to help build query parameters in GET requests
pub struct ParamBuilder<'a> {
    params: HashMap<&'a str, String>,
    url: &'a mut String,
}

impl<'a> ParamBuilder<'a> {
    /// Create a new `ParamBuilder` object
    pub fn new(url: &'a mut String) -> Self {
        Self {
            params: HashMap::new(),
            url,
        }
    }

    /// Insert a list of `T` objects - which can be `enum` types that
    /// implement `std::fmt::Display` for example - as a comma-separated
    /// string value for a query parameter named `key`.
    pub fn with_comma_separated_values<T: std::fmt::Display>(
        &mut self,
        key: &'a str,
        values: Option<Vec<T>>,
    ) -> &mut Self {
        if let Some(values) = values {
            let mut string_val = values
                .iter()
                .fold(String::new(), |accum, e| accum + &e.to_string() + ",");
            string_val.pop();
            self.params.insert(key, string_val);

            // params.insert("include", values.iter().join(","));
        }

        self
    }

    /// Insert a single`T` object which implements `std::fmt::Display` - such
    /// as a *string* - as a value for a query parameter named `key`.
    pub fn with_value<T: std::fmt::Display>(
        &mut self,
        key: &'a str,
        value: Option<T>,
    ) -> &mut Self {
        if let Some(value) = value {
            self.params.insert(key, value.to_string());
        }

        self
    }

    /// Add the *query parameters* to a provided `url`, if needed.
    pub fn build(&mut self) {
        if !self.params.is_empty() {
            let params_str: String = self
                .params
                .iter()
                .map(|(k, v)| format!("{}={}", *k, v))
                .collect::<Vec<String>>()
                .join("&");
            self.url.push('?');
            self.url.push_str(&params_str);
        }
    }
}
