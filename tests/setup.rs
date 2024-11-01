use std::path::PathBuf;
use yt_dlp::model::Video;
use yt_dlp::{utils, Youtube};

#[cfg(test)]
lazy_static::lazy_static! {
    pub static ref SHARED_SETUP: SharedSetup = SharedSetup::new();
}

#[cfg(test)]
pub struct SharedSetup {
    pub youtube: Youtube,
    pub video: Video,
    pub url: String,
}

#[cfg(test)]
impl SharedSetup {
    pub fn new() -> Self {
        let destination = PathBuf::from("temp/bin");
        let output_dir = PathBuf::from("temp/output");
        let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");

        let ffmpeg_file = utils::fetch_executable("ffmpeg");
        let ffmpeg_path = destination.join(&ffmpeg_file);

        let executable_file = utils::fetch_executable("yt-dlp");
        let executable_path = destination.join(&executable_file);

        std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");

        let youtube = Youtube::new(executable_path, ffmpeg_path, url.clone(), output_dir)
            .expect("Failed to create YouTube instance");

        let serialized_video = std::fs::read_to_string("tests/data/video_infos.json")
            .expect("Failed to read video_infos.json");
        let video: Video =
            serde_json::from_str(&serialized_video).expect("Failed to deserialize video_infos.json");

        Self {
            youtube,
            video,
            url,
        }
    }
}

#[cfg(test)]
impl Drop for SharedSetup {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.youtube.executable_path);
        let _ = std::fs::remove_file(&self.youtube.ffmpeg_path);

        let _ = std::fs::remove_dir_all(&self.youtube.output_dir);
        let _ = std::fs::remove_dir_all("temp");
    }
}

#[cfg(test)]
#[tokio::test]
pub async fn setup_test() {
    let destination = PathBuf::from("temp/bin");

    Youtube::install_binaries(destination)
        .await
        .expect("Failed to install binaries");
}
