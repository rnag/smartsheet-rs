use crate::models::User;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub attachment_type: String,
    pub created_at: String,
    pub created_by: User,
    pub id: u64,
    pub name: String,
    pub mime_type: Option<String>,
    pub size_in_kb: Option<u64>,
}
