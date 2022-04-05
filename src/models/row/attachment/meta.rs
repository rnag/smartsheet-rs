use super::*;
use crate::models::User;

use serde::{Deserialize, Serialize};

/// Represents meta details on an [Attachment] in Smartsheet.
///
/// Attachments can exist on a comment (that is, within a discussion), on a
/// row, or on a sheet.
///
/// [Attachment]: https://smartsheet-platform.github.io/api-docs/#objects
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentMeta {
    /// Attachment Id
    pub id: u64,
    /// Attachment name
    pub name: String,
    /// Attachment type (one of `AttachmentType`)
    ///
    /// # Note
    /// `Smartsheetgov.com` accounts are restricted to the following
    /// attachment types: BOX_COM, FILE, GOOGLE_DRIVE, LINK, or ONEDRIVE.
    pub attachment_type: AttachmentType,
    /// A timestamp of when the attachment was originally added
    pub created_at: String,
    /// User object containing name and email of the creator of this attachment
    pub created_by: User,
    /// The Id of the parent
    ///
    /// # Note
    /// This field does not appear to be populated when `Get Sheet` is called
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,
    /// The type of object the attachment belongs to (one of COMMENT, ROW, or SHEET)
    ///
    /// # Note
    /// This field does not appear to be populated when `Get Sheet` is called
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<ParentType>,
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
    /// The size of the file, if the attachmentType is `FILE`
    ///
    /// # Note
    /// This field only seems to be populated for `FILE` type attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_kb: Option<u64>,
}
