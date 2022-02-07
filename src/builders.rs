///! Builder constructs
///!
use core::option::Option;
use std::collections::HashMap;

pub struct ParamBuilder<'a> {
    params: HashMap<&'a str, String>,
}

impl<'a> ParamBuilder<'a> {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

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

    pub fn insert_value<T: std::fmt::Display>(&mut self, key: &'a str, value: Option<T>) {
        if let Some(value) = value {
            self.params.insert(key, value.to_string());
        }
    }

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
