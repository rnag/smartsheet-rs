use crate::models::{AttachmentMeta, User};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discussion {
    pub comment_count: u64,
    pub created_by: User,
    pub id: u64,
    pub last_commented_at: String,
    pub last_commented_user: User,
    pub title: String,
    pub comment_attachments: Option<Vec<AttachmentMeta>>,
}
