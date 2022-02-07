use core::option::Option;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactOption {
    pub email: Option<String>,
    pub name: String,
}
