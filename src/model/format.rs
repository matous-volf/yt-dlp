use serde::{Deserialize, Serialize};

/// Represents an available format of a video.
/// It can be audio, video, both of them, a manifest, or a storyboard.
///
/// A manifest is a file that contains metadata about the video streams, and how to assemble them.
/// A storyboard is a file that contains grid of images from the video, allowing users to preview the video.
/// Usually, these formats are not meant to be downloaded, but to be used by the player.
/// So, in most cases, you can ignore them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Format {
    /// The display name of the format, e.g. '303 - 1920x1080 (1080p60)'.
    pub format: String,
    /// The format ID, e.g. '303'.
    pub format_id: String,
    /// The format note, e.g. '1080p60'.
    pub format_note: Option<String>,

    /// The type of the format.
    #[serde(default)]
    pub protocol: Protocol,
    /// The language of the format.
    pub language: Option<String>,

    /// If the format has DRM.
    pub has_drm: Option<bool>,
    /// The extension of the file containing the format.
    #[serde(default)]
    pub container: Option<Container>,

    /// All the codec-related information.
    #[serde(flatten)]
    pub codec_info: CodecInfo,
    /// All the video resolution-related information.
    #[serde(flatten)]
    pub video_resolution: VideoResolution,
    /// All the download-related information.
    #[serde(flatten)]
    pub download_info: DownloadInfo,
    /// All the quality-related information.
    #[serde(flatten)]
    pub quality_info: QualityInfo,
    /// All the file-related information.
    #[serde(flatten)]
    pub file_info: FileInfo,
    /// All the storyboard-related information.
    #[serde(flatten)]
    pub storyboard_info: StoryboardInfo,
    /// All the rates-related information.
    #[serde(flatten)]
    pub rates_info: RatesInfo,

    /// The type of the format, computed based on other fields.
    #[serde(skip)]
    pub format_type: FormatType,
}

impl Format {
    pub fn is_video(&self) -> bool {
        self.format_type.is_video()
    }

    pub fn is_audio(&self) -> bool {
        self.format_type.is_audio()
    }
}

/// Represents the codec information of a format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodecInfo {
    /// The name of the audio codec, e.g. 'opus' or 'mp4a.xx' (where 'xx' is the codec version).
    #[serde(rename = "acodec")]
    pub audio_codec: Option<String>,
    /// The name of the video codec, e.g. 'vp9' or 'avc1.xx' (where 'xx' is the codec version).
    #[serde(rename = "vcodec")]
    pub video_codec: Option<String>,
    /// The extension of the audio file.
    #[serde(default)]
    pub audio_ext: Extension,
    /// The extension of the video file.
    #[serde(default)]
    pub video_ext: Extension,
    /// The number of audio channels.
    pub audio_channels: Option<i64>,
    /// The audio sample rate.
    pub asr: Option<i64>,
}

/// Represents the video resolution information of a format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoResolution {
    /// The width of the video.
    pub width: Option<i64>,
    /// The height of the video.
    pub height: Option<i64>,
    /// The frame rate of the video.
    pub fps: Option<f64>,
    /// The resolution of the video, e.g. '1920x1080' or 'audio only'.
    pub resolution: String,
    /// The aspect ratio of the video, usually '1.77' or '1.78' (corresponding to 16:9).
    pub aspect_ratio: Option<f64>,
}

/// Represents the download information of a format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadInfo {
    /// The decrypted URL of the format.
    pub url: String,
    /// The extension of the format.
    #[serde(default)]
    pub ext: Extension,
    /// The HTTP headers used by the downloader.
    pub http_headers: HttpHeaders,
    /// The manifest URL, if the format is a manifest.
    pub manifest_url: Option<String>,
    /// The options used by the downloader.
    pub downloader_options: Option<DownloaderOptions>,
}

/// Represents the quality information of a format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityInfo {
    /// A relative quality score, e.g. '-1' (for example, if the format is a manifest) or '9.5'.
    pub quality: Option<f64>,
    /// If the format is using a large dynamic range.
    #[serde(default)]
    pub dynamic_range: Option<DynamicRange>,
}

/// Represents the file information of a format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    /// The approximate file size of the format.
    pub filesize_approx: Option<i64>,
    /// The exact file size of the format.
    pub filesize: Option<i64>,
}

/// Represents the rates information of a format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatesInfo {
    /// The video bitrate of the format.
    #[serde(rename = "vbr")]
    pub video_rate: Option<f64>,
    /// The audio bitrate of the format.
    #[serde(rename = "abr")]
    pub audio_rate: Option<f64>,
    /// The total bitrate (video + audio) of the format.
    #[serde(rename = "tbr")]
    pub total_rate: Option<f64>,
}

/// Represents the storyboard information of a format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryboardInfo {
    /// The number of rows in the storyboard.
    pub rows: Option<i64>,
    /// The number of columns in the storyboard.
    pub columns: Option<i64>,
    /// The fragments of the storyboard.
    pub fragments: Option<Vec<Fragment>>,
}

/// Represents a fragment of a storyboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fragment {
    /// The URL of the fragment.
    pub url: String,
    /// The duration of the fragment.
    pub duration: f64,
}

/// Represents the options used by the downloader.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloaderOptions {
    /// The size of the HTTP chunk.
    pub http_chunk_size: i64,
}

/// Represents the HTTP headers used by the downloader.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HttpHeaders {
    /// The user agent used by the downloader.
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    /// The accept header used by the downloader.
    pub accept: String,
    /// The accept language used by the downloader.
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    /// The accept encoding used by the downloader.
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: String,
}

/// The available extensions of a format.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Extension {
    M4A,
    Mp4,
    Webm,

    Mhtml,

    #[default]
    Unknown,
}

/// The available containers extensions of a format.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Container {
    #[serde(rename = "webm_dash")]
    Webm,
    #[serde(rename = "m4a_dash")]
    M4A,
    #[serde(rename = "mp4_dash")]
    Mp4,

    #[default]
    Unknown,
}

/// The available protocols of a format.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    /// The HTTP protocol, used for audio and video formats.
    Https,
    /// The M3U8 protocol, used for manifest formats.
    #[serde(rename = "m3u8_native")]
    M3U8Native,
    /// The MHTML protocol, used for storyboard formats.
    Mhtml,

    #[default]
    Unknown,
}

/// The available dynamic ranges of a format.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DynamicRange {
    SDR,
    HDR,

    #[default]
    Unknown,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FormatType {
    Audio,
    Video,
    AudioAndVideo,

    Manifest,
    Storyboard,

    #[default]
    Unknown,
}

impl FormatType {
    pub fn fetch_type(format: &mut Format) {
        if format.download_info.manifest_url.is_some() {
            format.format_type = FormatType::Manifest;
            return;
        }

        if format.storyboard_info.fragments.is_some() {
            format.format_type = FormatType::Storyboard;
            return;
        }

        let audio = format.codec_info.audio_codec.is_some();
        let video = format.codec_info.video_codec.is_some();

        format.format_type = match (audio, video) {
            (true, true) => FormatType::AudioAndVideo,
            (true, false) => FormatType::Audio,
            (false, true) => FormatType::Video,
            _ => FormatType::Unknown,
        };
    }

    pub fn is_video(&self) -> bool {
        matches!(self, FormatType::Video | FormatType::AudioAndVideo)
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, FormatType::Audio | FormatType::AudioAndVideo)
    }

    pub fn is_storyboard(&self) -> bool {
        matches!(self, FormatType::Storyboard)
    }

    pub fn is_manifest(&self) -> bool {
        matches!(self, FormatType::Manifest)
    }
}
