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
    id: String,
    /// Alternate text for the image.
    alt_text: String,
    /// Original height (in pixels) of the uploaded image.
    height: u64,
    /// Original width (in pixels) of the uploaded image.
    width: u64,
}
