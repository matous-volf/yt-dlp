//! The fetchers for required dependencies.

use crate::error::Result;
use crate::fetcher::deps::ffmpeg::BuildFetcher;
use crate::fetcher::deps::youtube::GitHubFetcher;
use crate::utils::file_system;
use crate::{ternary, utils};
use derive_more::Constructor;
use std::path::PathBuf;

pub mod ffmpeg;
pub mod youtube;

/// Installs required libraries.
///
/// # Examples
///
/// ```rust,no_run
/// # use yt_dlp::fetcher::deps::LibraryInstaller;
/// # use std::path::PathBuf;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let destination = PathBuf::from("libs");
/// let installer = LibraryInstaller::new(destination);
///
/// let youtube = installer.install_youtube(None).await.unwrap();
/// let ffmpeg = installer.install_ffmpeg(None).await.unwrap();
/// # Ok(())
/// # }
/// ```
#[derive(Constructor, Clone, Debug)]
pub struct LibraryInstaller {
    /// The destination directory for the libraries.
    pub destination: PathBuf,
}

/// The installed libraries.
///
/// # Examples
///
/// ```rust,no_run
/// # use yt_dlp::fetcher::deps::Libraries;
/// # use std::path::PathBuf;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let destination = PathBuf::from("libs");
///
/// let youtube = destination.join("yt-dlp");
/// let ffmpeg = destination.join("ffmpeg");
///
/// let libraries = Libraries::new(youtube, ffmpeg);
/// # Ok(())
/// # }
/// ```
#[derive(Constructor, Clone, Debug)]
pub struct Libraries {
    /// The path to the installed yt-dlp binary.
    pub youtube: PathBuf,
    /// The path to the installed ffmpeg binary.
    pub ffmpeg: PathBuf,
}

impl LibraryInstaller {
    /// Install yt-dlp from the main repository.
    pub async fn install_youtube(&self, custom_name: Option<String>) -> Result<PathBuf> {
        self.install_youtube_from_repo("yt-dlp", "yt-dlp", None, custom_name)
            .await
    }

    /// Install yt-dlp from a custom repository, assuming releases assets are named correctly.
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self)))]
    pub async fn install_youtube_from_repo(
        &self,
        owner: &str,
        repo: &str,
        auth_token: Option<String>,
        custom_name: Option<String>,
    ) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Installing yt-dlp from {}/{}, with custom executable name: {:?}",
            owner,
            repo,
            custom_name
        );

        file_system::create_dir(self.destination.clone())?;

        let fetcher = GitHubFetcher::new(owner, repo);

        let name = custom_name.unwrap_or(String::from("yt-dlp"));
        let path = self.destination.join(utils::find_executable(&name));

        let release = fetcher.fetch_release(auth_token).await?;
        release.download(path.clone()).await?;

        Ok(path)
    }

    /// Install ffmpeg from static builds.
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self)))]
    pub async fn install_ffmpeg(&self, custom_name: Option<String>) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Installing ffmpeg with custom executable name: {:?}",
            custom_name
        );

        file_system::create_dir(self.destination.clone())?;

        let fetcher = BuildFetcher::new();
        let archive = self.destination.join("ffmpeg-release.zip");

        let release = fetcher.fetch_binary().await?;
        release.download(archive.clone()).await?;
        let path = fetcher.extract_binary(archive).await?;

        if let Some(name) = custom_name {
            let new_path = self.destination.join(utils::find_executable(&name));
            std::fs::rename(&path, &new_path)?;

            return Ok(new_path);
        }

        Ok(path)
    }
}

impl Libraries {
    /// Install the required dependencies.
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self)))]
    pub async fn install_dependencies(&self) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Installing required dependencies");

        let youtube = self.install_youtube().await?;
        let ffmpeg = self.install_ffmpeg().await?;

        Ok(Self::new(youtube, ffmpeg))
    }

    /// Install yt-dlp.
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self)))]
    pub async fn install_youtube(&self) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Installing yt-dlp");

        let parent = file_system::try_parent(self.youtube.clone())?;
        let installer = LibraryInstaller::new(parent);

        if self.youtube.exists() {
            return Ok(self.youtube.clone());
        }

        let name = utils::find_executable("yt-dlp");
        let file_name = file_system::try_name(self.youtube.clone())?;

        let custom_name = ternary!(file_name == name, None, Some(file_name));
        installer.install_youtube(custom_name).await
    }

    /// Install ffmpeg.
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self)))]
    pub async fn install_ffmpeg(&self) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Installing ffmpeg");

        let parent = file_system::try_parent(self.ffmpeg.clone())?;
        let installer = LibraryInstaller::new(parent);

        if self.ffmpeg.exists() {
            return Ok(self.ffmpeg.clone());
        }

        let name = utils::find_executable("ffmpeg");
        let file_name = file_system::try_name(self.ffmpeg.clone())?;

        let custom_name = ternary!(file_name == name, None, Some(file_name));
        installer.install_ffmpeg(custom_name).await
    }
}
