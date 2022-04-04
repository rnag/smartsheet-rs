use super::*;
use crate::utils::is_default;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// Additional info on the attachment, such as attachment name and type
    #[serde(flatten)]
    pub meta: AttachmentMeta,
    /// Attachment temporary URL (files only)
    ///
    /// # Note
    /// For other attachment types (such as LINK) this will instead be a
    /// permanent URL to the image.
    pub url: String,
    /// Attachment temporary URL time to live (files only)
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub url_expires_in_millis: u64,
}

impl Attachment {
    /// Returns the download URL for the Attachment.
    ///
    /// # Note
    /// For `FILE` type attachments, this will be a temporary S3 pre-signed URL
    /// that expires in `url_expires_in_millis`. For other attachment types
    /// such as `LINK`, this will instead be a permanent download URL.
    pub fn download_url(&self) -> &str {
        &self.url
    }
}
