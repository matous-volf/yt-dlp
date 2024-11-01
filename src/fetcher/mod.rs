//! Tools for fetching data from a URL.
//!
//! This module contains structs for fetching data from GitHub and ffmpeg, or over HTTP.
//! There is a platform module for detecting the current platform and architecture.
//! It also contains structs for representing the fetched data, such as GitHub releases and assets.

use crate::error::{Error, Result};
use crate::utils::file_system;
use derive_more::Display;
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::io::Write;
use std::path::PathBuf;

pub mod deps;
pub mod model;
pub mod platform;

/// The fetcher is responsible for fetching data from a URL.
/// # Examples
///
/// ```rust, no_run
/// # use yt_dlp::fetcher::Fetcher;
/// # use std::path::PathBuf;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let url = "https://example.com/file.txt";
/// let destination = PathBuf::from("file.txt");
///
/// let fetcher = Fetcher::new(url);
/// fetcher.fetch_asset(destination).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Display)]
#[display("Fetcher: {}", url)]
pub struct Fetcher {
    /// The URL to fetch data from.
    url: String,
}

impl Fetcher {
    /// Create a new fetcher for the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to fetch data from.
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    /// Fetch the data from the URL and return it as Serde value.
    ///
    /// # Arguments
    ///
    /// * `auth_token` - An optional authentication token to use for the request.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data could not be fetched or parsed.
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self)))]
    pub async fn fetch_json(&self, auth_token: Option<String>) -> Result<serde_json::Value> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Fetching JSON from {}", self.url);

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("rust-reqwest"));

        if let Some(auth_token) = auth_token {
            let value = HeaderValue::from_str(&format!("Bearer {}", auth_token))
                .map_err(|e| Error::Unknown(e.to_string()))?;

            headers.insert(reqwest::header::AUTHORIZATION, value);
        }

        let client = reqwest::Client::new();
        let response = client
            .get(&self.url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        let json = response.json().await?;
        Ok(json)
    }

    /// Downloads the asset at the given URL and writes it to the given destination.
    ///
    /// # Arguments
    ///
    /// * `destination` - The path to write the asset to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the asset could not be fetched or written to the destination.
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self)))]
    pub async fn fetch_asset(&self, destination: PathBuf) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Fetching asset from {} to {:?}", self.url, destination);

        let response = reqwest::get(&self.url).await?.error_for_status()?;

        if let Some(parent) = destination.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut dest = file_system::create_file(destination)?;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;

            dest.write_all(&chunk)?;
        }

        Ok(())
    }
}
