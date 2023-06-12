#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

//mod template;
mod video;

//pub use template::{OutputTemplateBuilder, OutputTemplateVariable};
pub use video::{DownloadFormats, DownloadProgress, DownloadStopped,
	DownloadSubtitles, DownloadTitle, VideoDownloader, VideoDownloaderOptions};
