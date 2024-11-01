//! Thumbnails-related models.

use serde::{Deserialize, Serialize};

/// Represents a thumbnail of a YouTube video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thumbnail {
    /// The URL of the thumbnail.
    pub url: String,
    /// The preference index of the thumbnail, e.g. '-35' or '0'.
    pub preference: i64,

    /// The ID of the thumbnail.
    pub id: String,
    /// The height of the thumbnail, can be `None`.
    pub height: Option<i64>,
    /// The width of the thumbnail, can be `None`.
    pub width: Option<i64>,
    /// The resolution of the thumbnail, can be `None`, e.g. '1920x1080'.
    pub resolution: Option<String>,
}
