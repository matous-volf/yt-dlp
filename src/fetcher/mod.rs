use crate::error::{Error, Result};
use crate::fetcher::model::{Asset, Release, WantedRelease};
use crate::fetcher::platform::{Architecture, Platform};
use crate::utils::file_system;
use derive_more::Display;
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::io::Write;
use std::path::PathBuf;

pub mod model;
pub mod platform;

/// The fetcher is responsible for fetching data from a URL.
/// # Examples
///
/// ```rust, no_run
/// # use yt_dlp::fetcher::Fetcher;
/// # use std::path::PathBuf;
///
/// # #[tokio::main]
/// # async fn main() {
///
/// let url = "https://example.com/file.txt";
/// let destination = PathBuf::from("file.txt");
///
/// let fetcher = Fetcher::new(url);
/// fetcher.fetch_asset(destination).await.expect("Failed to download asset");
/// # }
/// ```
#[derive(Debug, Display)]
#[display("Fetcher: {}", url)]
pub struct Fetcher {
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
    pub async fn fetch_json(&self, auth_token: Option<String>) -> Result<serde_json::Value> {
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
    pub async fn fetch_asset(&self, destination: PathBuf) -> Result<()> {
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

/// The GitHub fetcher is responsible for fetching the latest release of a GitHub repository.
/// It can also select the correct asset for the current platform and architecture.
///
/// # Example
///
/// ```rust, no_run
/// # use std::path::PathBuf;
/// # use yt_dlp::fetcher::GitHubFetcher;
///
/// # #[tokio::main]
/// # async fn main() {
///
/// let fetcher = GitHubFetcher::new("yt-dlp", "yt-dlp");
/// let release = fetcher.fetch_release(None).await?;
///
/// let destination = PathBuf::from("yt-dlp");
/// release.download(destination).await?;
/// # }
#[derive(Debug, Display)]
#[display("GitHub Fetcher: {}/{}", owner, repo)]
pub struct GitHubFetcher {
    owner: String,
    repo: String,
}

impl GitHubFetcher {
    /// Create a new fetcher for the given GitHub repository.
    ///
    /// # Arguments
    ///
    /// * `owner` - The owner of the GitHub repository.
    /// * `repo` - The name of the GitHub repository.
    pub fn new(owner: &str, repo: &str) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    /// Fetch the latest release of the GitHub repository, and select the correct asset for the current platform and architecture.
    ///
    /// # Arguments
    ///
    /// * `auth_token` - An optional GitHub personal access token to authenticate the request.
    pub async fn fetch_release(&self, auth_token: Option<String>) -> Result<WantedRelease> {
        let platform = Platform::detect();
        let architecture = Architecture::detect();

        self.fetch_release_for_platform(platform, architecture, auth_token)
            .await
    }

    /// Fetch the latest release of the GitHub repository, and select the correct asset for the given platform and architecture.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform to fetch the release for.
    /// * `architecture` - The architecture to fetch the release for.
    /// * `auth_token` - An optional GitHub personal access token to authenticate the request.
    pub async fn fetch_release_for_platform(
        &self,
        platform: Platform,
        architecture: Architecture,
        auth_token: Option<String>,
    ) -> Result<WantedRelease> {
        let release = self.fetch_latest_release(auth_token).await?;

        let asset = Self::select_asset(&platform, &architecture, &release)
            .ok_or(Error::Github(platform, architecture))?;

        Ok(WantedRelease {
            asset_name: asset.name.clone(),
            asset_url: asset.download_url.clone(),
        })
    }

    /// Fetch the latest release of the GitHub repository.
    ///
    /// # Arguments
    ///
    /// * `auth_token` - An optional GitHub personal access token to authenticate the request.
    pub async fn fetch_latest_release(&self, auth_token: Option<String>) -> Result<Release> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.owner, self.repo
        );

        let fetcher = Fetcher::new(&url);
        let response = fetcher.fetch_json(auth_token).await?;

        let release: Release = serde_json::from_value(response)?;
        Ok(release)
    }

    /// Select the correct asset from the release for the given platform and architecture.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform to select the asset for.
    /// * `architecture` - The architecture to select the asset for.
    /// * `release` - The release to select the asset from.
    pub fn select_asset<'a>(
        platform: &Platform,
        architecture: &Architecture,
        release: &'a Release,
    ) -> Option<&'a Asset> {
        const BASE_NAME: &str = "yt-dlp";

        let assets = &release.assets;
        let asset = assets.iter().find(|asset| {
            let name = &asset.name;

            match (platform, architecture) {
                (Platform::Windows, Architecture::X64) => {
                    name.contains(&format!("{}.exe", BASE_NAME))
                }
                (Platform::Windows, Architecture::X86) => {
                    name.contains(&format!("{}_x86.exe", BASE_NAME))
                }

                (Platform::Linux, Architecture::X64) => {
                    name.contains(&format!("{}_linux", BASE_NAME))
                }
                (Platform::Linux, Architecture::Armv7l) => {
                    name.contains(&format!("{}_linux_armv7l", BASE_NAME))
                }
                (Platform::Linux, Architecture::Aarch64) => {
                    name.contains(&format!("{}_linux_aarch64", BASE_NAME))
                }

                (Platform::Mac, _) => name.contains(&format!("{}_macos", BASE_NAME)),

                _ => false,
            }
        });

        asset
    }
}

/// The FFmpeg fetcher is responsible for fetching the FFmpeg binary for the current platform and architecture.
/// It can also extract the binary from the downloaded archive.
///
/// # Example
///
/// ```rust, no_run
/// # use yt_dlp::fetcher::FFmpeg;
/// # use std::path::PathBuf;
///
/// # #[tokio::main]
/// # async fn main() {
///
/// let path = PathBuf::from("ffmpeg-release.zip");
/// let fetcher = FFmpeg::new();
///
/// let release = fetcher.fetch_binary().await?;
/// release.download(path).await?;
///
/// fetcher.extract_binary(path).await?;
/// # }
/// ```
#[derive(Clone, Debug, Default, Display)]
pub struct FFmpeg;

impl FFmpeg {
    /// Create a new fetcher for FFmpeg.
    pub fn new() -> Self {
        Self
    }

    /// Fetch the FFmpeg binary for the current platform and architecture.
    pub async fn fetch_binary(&self) -> Result<WantedRelease> {
        let platform = Platform::detect();
        let architecture = Architecture::detect();

        self.fetch_binary_for_platform(platform, architecture).await
    }

    /// Fetch the FFmpeg binary for the given platform and architecture.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform to fetch the binary for.
    /// * `architecture` - The architecture to fetch the binary for.
    pub async fn fetch_binary_for_platform(
        &self,
        platform: Platform,
        architecture: Architecture,
    ) -> Result<WantedRelease> {
        let asset = self
            .select_asset(&platform, &architecture)
            .ok_or(Error::Binary(platform, architecture))?;

        Ok(WantedRelease {
            asset_name: asset.name.clone(),
            asset_url: asset.download_url.clone(),
        })
    }

    /// Select the correct FFmpeg asset for the given platform and architecture.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform to select the asset for.
    /// * `architecture` - The architecture to select the asset for.
    pub fn select_asset(&self, platform: &Platform, architecture: &Architecture) -> Option<Asset> {
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

    /// Extract the FFmpeg binary from the downloaded archive, for the current platform and architecture.
    /// The resulting binary will be placed in the same directory as the archive.
    /// The archive will be deleted after the binary has been extracted.
    pub async fn extract_binary(&self, archive: PathBuf) -> Result<()> {
        let platform = Platform::detect();
        let architecture = Architecture::detect();

        self.extract_binary_for_platform(archive, platform, architecture)
            .await
    }

    /// Extract the FFmpeg binary from the downloaded archive, for the given platform and architecture.
    /// The resulting binary will be placed in the same directory as the archive.
    /// The archive will be deleted after the binary has been extracted.
    ///
    /// # Arguments
    ///
    /// * `archive` - The path to the downloaded archive.
    /// * `platform` - The platform to extract the binary for.
    /// * `architecture` - The architecture to extract the binary for.
    pub async fn extract_binary_for_platform(
        &self,
        archive: PathBuf,
        platform: Platform,
        architecture: Architecture,
    ) -> Result<()> {
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

                tokio::fs::copy(executable, binary).await?;
                tokio::fs::remove_dir_all(destination).await?;
                tokio::fs::remove_file(archive).await?;
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

                file_system::set_executable(binary)?;
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

                file_system::set_executable(binary)?;
            }

            _ => return Err(Error::Binary(platform, architecture)),
        }

        Ok(())
    }
}
