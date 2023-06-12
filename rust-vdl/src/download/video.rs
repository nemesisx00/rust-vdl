#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::process::Stdio;
use futures::StreamExt;
use log::{debug, error, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::process::{Child, Command, ChildStderr, ChildStdout};
use tokio_util::codec::{FramedRead, LinesCodec};
use crate::dir::getUserDownloadsDir;

#[cfg(windows)] extern crate winapi;

const Regex_SubtitleLanguages: &str = r"\[info\].*?: Downloading subtitles: (.*)";
const Regex_VideoFormats: &str = r"\[info\].*?: Downloading [0-9]+ format\(s\): (.*)";
const Regex_VideoTitle: &str = r"\[download\] Destination:.*[\\\/](.*)\..*\..{3}";

// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DownloadStopped
{
	pub stopped: bool,
	pub forceStop: bool,
}

impl std::fmt::Display for DownloadStopped
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let formatted = format!("Has stopped: {} {}", self.stopped, match self.forceStop { true => "(Forced)", false => "" });
		return f.write_str(formatted.as_str());
    }
}

// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DownloadTitle
{
	pub title: String,
}

impl std::fmt::Display for DownloadTitle
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let formatted = format!("Title: {}", self.title.to_owned());
		return f.write_str(formatted.as_str());
    }
}

// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DownloadFormats
{
	pub formats: Vec<String>,
}

impl std::fmt::Display for DownloadFormats
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let formatted = format!("Formats to download: {}", self.formats.join(", "));
		return f.write_str(formatted.as_str());
    }
}

// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DownloadSubtitles
{
	pub languages: Vec<String>,
}

impl std::fmt::Display for DownloadSubtitles
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let formatted = format!("Subtitles to download: {}", self.languages.join(", "));
		return f.write_str(formatted.as_str());
    }
}

// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DownloadProgress
{
	pub estimatedSize: String,
	pub estimatedTime: String,
	pub fragmentStatus: String,
	pub percentComplete: String,
	pub transferRate: String,
}

impl DownloadProgress
{
	pub fn isValid(&self) -> bool
	{
		return !self.percentComplete.is_empty();
	}
	
	pub fn update(&mut self, instance: Self)
	{
		self.estimatedSize = instance.estimatedSize;
		self.estimatedTime = instance.estimatedTime;
		self.fragmentStatus = instance.fragmentStatus;
		self.percentComplete = instance.percentComplete;
		self.transferRate = instance.transferRate;
	}
}

impl std::fmt::Display for DownloadProgress
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let formatted = format!("Progress: {} {} {} {} {}", self.percentComplete, self.transferRate, self.estimatedSize, self.estimatedTime, self.fragmentStatus);
		return f.write_str(formatted.as_str());
    }
}

impl From<String> for DownloadProgress
{
	fn from(value: String) -> Self
	{
		let mut instance = Self::default();
		
		value.split_whitespace()
			.for_each(|w| {
				w.ends_with("%").then(|| instance.percentComplete = w.to_owned());
				w.ends_with("B").then(|| instance.estimatedSize = w.to_owned());
				w.ends_with("B/s").then(|| instance.transferRate = w.to_owned());
				w.find(":").is_some().then(|| instance.estimatedTime = w.to_owned());
				w.ends_with(")").then(|| instance.fragmentStatus = w[..w.len()-1].to_owned());
			});
		
		return instance;
	}
}

impl Into<String> for DownloadProgress
{
	fn into(self) -> String { return format!("{}", self); }
}

unsafe impl Send for DownloadProgress {}

// --------------------------------------------------

const Default_Format: &str = "bv*+ba/b";
const Default_OutputTemplate: &str = "%(upload_date)s - %(title)s.%(ext)s";
const Option_OutputOnNewLines: &str = "--newline";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideoDownloaderOptions
{
	pub ageLimit: i64,
	pub convertSubs: String,
	pub convertThumbnails: String,
	pub downloadPlaylist: bool,
	pub embedMetadata: bool,
	pub ffmpegLocation: String,
	pub format: String,
	pub formatSort: String,
	pub limitRate: String,
	pub output: String,
	pub outputPath: String,
	pub preferFreeFormats: bool,
	pub subFormat: String,
	pub subLangs: String,
	pub username: String,
	pub writeAutoSubs: bool,
	pub writeSubs: bool,
	pub writeThumbnail: bool,
}

impl Default for VideoDownloaderOptions
{
	fn default() -> Self
	{
		return Self
		{
			ageLimit: 0,
			convertSubs: String::default(),
			convertThumbnails: String::default(),
			downloadPlaylist: false,
			embedMetadata: false,
			ffmpegLocation: String::default(),
			format: Default_Format.to_string(),
			formatSort: String::default(),
			limitRate: String::default(),
			output: Default_OutputTemplate.to_owned(),
			outputPath: getUserDownloadsDir(),
			preferFreeFormats: false,
			subFormat: String::default(),
			subLangs: String::default(),
			username: String::default(),
			writeAutoSubs: false,
			writeSubs: false,
			writeThumbnail: false,
		};
	}
}

impl VideoDownloaderOptions
{
	pub fn generateArgumentList(&self) -> Vec<String>
	{
		let mut args = vec![];
		
		if self.ageLimit > 0
		{
			args.push("--age-limit".to_string());
			args.push(self.ageLimit.to_string());
		}
		
		if !self.convertSubs.is_empty()
		{
			args.push("--convert-subs".to_string());
			args.push(self.convertSubs.to_owned());
		}
		
		if !self.convertThumbnails.is_empty()
		{
			args.push("--convert-thumbnails".to_string());
			args.push(self.convertThumbnails.to_owned());
		}
		
		match self.downloadPlaylist
		{
			true => args.push("--yes-playlist".to_string()),
			false => args.push("--no-playlist".to_string()),
		}
		
		match self.embedMetadata
		{
			true => args.push("--embed-metadata".to_string()),
			false => args.push("--no-embed-metadata".to_string()),
		}
		
		if !self.ffmpegLocation.is_empty()
		{
			args.push("--ffmpeg-location".to_string());
			args.push(self.ffmpegLocation.to_owned());
		}
		
		if !self.format.is_empty()
		{
			args.push("--format".to_string());
			args.push(self.format.to_owned());
		}
		
		if !self.formatSort.is_empty()
		{
			args.push("--format-sort".to_string());
			args.push(self.formatSort.to_owned());
		}
		
		if !self.limitRate.is_empty()
		{
			args.push("--limit-rate".to_string());
			args.push(self.limitRate.to_owned());
		}
		
		if !self.output.is_empty()
		{
			args.push("--output".to_string());
			args.push(self.output.to_owned());
		}
		
		if !self.outputPath.is_empty()
		{
			args.push("--paths".to_string());
			args.push(self.outputPath.to_owned());
		}
		
		match self.preferFreeFormats
		{
			true => args.push("--prefer-free-formats".to_string()),
			false => args.push("--no-prefer-free-formats".to_string()),
		}
		
		if !self.subFormat.is_empty()
		{
			args.push("--sub-format".to_string());
			args.push(self.subFormat.to_owned());
		}
		
		if !self.subLangs.is_empty()
		{
			args.push("--sub-langs".to_string());
			args.push(self.subLangs.to_owned());
		}
		
		if !self.username.is_empty()
		{
			args.push("--username".to_string());
			args.push(self.username.to_owned());
		}
		
		match self.writeAutoSubs
		{
			true => args.push("--write-auto-subs".to_string()),
			false => args.push("--no-write-auto-subs".to_string()),
		}
		
		match self.writeSubs
		{
			true => args.push("--write-subs".to_string()),
			false => args.push("--no-write-subs".to_string()),
		}
		
		match self.writeThumbnail
		{
			true => args.push("--write-thumbnail".to_string()),
			false => args.push("--no-write-thumbnail".to_string()),
		}
		
		return args;
	}
}

// --------------------------------------------------

pub struct VideoDownloader
{
	pub binary: String,
	pub options: VideoDownloaderOptions,
	pub child: Option<Child>,
	formatRegex: Regex,
	subtitleRegex: Regex,
	titleRegex: Regex,
}

impl VideoDownloader
{
	pub fn new(binary: String, options: VideoDownloaderOptions) -> Self
	{
		let formatRegex = Regex::new(Regex_VideoFormats).expect("Failed to compile Video Formats regular expression.");
		let subtitleRegex = Regex::new(Regex_SubtitleLanguages).expect("Failed to compile Subtitle Languages regular expression");
		let titleRegex = Regex::new(Regex_VideoTitle).expect("Failed to compile Video Title regular expression.");
		
		return Self
		{
			binary: binary.into(),
			options,
			child: None,
			formatRegex,
			subtitleRegex,
			titleRegex,
		};
	}
	
	#[allow(dead_code)]
	pub async fn cancel(&mut self)
	{
		if self.child.is_some()
		{
			match self.child.as_mut().unwrap().kill().await
			{
				Ok(_) => debug!("Canceled child process!"),
				Err(e) => error!("{}", e),
			};
		}
	}
	
	pub async fn download(&mut self, video: String,
		formatsHandler: Box<dyn Fn(DownloadFormats) + Send>,
		progressHandler: Box<dyn Fn(DownloadProgress) + Send>,
		stoppedHandler: Box<dyn Fn(DownloadStopped) + Send>,
		subtitlesHandler: Box<dyn Fn(DownloadSubtitles) + Send>,
		titleHandler: Box<dyn Fn(DownloadTitle) + Send>
	)
	{
		if !video.is_empty()
		{
			let mut args = vec![];
			
			let generatedArgs = self.options.generateArgumentList();
			generatedArgs.iter().for_each(|s| args.push(s.as_str()));
			
			args.push(video.as_str());
			
			let proc = self.spawnCommand(args.as_mut());
			match proc
			{
				Ok(mut child) => {
					self.processOutput(
						child.stdout.take(), child.stderr.take(),
						formatsHandler, progressHandler, stoppedHandler, subtitlesHandler, titleHandler
					).await;
					self.child = Some(child);
				},
				Err(e) => error!("Error downloading video: {} -> {}", video, e)
			};
		}
	}
	
	#[cfg(windows)]
	fn spawnCommand(&self, args: &mut Vec<&str>) -> tokio::io::Result<Child>
	{
		let mut finalArgs = vec![Option_OutputOnNewLines.clone()];
		finalArgs.append(args);
		
		return Command::new(self.binary.to_owned())
			.creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
			.kill_on_drop(true)
			.stderr(Stdio::piped())
			.stdout(Stdio::piped())
			.args(finalArgs)
			.spawn();
	}
	
	#[cfg(not(windows))]
	fn spawnCommand(&self, args: &mut Vec<&str>) -> tokio::io::Result<Child>
	{
		let mut finalArgs = vec![Option_OutputOnNewLines.clone()];
		finalArgs.append(args);
		
		return Command::new(self.binary.to_owned())
			.kill_on_drop(true)
			.stderr(Stdio::piped())
			.stdout(Stdio::piped())
			.args(finalArgs)
			.spawn();
	}
	
	async fn processOutput(&self, stdout: Option<ChildStdout>, stderr: Option<ChildStderr>,
		formatsHandler: Box<dyn Fn(DownloadFormats) + Send>,
		progressHandler: Box<dyn Fn(DownloadProgress) + Send>,
		stoppedHandler: Box<dyn Fn(DownloadStopped) + Send>,
		subtitlesHandler: Box<dyn Fn(DownloadSubtitles) + Send>,
		titleHandler: Box<dyn Fn(DownloadTitle) + Send>
	)
	{
		match stdout
		{
			Some(so) => {
				let mut reader = FramedRead::new(so, LinesCodec::new());
				while let Some(Ok(line)) = reader.next().await
				{
					debug!("{}", line);
					
					if line.starts_with("[download]")
					{
						if line.ends_with("has already been downloaded")
						{
							let payload = DownloadStopped { forceStop: true, ..Default::default() };
							debug!("{}", payload);
							(stoppedHandler)(payload);
						}
						else if let Some(captures) = self.titleRegex.captures(line.as_str())
						{
							let title = captures.get(1).map_or("", |m| m.as_str());
							let payload = DownloadTitle { title: title.to_string() };
							debug!("{}", payload);
							(titleHandler)(payload);
						}
						else
						{
							let payload = DownloadProgress::from(line.to_owned());
							debug!("{}", payload);
							payload.isValid()
								.then(|| (progressHandler)(payload));
						}
					}
					else if line.starts_with("[info]")
					{
						if let Some(captures) = self.formatRegex.captures(line.as_str())
						{
							let formatString = captures.get(1).map_or("", |m| m.as_str());
							let formats: Vec<&str> = formatString.split("+").collect();
							let mut formatParts = vec![];
							
							for f in formats
							{
								formatParts.push(f.to_owned());
							}
							
							let payload = DownloadFormats { formats: formatParts.to_owned() };
							debug!("{}", payload);
							(formatsHandler)(payload);
						}
						else if let Some(captures) = self.subtitleRegex.captures(line.as_str())
						{
							let subtitlesString = captures.get(1).map_or("", |m| m.as_str());
							let languageFragments: Vec<&str> = subtitlesString.split(", ").collect();
							
							let mut languages = vec![];
							for lang in languageFragments
							{
								languages.push(lang.to_owned());
							}
							
							let payload = DownloadSubtitles { languages: languages.to_owned() };
							debug!("{}", payload);
							(subtitlesHandler)(payload);
						}
					}
				}
				
				let payload = DownloadStopped { stopped: true, ..Default::default() };
				debug!("Download finished?");
				debug!("{}", payload);
				(stoppedHandler)(payload);
			},
			None => warn!("No ChildStdout"),
		};
		
		match stderr
		{
			Some(se) => {
				let mut reader = FramedRead::new(se, LinesCodec::new());
				while let Some(line) = reader.next().await
				{
					match line
					{
						Ok(o) => debug!("{}", o),
						Err(e) => error!("{}", e),
					}
				}
			},
			None => warn!("No ChildStderr"),
		}
	}
}
