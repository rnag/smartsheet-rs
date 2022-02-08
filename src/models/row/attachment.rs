use crate::models::User;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "attachmentType")]
    pub attachment_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    pub created_by: User,
    pub id: u64,
    pub name: String,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "sizeInKb")]
    pub size_in_kb: Option<u64>,
}
