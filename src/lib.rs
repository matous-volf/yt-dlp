//! A YouTube video fetcher that uses yt-dlp to fetch video information and download it.
//!
//! # Examples
//!
//! - 📦 Installing the [yt-dlp](https://github.com/yt-dlp/yt-dlp/) and [ffmpeg](https://ffmpeg.org/) binaries:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let destination = PathBuf::from("bin");
//! let output_dir = PathBuf::from("output");
//! let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
//!
//! let fetcher = Youtube::new_with_binaries(destination, url, output_dir).await.expect("Failed to install binaries");
//! # }
//! ```
//!
//! - 📦 Installing the yt-dlp binary only:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let destination = PathBuf::from("bin");
//!
//! Youtube::install_youtube(destination).await.expect("Failed to install yt-dlp");
//! # }
//! ```
//!
//! - 📦 Installing the ffmpeg binary only:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let destination = PathBuf::from("bin");
//!
//! Youtube::install_ffmpeg(destination).await.expect("Failed to install ffmpeg");
//! # }
//! ```
//!
//! - 🔄 Updating the yt-dlp binary:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let ffmpeg = PathBuf::from("ffmpeg");
//! let executable = PathBuf::from("yt-dlp");
//! let output_dir = PathBuf::from("output");
//! let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
//!
//! let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir).expect("Failed to create fetcher");
//! fetcher.update_downloader().await.expect("Failed to update yt-dlp");
//! # }
//! ```
//!
//! - 📥 Fetching a video (with its audio) and downloading it:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let ffmpeg = PathBuf::from("ffmpeg");
//! let executable = PathBuf::from("yt-dlp");
//! let output_dir = PathBuf::from("output");
//! let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
//!
//! let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir).expect("Failed to create fetcher");
//!
//! let video = fetcher.fetch_infos().await.expect("Failed to fetch video");
//! println!("Video title: {}", video.title);
//!
//! fetcher.download_video("video.mp4").await.expect("Failed to download video");
//! # }
//! ```
//!
//! - 🎬 Fetching a video (without its audio) and downloading it:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let ffmpeg = PathBuf::from("ffmpeg");
//! let executable = PathBuf::from("yt-dlp");
//! let output_dir = PathBuf::from("output");
//! let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
//!
//! let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir).expect("Failed to create fetcher");
//!
//! let video = fetcher.fetch_infos().await.expect("Failed to fetch video");
//! println!("Video title: {}", video.title);
//!
//! fetcher.download_video_stream("video.mp4").await.expect("Failed to download video without audio");
//! # }
//! ```
//!
//! - 🎵 Fetching an audio and downloading it:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let ffmpeg = PathBuf::from("ffmpeg");
//! let executable = PathBuf::from("yt-dlp");
//! let output_dir = PathBuf::from("output");
//! let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
//!
//! let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir).expect("Failed to create fetcher");
//!
//! let video = fetcher.fetch_infos().await.expect("Failed to fetch video");
//! println!("Video title: {}", video.title);
//!
//! fetcher.download_audio_stream("audio.mp3").await.expect("Failed to download audio");
//! # }
//! ```
//!
//! - 📜 Fetching a specific format and downloading it:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let ffmpeg = PathBuf::from("ffmpeg");
//! let executable = PathBuf::from("yt-dlp");
//! let output_dir = PathBuf::from("output");
//! let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
//!
//! let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir).expect("Failed to create fetcher");
//!
//! let video = fetcher.fetch_infos().await.expect("Failed to fetch video");
//! println!("Video title: {}", video.title);
//!
//! let format = video.best_video_format().unwrap(); // or video.best_audio_format if you want audio
//! fetcher.download_format(&format, "video.mp4").await.expect("Failed to download video format");
//! # }
//! ```
//!
//! - ⚙️ Combining an audio and a video file into a single file:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let ffmpeg = PathBuf::from("ffmpeg");
//! let executable = PathBuf::from("yt-dlp");
//! let output_dir = PathBuf::from("output");
//! let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
//!
//! let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir).expect("Failed to create fetcher");
//!
//! let video = fetcher.fetch_infos().await.expect("Failed to fetch video");
//! println!("Video title: {}", video.title);
//!
//! fetcher.download_video_stream("video.mp4").await.expect("Failed to download video");
//! fetcher.download_audio_stream("audio.mp3").await.expect("Failed to download audio");
//!
//! fetcher.combine_audio_and_video("video.mp4", "audio.mp3", "output.mp4").await.expect("Failed to combine audio and video");
//! # }
//! ```
//!
//! - 📸 Fetching a thumbnail and downloading it:
//! ```rust, no_run
//! # use yt_dlp::Youtube;
//! # use std::path::PathBuf;
//! # #[tokio::main]
//! # async fn main() {
//! let ffmpeg = PathBuf::from("ffmpeg");
//! let executable = PathBuf::from("yt-dlp");
//! let output_dir = PathBuf::from("output");
//! let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
//!
//! let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir).expect("Failed to create fetcher");
//!
//! let video = fetcher.fetch_infos().await.expect("Failed to fetch video");
//! println!("Video title: {}", video.title);
//!
//! fetcher.download_thumbnail("thumbnail.jpg").await.expect("Failed to download thumbnail");
//! # }
//! ```

use crate::error::{Error, Result};
use crate::fetcher::{FFmpeg, Fetcher, GitHubFetcher};
use crate::model::format::{Format, FormatType};
use crate::model::Video;
use crate::utils::file_system;
use derive_more::Display;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use utils::executor::Executor;

pub mod error;
pub mod fetcher;
pub mod model;
pub mod utils;

/// A YouTube video fetcher that uses yt-dlp to fetch video information and download it.
///
/// The 'yt-dlp' executable and 'ffmpeg' build can be installed with this fetcher.
///
/// The video can be downloaded with or without its audio, and the audio and video can be combined.
/// The video thumbnail can also be downloaded.
///
/// # Examples
///
/// ```rust, no_run
/// # use yt_dlp::Youtube;
/// # use std::path::PathBuf;
/// # #[tokio::main]
/// # async fn main() {
/// let ffmpeg = PathBuf::from("ffmpeg");
/// let executable = PathBuf::from("yt-dlp");
/// let output_dir = PathBuf::from("output");
/// let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();
///
/// let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir).expect("Failed to create fetcher");
///
/// let video = fetcher.fetch_infos().await.expect("Failed to fetch video");
/// println!("Video title: {}", video.title);
///
/// fetcher.download_video("video.mp4").await.expect("Failed to download video");
/// # }
/// ```
#[derive(Clone, Debug, Display)]
#[display("Youtube: url={}, output_dir={:?}, args={:?}", url, output_dir, args)]
pub struct Youtube {
    pub executable_path: PathBuf,
    pub ffmpeg_path: PathBuf,
    pub url: String,

    pub output_dir: PathBuf,
    pub args: Vec<String>,

    video: Option<Video>,
}

impl Youtube {
    /// Creates a new YouTube fetcher with the given yt-dlp executable, ffmpeg executable and video URL.
    /// The output directory can be void if you only want to fetch the video information.
    ///
    /// # Arguments
    ///
    /// * `executable` - The path to the yt-dlp executable.
    /// * `ffmpeg` - The path to the ffmpeg executable.
    /// * `url` - The URL of the video to fetch.
    /// * `output_dir` - The directory where the video will be downloaded.
    pub fn new(
        executable: PathBuf,
        ffmpeg: PathBuf,
        url: String,
        output_dir: PathBuf,
    ) -> Result<Self> {
        file_system::create_parent_dir(executable.clone())?;
        file_system::create_parent_dir(ffmpeg.clone())?;

        file_system::create_parent_dir(output_dir.clone())?;

        Ok(Self {
            executable_path: executable,
            ffmpeg_path: ffmpeg,
            url,
            output_dir,

            args: Vec::new(),
            video: None,
        })
    }

    /// Creates a new YouTube fetcher, and installs the yt-dlp and ffmpeg binaries.
    /// The output directory can be void if you only want to fetch the video information.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `executables_dir` - The directory where the binaries will be installed.
    /// * `url` - The URL of the video to fetch.
    /// * `output_dir` - The directory where the video will be downloaded.
    pub async fn new_with_binaries(
        executables_dir: PathBuf,
        url: String,
        output_dir: PathBuf,
    ) -> Result<Self> {
        let yt_dlp = executables_dir.join(utils::fetch_executable("yt-dlp"));
        let ffmpeg = executables_dir.join(utils::fetch_executable("ffmpeg"));

        Self::install_binaries(executables_dir).await?;
        Self::new(yt_dlp, ffmpeg, url, output_dir)
    }

    /// Installs the yt-dlp and ffmpeg binaries.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `destination` - The directory where the binaries will be installed.
    pub async fn install_binaries(destination: PathBuf) -> Result<()> {
        let cloned_destination = destination.clone();
        let youtube_handle =
            tokio::task::spawn(async move { Self::install_youtube(cloned_destination).await });

        let cloned_destination = destination.clone();
        let ffmpeg_handle =
            tokio::task::spawn(async move { Self::install_ffmpeg(cloned_destination).await });

        utils::await_two(youtube_handle, ffmpeg_handle).await?;
        Ok(())
    }

    /// Installs the yt-dlp binary, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `destination` - The directory where the binary will be installed.
    pub async fn install_youtube(destination: PathBuf) -> Result<PathBuf> {
        let youtube = GitHubFetcher::new("yt-dlp", "yt-dlp");
        let youtube_destination = destination.join(utils::fetch_executable("yt-dlp"));

        let release = youtube.fetch_release(None).await?;
        release.download(youtube_destination.clone()).await?;

        Ok(youtube_destination)
    }

    /// Installs the ffmpeg binary, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `destination` - The directory where the binary will be installed.
    pub async fn install_ffmpeg(destination: PathBuf) -> Result<PathBuf> {
        let ffmpeg = FFmpeg::new();
        let ffmpeg_archive = destination.join("ffmpeg-release.zip");

        let release = ffmpeg.fetch_binary().await?;
        release.download(ffmpeg_archive.clone()).await?;
        ffmpeg.extract_binary(ffmpeg_archive).await?;

        let ffmpeg_destination = destination.join(utils::fetch_executable("ffmpeg"));
        Ok(ffmpeg_destination)
    }

    /// Sets the arguments to pass to yt-dlp.
    pub fn with_args(&mut self, mut args: Vec<String>) -> &mut Self {
        self.args.append(&mut args);
        self
    }

    /// Adds an argument to pass to yt-dlp.
    pub fn with_arg(&mut self, arg: &str) -> &mut Self {
        self.args.push(arg.to_string());
        self
    }

    /// Updates the yt-dlp executable.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the yt-dlp executable could not be updated.
    pub async fn update_downloader(&self) -> Result<()> {
        let args = vec!["--update"];

        let executor = Executor {
            executable_path: self.executable_path.clone(),
            timeout: Duration::from_secs(30),
            args: utils::to_owned(args),
        };

        executor.execute().await?;
        Ok(())
    }

    /// Fetches the video information.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video information could not be fetched.
    pub async fn fetch_infos(&mut self) -> Result<Video> {
        self.with_arg("--no-progress");
        self.with_arg("--dump-json");

        let url = self.url.clone();
        self.with_arg(&url);

        let executor = Executor {
            executable_path: self.executable_path.clone(),
            timeout: Duration::from_secs(30),
            args: self.args.clone(),
        };

        let output = executor.execute().await?;
        let mut video: Video = serde_json::from_str(&output.stdout).map_err(Error::Serde)?;

        video.formats.iter_mut().for_each(|format| {
            FormatType::fetch_type(format);
        });

        self.video = Some(video.clone());
        Ok(video)
    }

    /// Downloads the video (with its audio), and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    pub async fn download_video(&mut self, file_name: &str) -> Result<PathBuf> {
        if self.video.is_none() {
            self.fetch_infos().await?;
        }

        self.download_fetched_video(file_name).await
    }

    /// Downloads the previously fetched video (with its audio), and returns its path.
    /// You must have fetched the video information before calling this function.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be downloaded, or if there is no video to download.
    pub async fn download_fetched_video(&self, file_name: &str) -> Result<PathBuf> {
        let youtube_arc = Arc::new(self.clone());
        let file_name = file_name.to_string();

        let video_handle = {
            let youtube_clone = Arc::clone(&youtube_arc);
            let video_name = format!("video_{}.mp4", file_name.clone());

            tokio::spawn(async move {
                youtube_clone
                    .download_fetched_video_stream(video_name)
                    .await
            })
        };

        let audio_handle = {
            let youtube_clone = Arc::clone(&youtube_arc);
            let audio_name = format!("audio_{}.m4a", file_name.clone());

            tokio::spawn(async move {
                youtube_clone
                    .download_fetched_audio_stream(audio_name)
                    .await
            })
        };

        utils::await_two(video_handle, audio_handle).await?;

        self.combine_audio_and_video(
            &format!("audio_{}.m4a", file_name),
            &format!("video_{}.mp4", file_name),
            &file_name,
        )
        .await
    }

    /// Downloads the video only, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    pub async fn download_video_stream(&mut self, file_name: &str) -> Result<PathBuf> {
        if self.video.is_none() {
            self.fetch_infos().await?;
        }

        self.download_fetched_video_stream(file_name.to_string())
            .await
    }

    /// Downloads the previously fetched video only, and returns its path.
    /// You must have fetched the video information before calling this function.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be downloaded, or if there is no video to download.
    pub async fn download_fetched_video_stream(&self, file_name: String) -> Result<PathBuf> {
        if self.video.is_none() {
            return Err(Error::Video("No video to download".to_string()));
        }

        let video = self.video.as_ref().unwrap();
        let best_video = video
            .best_video_format()
            .ok_or(Error::Video("No video format available".to_string()))?;

        self.download_format(best_video, &file_name).await
    }

    /// Downloads the audio, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    pub async fn download_audio_stream(&mut self, file_name: &str) -> Result<PathBuf> {
        if self.video.is_none() {
            self.fetch_infos().await?;
        }

        self.download_fetched_audio_stream(file_name.to_string())
            .await
    }

    /// Downloads the previously fetched audio, and returns its path.
    /// You must have fetched the video information before calling this function.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be downloaded, or if there is no video to download.
    pub async fn download_fetched_audio_stream(&self, file_name: String) -> Result<PathBuf> {
        if self.video.is_none() {
            return Err(Error::Video("No video to download".to_string()));
        }

        let video = self.video.as_ref().unwrap();
        let best_audio = video
            .best_audio_format()
            .ok_or(Error::Video("No audio format available".to_string()))?;

        self.download_format(best_audio, &file_name).await
    }

    /// Downloads a specific format, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be downloaded.
    pub async fn download_format(&self, format: &Format, file_name: &str) -> Result<PathBuf> {
        let path = self.output_dir.join(file_name);
        let url = format.download_info.url.clone();

        let fetcher = Fetcher::new(&url);
        fetcher.fetch_asset(path.clone()).await?;

        Ok(path)
    }

    /// Combines the audio and video files into a single file.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Errors
    ///
    /// This function will return an error if the audio and video files could not be combined.
    pub async fn combine_audio_and_video(
        &self,
        audio_file: &str,
        video_file: &str,
        output_file: &str,
    ) -> Result<PathBuf> {
        let audio_path = self.output_dir.join(audio_file);
        let video_path = self.output_dir.join(video_file);
        let output_path = self.output_dir.join(output_file);

        let audio = audio_path
            .to_str()
            .ok_or(Error::Path("Invalid audio path".to_string()))?;
        let video = video_path
            .to_str()
            .ok_or(Error::Path("Invalid video path".to_string()))?;
        let output = output_path
            .to_str()
            .ok_or(Error::Path("Invalid output path".to_string()))?;

        let args = vec![
            "-i", audio, "-i", video, "-c:v", "copy", "-c:a", "aac", output,
        ];

        let executor = Executor {
            executable_path: self.ffmpeg_path.clone(),
            timeout: Duration::from_secs(30),
            args: utils::to_owned(args),
        };

        executor.execute().await?;
        Ok(output_path)
    }

    /// Downloads the thumbnail of the video, usually in the highest resolution available.
    /// Be careful, this function may take a while to execute.
    pub async fn download_thumbnail(&mut self, file_name: &str) -> Result<PathBuf> {
        if self.video.is_none() {
            self.fetch_infos().await?;
        }

        self.download_fetched_thumbnail(file_name).await
    }

    /// Downloads the thumbnail of the previously fetched video, usually in the highest resolution available.
    /// You must have fetched the video information before calling this function.
    /// Be careful, this function may take a while to execute.
    pub async fn download_fetched_thumbnail(&self, file_name: &str) -> Result<PathBuf> {
        if self.video.is_none() {
            return Err(Error::Video("No video to download".to_string()));
        }

        let video = self.video.as_ref().unwrap();
        let thumbnail = video.thumbnail.clone();

        let path = self.output_dir.join(file_name);

        let fetcher = Fetcher::new(&thumbnail);
        fetcher.fetch_asset(path.clone()).await?;

        Ok(path)
    }
}
