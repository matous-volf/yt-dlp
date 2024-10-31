use crate::setup::SHARED_SETUP;

pub mod setup;

#[cfg(test)]
#[tokio::test]
pub async fn video_combining_test() {
    let mut fetcher = SHARED_SETUP.youtube.clone();

    let video = fetcher
        .fetch_infos()
        .await
        .expect("Failed to fetch video infos");

    let video_file_name = format!("{}.mp4", video.title);
    let audio_file_name = format!("{}.mp3", video.title);
    let output_file_name = format!("{}.mkv", video.title);

    fetcher
        .download_video(&video_file_name)
        .await
        .expect("Failed to download video");
    fetcher
        .download_audio_stream(&audio_file_name)
        .await
        .expect("Failed to download audio");

    fetcher
        .combine_audio_and_video(&audio_file_name, &video_file_name, &output_file_name)
        .await
        .expect("Failed to combine audio and video");

    let path = fetcher.output_dir.join(&output_file_name);

    assert!(path.exists());
    assert_eq!(path.extension().unwrap(), "mkv");

    let file_size = std::fs::metadata(&path)
        .expect("Failed to get file metadata")
        .len();
    assert!(file_size > 0);

    std::fs::remove_file(&path).expect("Failed to remove file");
}
