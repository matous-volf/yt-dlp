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
    pub name: String,
}

/// The available extensions for automatic caption files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Extension {
    Json3,
    Srv1,
    Srv2,
    Srv3,
    Ttml,
    Vtt,
}
