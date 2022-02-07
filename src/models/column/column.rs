use crate::models::ContactOption;
use core::option::Option;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub id: u64,
    pub index: u64,
    pub locked: Option<bool>,
    #[serde(rename = "lockedForUser")]
    pub locked_for_user: Option<bool>,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub validation: bool,
    pub version: u64,
    pub width: u64,
    pub description: Option<String>,
    pub options: Option<Vec<String>>,
    pub hidden: Option<bool>,
    pub symbol: Option<String>,
    pub tags: Option<Vec<String>>,
    pub primary: Option<bool>,
    pub format: Option<String>,
    pub formula: Option<String>,
    #[serde(rename = "contactOptions")]
    pub contact_options: Option<Vec<ContactOption>>,
}
