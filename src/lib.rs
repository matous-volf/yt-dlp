#![doc = include_str!("../README.md")]

use crate::error::{Error, Result};
use crate::fetcher::deps::{Libraries, LibraryInstaller};
use crate::fetcher::Fetcher;
use crate::model::format::Format;
use crate::model::Video;
use crate::utils::file_system;
use derive_more::Display;
use std::path::{Path, PathBuf};
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
/// # use yt_dlp::fetcher::deps::Libraries;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let libraries_dir = PathBuf::from("libs");
/// let output_dir = PathBuf::from("output");
///
/// let youtube = libraries_dir.join("yt-dlp");
/// let ffmpeg = libraries_dir.join("ffmpeg");
///
/// let libraries = Libraries::new(youtube, ffmpeg);
/// let mut fetcher = Youtube::new(libraries, output_dir)?;
///
/// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
/// let video = fetcher.fetch_video_infos(url).await?;
/// println!("Video title: {}", video.title);
///
/// fetcher.download_video(&video, "video.mp4").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Display)]
#[display("Youtube: output_dir={:?}, args={:?}", output_dir, args)]
pub struct Youtube {
    /// The required libraries.
    pub libraries: Libraries,

    /// The directory where the video (or formats) will be downloaded.
    pub output_dir: PathBuf,
    /// The arguments to pass to 'yt-dlp'.
    pub args: Vec<String>,
}

impl Youtube {
    /// Creates a new YouTube fetcher with the given yt-dlp executable, ffmpeg executable and video URL.
    /// The output directory can be void if you only want to fetch the video information.
    ///
    /// # Arguments
    ///
    /// * `libraries` - The required libraries.
    /// * `output_dir` - The directory where the video will be downloaded.
    ///
    /// # Errors
    ///
    /// This function will return an error if the parent directories of the executables and output directory could not be created.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let libraries_dir = PathBuf::from("libs");
    /// let output_dir = PathBuf::from("output");
    ///
    /// let youtube = libraries_dir.join("yt-dlp");
    /// let ffmpeg = libraries_dir.join("ffmpeg");
    ///
    /// let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub fn new(libraries: Libraries, output_dir: impl AsRef<Path>) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Creating a new video fetcher");

        file_system::create_parent_dir(&output_dir)?;

        Ok(Self {
            libraries,

            output_dir: output_dir.as_ref().to_path_buf(),
            args: Vec::new(),
        })
    }

    /// Creates a new YouTube fetcher, and installs the yt-dlp and ffmpeg binaries.
    /// The output directory can be void if you only want to fetch the video information.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `executables_dir` - The directory where the binaries will be installed.
    /// * `output_dir` - The directory where the video will be downloaded.
    ///
    /// # Errors
    ///
    /// This function will return an error if the executables could not be installed.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let executables_dir = PathBuf::from("libs");
    /// let output_dir = PathBuf::from("output");
    ///
    /// let fetcher = Youtube::with_new_binaries(executables_dir, output_dir).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn with_new_binaries(
        executables_dir: impl AsRef<Path>,
        output_dir: impl AsRef<Path>,
    ) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Creating a new video fetcher with binaries installation");

        let installer = LibraryInstaller::new(executables_dir.as_ref().to_path_buf());
        let youtube = installer.install_youtube(None).await?;
        let ffmpeg = installer.install_ffmpeg(None).await?;

        let libraries = Libraries::new(youtube, ffmpeg);
        Self::new(libraries, output_dir)
    }

    /// Sets the arguments to pass to yt-dlp.
    ///
    /// # Arguments
    ///
    /// * `args` - The arguments to pass to yt-dlp.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let mut fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let args = vec!["--no-progress".to_string()];
    /// fetcher.with_args(args);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_args(&mut self, mut args: Vec<String>) -> &mut Self {
        self.args.append(&mut args);
        self
    }

    /// Adds an argument to pass to yt-dlp.
    ///
    /// # Arguments
    ///
    /// * `arg` - The argument to pass to yt-dlp.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let mut fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// fetcher.with_arg("--no-progress");
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// fetcher.update_downloader().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn update_downloader(&self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Updating the downloader");

        let args = vec!["--update"];

        let executor = Executor {
            executable_path: self.libraries.youtube.clone(),
            timeout: Duration::from_secs(30),
            args: utils::to_owned(args),
        };

        executor.execute().await?;
        Ok(())
    }

    /// Fetches the video information from the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to fetch.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video information could not be fetched.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video = fetcher.fetch_video_infos(url).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn fetch_video_infos(&self, url: String) -> Result<Video> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Fetching video information for {}", url);

        let download_args = vec!["--no-progress", "--dump-json", &url];

        let mut final_args = self.args.clone();
        final_args.append(&mut utils::to_owned(download_args));

        let executor = Executor {
            executable_path: self.libraries.youtube.clone(),
            timeout: Duration::from_secs(30),
            args: final_args,
        };

        let output = executor.execute().await?;
        let video: Video = serde_json::from_str(&output.stdout).map_err(Error::Serde)?;

        Ok(video)
    }

    /// Downloads the video (with its audio) from the given URL, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download.
    /// * `output` - The name of the file to save the video to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video_path = fetcher.download_video_from_url(url, "my-video.mp4").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_video_from_url(&self, url: String, output: &str) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading video from {}", url);

        let video = self.fetch_video_infos(url.clone()).await?;

        self.download_video(&video, output).await
    }

    /// Downloads the video (with its audio), and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `video` - The video to download.
    /// * `output` - The name of the file to save the video to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video = fetcher.fetch_video_infos(url).await?;
    ///
    /// let video_path = fetcher.download_video(&video, "my-video.mp4").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_video(&self, video: &Video, output: &str) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading video {}", video.title);

        let output_path = self.output_dir.join(output);
        let file_name = file_system::try_without_extension(output_path.clone())?;

        let audio_name = format!("audio-{}.mp3", file_name.clone());
        self.download_audio_stream(video, &audio_name).await?;

        let video_name = format!("video-{}.mp4", file_name.clone());
        self.download_video_stream(video, &video_name).await?;

        self.combine_audio_and_video(&audio_name, &video_name, output)
            .await
    }

    /// Fetch the YouTube video from the given URL, the video only, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download.
    /// * `output` - The name of the file to save the video to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video_path = fetcher.download_video_stream_from_url(url, "my-video-stream.mp4").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_video_stream_from_url(
        &self,
        url: String,
        output: &str,
    ) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading video stream from {}", url);

        let video = self.fetch_video_infos(url).await?;

        self.download_video_stream(&video, output).await
    }

    /// Fetch the YouTube video, the video only, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `video` - The video to download.
    /// * `output` - The name of the file to save the video to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video = fetcher.fetch_video_infos(url).await?;
    ///
    /// let video_path = fetcher.download_video_stream(&video, "my-video-stream.mp4").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_video_stream(&self, video: &Video, output: &str) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading video stream {}", video.title);

        let best_video = video
            .best_video_format()
            .ok_or(Error::Video("No video format available".to_string()))?;

        self.download_format(best_video, output).await
    }

    /// Downloads the audio from the given URL, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download.
    /// * `output` - The name of the file to save the audio to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let audio_path = fetcher.download_audio_stream_from_url(url, "my-audio-stream.mp3").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_audio_stream_from_url(
        &self,
        url: String,
        output: &str,
    ) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading audio stream from {}", url);

        let video = self.fetch_video_infos(url).await?;

        self.download_audio_stream(&video, output).await
    }

    /// Downloads the audio, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `video` - The video to download.
    /// * `output` - The name of the file to save the audio to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be fetched or downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video = fetcher.fetch_video_infos(url).await?;
    ///
    /// let audio_path = fetcher.download_audio_stream(&video, "my-audio-stream.mp3").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_audio_stream(&self, video: &Video, output: &str) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading audio stream {}", video.title);

        let best_audio = video
            .best_audio_format()
            .ok_or(Error::Video("No audio format available".to_string()))?;

        self.download_format(best_audio, output).await
    }

    /// Downloads a specific format, and returns its path.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `format` - The format to download.
    /// * `output` - The name of the file to save the format to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the video could not be downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video = fetcher.fetch_video_infos(url).await?;
    ///
    /// let video_format = video.best_video_format().unwrap();
    /// let format_path = fetcher.download_format(&video_format, "my-video-stream.mp4").await?;
    ///
    /// let audio_format = video.worst_audio_format().unwrap();
    /// let audio_path = fetcher.download_format(&audio_format, "my-audio-stream.mp3").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_format(&self, format: &Format, output: &str) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading format {}", format.download_info.url);

        let path = self.output_dir.join(output);
        let url = format.download_info.url.clone();

        let fetcher = Fetcher::new(&url);
        fetcher.fetch_asset(path.clone()).await?;

        Ok(path)
    }

    /// Combines the audio and video files into a single file.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `audio_file` - The name of the audio file to combine.
    /// * `video_file` - The name of the video file to combine.
    /// * `output_file` - The name of the output file.
    ///
    /// # Errors
    ///
    /// This function will return an error if the audio and video files could not be combined.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video = fetcher.fetch_video_infos(url).await?;
    ///
    /// let audio_format = video.best_audio_format().unwrap();
    /// let audio_path = fetcher.download_format(&audio_format, "audio-stream.mp3").await?;
    ///
    /// let video_format = video.worst_video_format().unwrap();
    /// let format_path = fetcher.download_format(&video_format, "video-stream.mp4").await?;
    ///
    /// let output_path = fetcher.combine_audio_and_video("audio-stream.mp3", "video-stream.mp4", "my-output.mp4").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn combine_audio_and_video(
        &self,
        audio_file: &str,
        video_file: &str,
        output_file: &str,
    ) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Combining audio and video files {} and {}, into {}",
            audio_file,
            video_file,
            output_file
        );

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
            executable_path: self.libraries.ffmpeg.clone(),
            timeout: Duration::from_secs(30),
            args: utils::to_owned(args),
        };

        executor.execute().await?;
        Ok(output_path)
    }

    /// Downloads the thumbnail of the video from the given URL, usually in the highest resolution available.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download the thumbnail from.
    /// * `file_name` - The name of the file to save the thumbnail to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the thumbnail could not be fetched or downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let thumbnail_path = fetcher.download_thumbnail_from_url(url, "thumbnail.jpg").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_thumbnail_from_url(
        &self,
        url: String,
        file_name: &str,
    ) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading thumbnail from {}", url);

        let video = self.fetch_video_infos(url).await?;

        self.download_thumbnail(&video, file_name).await
    }

    /// Downloads the thumbnail of the video from the given URL, usually in the highest resolution available.
    /// Be careful, this function may take a while to execute.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download the thumbnail from.
    /// * `file_name` - The name of the file to save the thumbnail to.
    ///
    /// # Errors
    ///
    /// This function will return an error if the thumbnail could not be fetched or downloaded.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use yt_dlp::Youtube;
    /// # use std::path::PathBuf;
    /// # use yt_dlp::fetcher::deps::Libraries;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let libraries_dir = PathBuf::from("libs");
    /// # let output_dir = PathBuf::from("output");
    /// # let youtube = libraries_dir.join("yt-dlp");
    /// # let ffmpeg = libraries_dir.join("ffmpeg");
    /// # let libraries = Libraries::new(youtube, ffmpeg);
    /// let fetcher = Youtube::new(libraries, output_dir)?;
    ///
    /// let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    /// let video = fetcher.fetch_video_infos(url).await?;
    ///
    /// let thumbnail_path = fetcher.download_thumbnail(&video, "thumbnail.jpg").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", instrument(level = "debug"))]
    pub async fn download_thumbnail(&self, video: &Video, file_name: &str) -> Result<PathBuf> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Downloading thumbnail {}", video.title);

        let path = self.output_dir.join(file_name);

        let fetcher = Fetcher::new(&video.thumbnail);
        fetcher.fetch_asset(path.clone()).await?;

        Ok(path)
    }
}
