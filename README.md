<h2 align="center">⚙️ An asynchronous Rust library (with auto dependencies downloading) for yt-dlp, a feature-rich youtube (and others) audio/video downloader 🎬️</h2>

<div align="center">This library is a Rust wrapper around the yt-dlp command line tool, which is a fork of youtube-dl with a lot of additional features and improvements.</div>
<div align="center">The crate is designed to be used in a Rust project to download audio and video from various websites.</div>
<div align="center">You don't need to care about dependencies, yt-dlp and ffmpeg will be downloaded automatically.</div>

<br>
<div align="center">⚠️ The project is still in development, so if you encounter any bugs or have any feature requests, please open an issue or a discussion.</div>
<br>

<div align="center">
  <a href="https://github.com/boul2gom/yt-dlp/issues/new?assignees=&labels=bug&template=BUG_REPORT.md&title=bug%3A+">Report a Bug</a>
  ·
  <a href="https://github.com/boul2gom/yt-dlp/discussions/new?assignees=&labels=enhancement&title=feat%3A+">Request a Feature</a>
  ·
  <a href="https://github.com/boul2gom/yt-dlp/discussions/new?assignees=&labels=help%20wanted&title=ask%3A+">Ask a Question</a>
</div>

---

<p align="center">
  <img src="https://img.shields.io/github/actions/workflow/status/boul2gom/yt-dlp/ci-dev.yml?label=Develop%20CI&logo=Github" alt="Develop CI"/>
  <img src="https://img.shields.io/github/v/release/boul2gom/yt-dlp?label=Release&logo=Github" alt="Release"/>
  <img src="https://img.shields.io/github/license/boul2gom/yt-dlp?label=License&logo=Github" alt="License">
<p align="center">
  <img src="https://img.shields.io/github/discussions/boul2gom/yt-dlp?label=Discussions&logo=Github" alt="Discussions">
  <img src="https://img.shields.io/github/issues-raw/boul2gom/yt-dlp?label=Issues&logo=Github" alt="Issues">
  <img src="https://img.shields.io/github/issues-pr-raw/boul2gom/yt-dlp?label=Pull requests&logo=Github" alt="Pull requests">
  <img src="https://img.shields.io/github/stars/boul2gom/yt-dlp?label=Stars&logo=Github" alt="Stars">
  <img src="https://img.shields.io/github/forks/boul2gom/yt-dlp?label=Forks&logo=Github" alt="Forks">
</p>

---

# 📥 How to get it
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
yt-dlp = "latest version of the crate"
```

# 📚 Examples

- 📦 Installing the [yt-dlp](https://github.com/yt-dlp/yt-dlp/) and [ffmpeg](https://ffmpeg.org/) binaries:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let destination = PathBuf::from("bin");
    let output_dir = PathBuf::from("output");
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();

    let fetcher = Youtube::new_with_binaries(destination, url, output_dir).await.expect("Failed to install binaries");
}
```

- 📦 Installing the yt-dlp binary only:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let destination = PathBuf::from("bin");
    Youtube::install_youtube(destination).await.expect("Failed to install yt-dlp");
}
```

- 📦 Installing the ffmpeg binary only:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let destination = PathBuf::from("bin");
    Youtube::install_ffmpeg(destination).await.expect("Failed to install ffmpeg");
}
```

- 🔄 Updating the yt-dlp binary:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let ffmpeg = PathBuf::from("ffmpeg");
    let executable = PathBuf::from("yt-dlp");
    let output_dir = PathBuf::from("output");
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();

    let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir);
    fetcher.update_downloader().await.expect("Failed to update yt-dlp");
}
```

- 📥 Fetching a video (with its audio) and downloading it:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let ffmpeg = PathBuf::from("ffmpeg");
    let executable = PathBuf::from("yt-dlp");
    let output_dir = PathBuf::from("output");
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();

    let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir);

    let video = fetcher.fetch_infos().await.expect("Failed to fetch video infos");
    println!("Video title: {}", video.title);

    fetcher.download_video("video.mp4").await.expect("Failed to download video");
}
```

- 🎬 Fetching a video (without its audio) and downloading it:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let ffmpeg = PathBuf::from("ffmpeg");
    let executable = PathBuf::from("yt-dlp");
    let output_dir = PathBuf::from("output");
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();

    let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir);

    let video = fetcher.fetch_infos().await.expect("Failed to fetch video infos");
    println!("Video title: {}", video.title);

    fetcher.download_video_stream("video.mp4").await.expect("Failed to download video without audio");
}
```

- 🎵 Fetching an audio and downloading it:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let ffmpeg = PathBuf::from("ffmpeg");
    let executable = PathBuf::from("yt-dlp");
    let output_dir = PathBuf::from("output");
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();

    let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir);

    let audio = fetcher.fetch_audio_infos().await.expect("Failed to fetch audio infos");
    println!("Audio title: {}", audio.title);

    fetcher.download_audio_stream("audio.mp3").await.expect("Failed to download audio");
}
```

- 📜 Fetching a specific format and downloading it:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let ffmpeg = PathBuf::from("ffmpeg");
    let executable = PathBuf::from("yt-dlp");
    let output_dir = PathBuf::from("output");
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();

    let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir);

    let video = fetcher.fetch_infos().await.expect("Failed to fetch video infos");
    println!("Video title: {}", video.title);

    let format = video.best_video_format().unwrap(); // or video.best_audio_format if you want audio
    fetcher.download_format(&format, "video.mp4").await.expect("Failed to download video format");
}
```

- ⚙️ Combining an audio and a video file into a single file:
```rust
use yt_dlp::Youtube;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let ffmpeg = PathBuf::from("ffmpeg");
    let executable = PathBuf::from("yt-dlp");
    let output_dir = PathBuf::from("output");
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string();

    let mut fetcher = Youtube::new(executable, ffmpeg, url, output_dir);

    let video = fetcher.fetch_infos().await.expect("Failed to fetch video infos");
    println!("Video title: {}", video.title);

    fetcher.download_video_stream("video.mp4").await.expect("Failed to download video");
    fetcher.download_audio_stream("audio.mp3").await.expect("Failed to download audio");

    fetcher.combine_audio_and_video("video.mp4", "audio.mp3", "output.mp4").await.expect("Failed to combine audio and video");
}
```

# 💡Support coming soon
- [ ] Subtitles
- [ ] Chapters
- [ ] Heatmap
- [ ] Playlist (and index)
