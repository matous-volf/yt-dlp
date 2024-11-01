use crate::setup::SHARED_SETUP;

pub mod setup;

#[cfg(test)]
#[tokio::test]
pub async fn video_thumbnail_test() {
    let mut fetcher = SHARED_SETUP.youtube.clone();
    let video = SHARED_SETUP.video.clone();

    let file_name = format!("{}.jpg", video.title);
    fetcher
        .download_thumbnail(&file_name)
        .await
        .expect("Failed to download thumbnail");

    let path = fetcher.output_dir.join(&file_name);

    assert!(path.exists());
    assert_eq!(path.extension().unwrap(), "jpg");

    let file_size = std::fs::metadata(&path)
        .expect("Failed to get file metadata")
        .len();

    assert!(file_size > 0);

    std::fs::remove_file(&path).expect("Failed to remove file");
}
