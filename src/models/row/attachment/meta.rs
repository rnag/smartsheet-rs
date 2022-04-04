use super::*;
use crate::models::User;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentMeta {
    /// Attachment Id
    pub id: u64,
    /// The Id of the parent
    pub parent_id: u64,
    /// Attachment type (one of `AttachmentType`)
    ///
    /// # Note
    /// `Smartsheetgov.com` accounts are restricted to the following
    /// attachment types: BOX_COM, FILE, GOOGLE_DRIVE, LINK, or ONEDRIVE.
    pub attachment_type: AttachmentType,
    /// Attachment sub type, valid only for the following attachment
    /// types: EGNYTE, GOOGLE_DRIVE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_sub_type: Option<AttachmentSubType>,
    /// Attachment MIME type (PNG, etc.)
    ///
    /// # Note
    /// This field only seems to be populated for `FILE` type attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// The type of object the attachment belongs to (one of COMMENT, ROW, or SHEET)
    pub parent_type: ParentType,
    /// A timestamp of when the attachment was originally added
    pub created_at: String,
    /// User object containing name and email of the creator of this attachment
    pub created_by: User,
    /// Attachment name
    pub name: String,
    /// The size of the file, if the attachmentType is `FILE`
    ///
    /// # Note
    /// This field only seems to be populated for `FILE` type attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_kb: Option<u64>,
}
