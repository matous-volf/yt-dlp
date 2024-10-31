use crate::fetcher::platform::{Architecture, Platform};
use thiserror::Error;

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// The possible errors that can occur.
#[derive(Debug, Error)]
pub enum Error {
    #[error("An error occurred while running the runtime: {0}")]
    Runtime(#[from] tokio::task::JoinError),
    #[error("An IO error occurred: {0}")]
    IO(#[from] std::io::Error),
    #[error("An error occurred while extracting the archive: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("An error occurred while fetching: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("An error occurred while parsing JSON: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("No GitHub asset found for platform {0}/{1}")]
    Github(Platform, Architecture),
    #[error("No FFmpeg binary found for platform {0}/{1}")]
    Binary(Platform, Architecture),
    #[error("Failed to execute command: {0}")]
    Command(String),
    #[error("Failed to fetch video: {0}")]
    Video(String),
    #[error("An invalid path was provided: {0}")]
    Path(String),

    #[error("An unknown error occurred: {0}")]
    Unknown(String),
}
