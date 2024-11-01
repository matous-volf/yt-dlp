use crate::setup::SHARED_SETUP;

pub mod setup;

#[cfg(test)]
#[tokio::test]
pub async fn video_infos_test() {
    let video = SHARED_SETUP.video.clone();

    assert_eq!(video.id, "dQw4w9WgXcQ");
    assert_eq!(video.title, "Rick Astley - Never Gonna Give You Up (Official Music Video)");
}

#[cfg(test)]
#[tokio::test]
pub async fn worst_video_format_downloading_test() {
    let fetcher = SHARED_SETUP.youtube.clone();
    let video = SHARED_SETUP.video.clone();

    let worst_format = video
        .worst_video_format()
        .expect("Failed to get best video format");
    fetcher
        .download_format(&worst_format, "worst_video_format.mp4")
        .await
        .expect("Failed to download video format");

    let path = fetcher.output_dir.join("worst_video_format.mp4");

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
pub async fn worst_audio_format_downloading_test() {
    let fetcher = SHARED_SETUP.youtube.clone();
    let video = SHARED_SETUP.video.clone();

    let worst_audio = video
        .worst_audio_format()
        .expect("Failed to get best audio format");
    fetcher
        .download_format(&worst_audio, "worst_audio_format.mp3")
        .await
        .expect("Failed to download audio format");

    let path = fetcher.output_dir.join("worst_audio_format.mp3");

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

#[cfg(test)]
#[tokio::test]
pub async fn worst_formats_combining_test() {
    let fetcher = SHARED_SETUP.youtube.clone();
    let video = SHARED_SETUP.video.clone();

    let worst_audio = video
        .worst_audio_format()
        .expect("Failed to get best audio format");
    fetcher
        .download_format(&worst_audio, "audio.mp3")
        .await
        .expect("Failed to download audio format");

    let worst_video = video
        .worst_video_format()
        .expect("Failed to get best video format");
    fetcher
        .download_format(&worst_video, "video.mp4")
        .await
        .expect("Failed to download video format");


    let output_path = fetcher.combine_audio_and_video("audio.mp3",
                                                      "video.mp4",
                                                      "output.mp4").await.expect("Failed to combine audio and video");

    assert!(output_path.exists());
    assert_eq!(output_path.extension().unwrap(), "mp4");

    let file_size = std::fs::metadata(&output_path)
        .expect("Failed to get file metadata")
        .len();
    assert!(file_size > 0);

    std::fs::remove_file(&output_path).expect("Failed to remove file");
    std::fs::remove_file(fetcher.output_dir.join("audio.mp3")).expect("Failed to remove file");
    std::fs::remove_file(fetcher.output_dir.join("video.mp4")).expect("Failed to remove file");
}