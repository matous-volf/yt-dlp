//! Captions-related models.

use serde::{Deserialize, Serialize};

/// Represents an automatic caption of a YouTube video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomaticCaption {
    /// The extension of the caption file.
    #[serde(rename = "ext")]
    pub extension: Extension,
    /// The URL of the caption file.
    pub url: String,
    /// The language of the caption file, e.g. 'English'.
    pub name: Option<String>,
}

/// The available extensions for automatic caption files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Extension {
    /// The JSON extension.
    Json3,
    /// The Srv1 extension.
    Srv1,
    /// The Srv2 extension.
    Srv2,
    /// The Srv3 extension.
    Srv3,
    /// The Ttml extension.
    Ttml,
    /// The Vtt extension.
    Vtt,
}
