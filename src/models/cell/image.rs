use serde::{Deserialize, Serialize};

/// Cell Image object
///
/// # Docs
/// <https://smartsheet.redoc.ly/#section/Image-Object>
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Image Id.
    pub id: String,
    /// Alternate text for the image.
    pub alt_text: String,
    /// Original height (in pixels) of the uploaded image.
    pub height: u64,
    /// Original width (in pixels) of the uploaded image.
    pub width: u64,
}
