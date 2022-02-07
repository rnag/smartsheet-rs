use crate::models::{Attachment, User};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discussion {
    #[serde(rename = "commentCount")]
    pub comment_count: u64,
    #[serde(rename = "createdBy")]
    pub created_by: User,
    pub id: u64,
    #[serde(rename = "lastCommentedAt")]
    pub last_commented_at: String,
    #[serde(rename = "lastCommentedUser")]
    pub last_commented_user: User,
    pub title: String,
    #[serde(rename = "commentAttachments")]
    pub comment_attachments: Option<Vec<Attachment>>,
}
