use std::path::PathBuf;
use yt_dlp::utils;

pub mod setup;

#[cfg(test)]
#[tokio::test]
pub async fn youtube_binary_test() {
    let executable_name = utils::fetch_executable("yt-dlp");
    let executable_path = PathBuf::from(format!("temp/bin/{}", executable_name));

    assert!(executable_path.exists());
}

#[cfg(test)]
#[tokio::test]
pub async fn ffmpeg_binary_test() {
    let ffmpeg_name = utils::fetch_executable("ffmpeg");
    let ffmpeg_path = PathBuf::from(format!("temp/bin/{}", ffmpeg_name));

    assert!(ffmpeg_path.exists());
}
