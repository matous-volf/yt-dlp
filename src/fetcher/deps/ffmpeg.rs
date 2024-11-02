//! Fetch the latest release of 'ffmpeg' from static builds.

use crate::error::{Error, Result};
use crate::fetcher::model::{Asset, WantedRelease};
use crate::fetcher::platform::{Architecture, Platform};
use crate::utils::file_system;
use derive_more::Display;
use std::path::PathBuf;

/// The ffmpeg fetcher is responsible for fetching the ffmpeg binary for the current platform and architecture.
/// It can also extract the binary from the downloaded archive.
///
/// # Example
///
/// ```rust, no_run
/// # use yt_dlp::fetcher::deps::ffmpeg::BuildFetcher;
/// # use std::path::PathBuf;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let path = PathBuf::from("ffmpeg-release.zip");
/// let fetcher = BuildFetcher::new();
///
/// let release = fetcher.fetch_binary().await?;
/// release.download(path.clone()).await?;
///
/// fetcher.extract_binary(path).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default, Display)]
pub struct BuildFetcher;

impl BuildFetcher {
    /// Create a new fetcher for ffmpeg.
    pub fn new() -> Self {
        Self
    }

    /// Fetch the ffmpeg binary for the current platform and architecture.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", skip(self)))]
    pub async fn fetch_binary(&self) -> Result<WantedRelease> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Fetching ffmpeg binary");

        let platform = Platform::detect();
        let architecture = Architecture::detect();

        self.fetch_binary_for_platform(platform, architecture).await
    }

    /// Fetch the ffmpeg binary for the given platform and architecture.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform to fetch the binary for.
    /// * `architecture` - The architecture to fetch the binary for.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", skip(self)))]
    pub async fn fetch_binary_for_platform(
        &self,
        platform: Platform,
        architecture: Architecture,
    ) -> Result<WantedRelease> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Fetching ffmpeg binary for platform: {:?}, architecture: {:?}",
            platform,
            architecture
        );

        let asset = self
            .select_asset(&platform, &architecture)
            .ok_or(Error::Binary(platform, architecture))?;

        Ok(WantedRelease {
            asset_name: asset.name.clone(),
            asset_url: asset.download_url.clone(),
        })
    }

    /// Select the correct ffmpeg asset for the given platform and architecture.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform to select the asset for.
    /// * `architecture` - The architecture to select the asset for.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", skip(self)))]
    pub fn select_asset(&self, platform: &Platform, architecture: &Architecture) -> Option<Asset> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Selecting ffmpeg asset for platform: {:?}, architecture: {:?}",
            platform,
            architecture
        );

        let url = match (platform, architecture) {
            (Platform::Windows, _) => {
                "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
            }

            (Platform::Mac, Architecture::X64) => "https://www.osxexperts.net/ffmpeg71intel.zip",
            (Platform::Mac, Architecture::Aarch64) => "https://www.osxexperts.net/ffmpeg71arm.zip",

            (Platform::Linux, Architecture::X64) => {
                "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz"
            }
            (Platform::Linux, Architecture::X86) => {
                "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-i686-static.tar.xz"
            }
            (Platform::Linux, Architecture::Armv7l) => {
                "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-armhf-static.tar.xz"
            }
            (Platform::Linux, Architecture::Aarch64) => {
                "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-arm64-static.tar.xz"
            }

            _ => return None,
        };

        let name = url.split('/').last()?;
        let asset = Asset {
            name: name.to_string(),
            download_url: url.to_string(),
        };

        Some(asset)
    }

    /// Extract the ffmpeg binary from the downloaded archive, for the current platform and architecture.
    /// The resulting binary will be placed in the same directory as the archive.
    /// The archive will be deleted after the binary has been extracted.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", skip(self)))]
    pub async fn extract_binary(&self, archive: PathBuf) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Extracting ffmpeg binary from archive: {:?}", archive);

        let platform = Platform::detect();
        let architecture = Architecture::detect();

        self.extract_binary_for_platform(archive, platform, architecture)
            .await
    }

    /// Extract the ffmpeg binary from the downloaded archive, for the given platform and architecture.
    /// The resulting binary will be placed in the same directory as the archive.
    /// The archive will be deleted after the binary has been extracted.
    ///
    /// # Arguments
    ///
    /// * `archive` - The path to the downloaded archive.
    /// * `platform` - The platform to extract the binary for.
    /// * `architecture` - The architecture to extract the binary for.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", skip(self)))]
    pub async fn extract_binary_for_platform(
        &self,
        archive: PathBuf,
        platform: Platform,
        architecture: Architecture,
    ) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Extracting ffmpeg binary for platform: {:?}, architecture: {:?}, from archive: {:?}",
            platform,
            architecture,
            archive
        );

        let destination = archive.with_extension("");

        let archive_clone = archive.clone();
        let destination_clone = destination.clone();

        match platform {
            Platform::Windows => {
                tokio::task::spawn_blocking(move || {
                    file_system::extract_zip(archive_clone, destination_clone)
                })
                .await??;

                let extracted = destination.join("ffmpeg-7.1-essentials_build");
                let executable = extracted.join("bin").join("ffmpeg.exe");

                let parent = destination
                    .parent()
                    .ok_or(Error::Binary(platform, architecture))?;
                let binary = parent.join("ffmpeg.exe");

                tokio::fs::copy(executable, binary.clone()).await?;
                tokio::fs::remove_dir_all(destination).await?;
                tokio::fs::remove_file(archive).await?;

                Ok(binary)
            }
            Platform::Mac => {
                tokio::task::spawn_blocking(move || {
                    file_system::extract_zip(archive_clone, destination_clone)
                })
                .await??;

                let executable = destination.join("ffmpeg");

                let parent = destination
                    .parent()
                    .ok_or(Error::Binary(platform, architecture))?;
                let binary = parent.join("ffmpeg");

                tokio::fs::copy(executable, binary.clone()).await?;
                tokio::fs::remove_dir_all(destination).await?;
                tokio::fs::remove_file(archive).await?;

                file_system::set_executable(binary.clone())?;
                Ok(binary)
            }
            Platform::Linux => {
                tokio::task::spawn_blocking(move || {
                    file_system::extract_tar_xz(archive_clone, destination_clone)
                })
                .await??;

                let extracted = match architecture {
                    Architecture::X64 => "ffmpeg-7.0.2-amd64-static",
                    Architecture::X86 => "ffmpeg-7.0.2-i686-static",
                    Architecture::Armv7l => "ffmpeg-7.0.2-armhf-static",
                    Architecture::Aarch64 => "ffmpeg-7.0.2-arm64-static",
                    _ => return Err(Error::Binary(platform, architecture)),
                };
                let extracted = destination.join(extracted);
                let executable = extracted.join("ffmpeg");

                let parent = destination
                    .parent()
                    .ok_or(Error::Binary(platform, architecture))?;
                let binary = parent.join("ffmpeg");

                tokio::fs::copy(executable, binary.clone()).await?;
                tokio::fs::remove_dir_all(destination).await?;

                file_system::set_executable(binary.clone())?;
                Ok(binary)
            }

            _ => Err(Error::Binary(platform, architecture)),
        }
    }
}
