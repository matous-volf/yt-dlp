//! Fetchable-data models.

use crate::error::Result;
use crate::fetcher::Fetcher;
use derive_more::Display;
use serde::Deserialize;
use std::path::PathBuf;

/// A GitHub release.
#[derive(Debug, Deserialize, Display)]
#[display("Release: tag={}, assets={};", tag_name, assets.len())]
pub struct Release {
    /// The tag name of the release.
    pub tag_name: String,
    /// The assets of the release.
    pub assets: Vec<Asset>,
}

/// A release asset.
#[derive(Debug, Deserialize, Display)]
#[display("Asset: name={}, url={};", name, download_url)]
pub struct Asset {
    /// The name of the asset.
    pub name: String,
    /// The download URL of the asset.
    #[serde(rename = "browser_download_url")]
    pub download_url: String,
}

/// A wanted release, for the current platform and architecture.
#[derive(Debug, Display)]
#[display("WantedRelease: asset={}, url={};", asset_name, asset_url)]
pub struct WantedRelease {
    /// The name of the asset.
    pub asset_name: String,
    /// The URL of the asset.
    pub asset_url: String,
}

impl WantedRelease {
    /// Download the release asset to the given destination.
    ///
    /// # Arguments
    ///
    /// * `destination` - The path to write the asset to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the asset could not be downloaded or written to the destination.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::fetcher::model::WantedRelease;
    /// # use std::path::PathBuf;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let release = WantedRelease {
    ///     asset_name: "yt-dlp".to_string(),
    ///     asset_url: "https://github.com/yt-dlp/yt-dlp/releases/download/2024.10.22/yt-dlp".to_string(),
    /// };
    ///
    /// let destination = PathBuf::from("yt-dlp");
    /// release.download(destination).await?;
    /// # Ok(())
    /// # }
    pub async fn download(&self, destination: PathBuf) -> Result<()> {
        let fetcher = Fetcher::new(&self.asset_url);
        fetcher.fetch_asset(destination).await
    }
}
