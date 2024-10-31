use crate::setup::SHARED_SETUP;

pub mod setup;

#[cfg(test)]
#[tokio::test]
pub async fn video_fetching_test() {
    let mut fetcher = SHARED_SETUP.youtube.clone();

    let video = fetcher
        .fetch_infos()
        .await
        .expect("Failed to fetch video infos");

    assert_eq!(video.id, "dQw4w9WgXcQ");
    assert_eq!(video.title, "Rick Astley - Never Gonna Give You Up (Video)");
}

#[cfg(test)]
#[tokio::test]
pub async fn video_format_downloading_test() {
    let mut fetcher = SHARED_SETUP.youtube.clone();

    let video = fetcher
        .fetch_infos()
        .await
        .expect("Failed to fetch video infos");

    let file_name = format!("{}.mp4", video.title);
    let worst_format = video
        .worst_video_format()
        .expect("Failed to get best video format");
    fetcher
        .download_format(&worst_format, &file_name)
        .await
        .expect("Failed to download video format");

    let path = fetcher.output_dir.join(&file_name);

    assert!(path.exists());
    assert_eq!(path.extension().unwrap(), "mp4");

    let file_size = std::fs::metadata(&path)
        .expect("Failed to get file metadata")
        .len();
    let expected_size = worst_format
        .file_info
        .filesize
        .expect("Failed to get file size") as u64;

    assert_eq!(file_size, expected_size);

    std::fs::remove_file(&path).expect("Failed to remove file");
}

#[cfg(test)]
#[tokio::test]
pub async fn video_audio_format_downloading_test() {
    let mut fetcher = SHARED_SETUP.youtube.clone();

    let video = fetcher
        .fetch_infos()
        .await
        .expect("Failed to fetch video infos");

    let file_name = format!("{}.mp3", video.title);
    let worst_audio = video
        .worst_audio_format()
        .expect("Failed to get best audio format");
    fetcher
        .download_format(&worst_audio, &file_name)
        .await
        .expect("Failed to download audio format");

    let path = fetcher.output_dir.join(&file_name);

    assert!(path.exists());
    assert_eq!(path.extension().unwrap(), "mp3");

    let file_size = std::fs::metadata(&path)
        .expect("Failed to get file metadata")
        .len();
    let expected_size = worst_audio
        .file_info
        .filesize
        .expect("Failed to get file size") as u64;

    assert_eq!(file_size, expected_size);

    std::fs::remove_file(&path).expect("Failed to remove file");
}
