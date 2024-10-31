use std::path::PathBuf;
use yt_dlp::{utils, Youtube};

#[cfg(test)]
lazy_static::lazy_static! {
    pub static ref SHARED_SETUP: SharedSetup = SharedSetup::new();
}

#[cfg(test)]
pub struct SharedSetup {
    pub youtube: Youtube,
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

        let youtube = Youtube::new(executable_path, ffmpeg_path, url.clone(), output_dir)
            .expect("Failed to create YouTube instance");

        Self { youtube, url }
    }
}

#[cfg(test)]
impl Drop for SharedSetup {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.youtube.executable_path);
        let _ = std::fs::remove_file(&self.youtube.ffmpeg_path);

        let _ = std::fs::remove_dir_all(&self.youtube.output_dir);
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
