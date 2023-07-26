#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::{process::Stdio};
use futures::StreamExt;
use log::{debug, error, trace, warn};
use fancy_regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use tokio::process::{Child, Command, ChildStderr, ChildStdout};
use tokio_util::codec::{FramedRead, LinesCodec};
use crate::dir::getUserDownloadsDir;

#[cfg(windows)] extern crate winapi;

const Regex_DownloadPlaylistCount: &str = r"\[download\] Downloading item (\d+) of (\d+)";
const Regex_DownloadTitle: &str = r"\[download\] Destination: (?:.*[\\\/])?(.*)\..{3,4}";
const Regex_InfoFormats: &str = r"\[info\].*: Downloading \d+ format\(s\): (.+)";
const Regex_InfoSubtitles: &str = r"\[info\].*: Downloading subtitles: (.+)";
//const Regex_VideoTitle: &str = r"\[download\] Destination: (?:.*[\\\/])?(.*)(?:\.(.*))(?=\..{3,4})\..{3,4}";

const Separator_PartFormat: &str = "+";
const Separator_Subtitle: &str = ", ";

// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DownloadProgress
{
	pub size: String,
	pub time: String,
	pub fragmentStatus: String,
	pub label: String,
	pub percentComplete: String,
	pub transferRate: String,
}

impl DownloadProgress
{
	pub fn update(&mut self, instance: Self)
	{
		self.size = instance.size.to_owned();
		self.time = instance.time.to_owned();
		self.fragmentStatus = instance.fragmentStatus.to_owned();
		self.label = instance.label.to_owned();
		self.percentComplete = instance.percentComplete.to_owned();
		self.transferRate = instance.transferRate.to_owned();
	}
}

impl std::fmt::Display for DownloadProgress
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let formatted = format!("Progress {}: {} {} {} {} {}", self.label, self.percentComplete, self.transferRate, self.size, self.time, self.fragmentStatus);
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
				w.ends_with("B").then(|| instance.size = w.to_owned());
				w.ends_with("B/s").then(|| instance.transferRate = w.to_owned());
				w.find(":").is_some().then(|| instance.time = w.to_owned());
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DownloadReset
{
	pub label: String,
	pub playlistCurrent: usize,
	pub playlistMax: usize,
}

impl std::fmt::Display for DownloadReset
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let formatted = format!("Resetting download progress for part {} of {} - '{}'", self.playlistCurrent, self.playlistMax, self.label);
		return f.write_str(formatted.as_str());
	}
}

// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DownloadStopped
{
	pub label: String,
}

impl std::fmt::Display for DownloadStopped
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let formatted = format!("Download has stopped: '{}'", self.label);
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
	currentDownloadLabel: String,
	partLabels: Vec<String>,
	playlistCurrent: usize,
	playlistMax: usize,
	
	regexInfoFormats: Regex,
	regexInfoSubtitles: Regex,
	regexDownloadPlaylistCount: Regex,
	regexDownloadTitle: Regex,
}

impl VideoDownloader
{
	pub fn new(binary: String, options: VideoDownloaderOptions) -> Self
	{
		let regexInfoFormats = Regex::new(Regex_InfoFormats).expect("Failed to compile Info Formats regular expression");
		let regexInfoSubtitles = Regex::new(Regex_InfoSubtitles).expect("Failed to compile Info Subtitles regular expression");
		let regexDownloadPlaylistCount = Regex::new(Regex_DownloadPlaylistCount).expect("Failed to compile Download Playlist Count regular expression.");
		let regexDownloadTitle = Regex::new(Regex_DownloadTitle).expect("Failed to compile Download Title regular expression.");
		
		return Self
		{
			binary: binary.into(),
			options,
			child: None,
			currentDownloadLabel: String::default(),
			partLabels: Vec::<String>::default(),
			playlistCurrent: 0,
			playlistMax: 0,
			regexInfoFormats,
			regexInfoSubtitles,
			regexDownloadPlaylistCount,
			regexDownloadTitle,
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
		progressHandler: Box<dyn Fn(DownloadProgress) + Send>,
		resetHandler: Box<dyn Fn(DownloadReset) + Send>,
		stoppedHandler: Box<dyn Fn(DownloadStopped) + Send>,
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
						progressHandler, resetHandler, stoppedHandler, titleHandler
					).await;
					self.child = Some(child);
				},
				Err(e) => error!("Error downloading video: {} -> {}", video, e)
			};
		}
	}
	
	fn parseTitle(&self, fullTitle: String) -> (String, String)
	{
		let mut title = String::default();
		let mut partLabel = String::default();
		
		//Detect which part label is at the end of this full title string
		if let Some(foundLabel) = self.partLabels.iter().find(|label| fullTitle.ends_with(format!("{}", label).as_str()))
		{
			partLabel = foundLabel.to_owned();
			
			//Slice out the actual title, without the part label
			if let Some(endIndex) = fullTitle.find(format!(".{}", partLabel).as_str())
			{
				title = fullTitle[..endIndex].to_string();
			}
			else if let Some(endIndex) = fullTitle.find(format!(".f{}", partLabel).as_str())
			{
				title = fullTitle[..endIndex].to_string();
			}
		}
		
		return (title, partLabel);
	}
	
	async fn processOutput(&mut self, stdout: Option<ChildStdout>, stderr: Option<ChildStderr>,
		progressHandler: Box<dyn Fn(DownloadProgress) + Send>,
		resetHandler: Box<dyn Fn(DownloadReset) + Send>,
		stoppedHandler: Box<dyn Fn(DownloadStopped) + Send>,
		titleHandler: Box<dyn Fn(DownloadTitle) + Send>
	)
	{
		match stdout
		{
			Some(so) => {
				let mut reader = FramedRead::new(so, LinesCodec::new());
				while let Some(opt) = reader.next().await
				{
					match opt
					{
						Ok(line) => {
							trace!("{}", line);
							
							if line.starts_with("[download]")
							{
								if let Ok(Some(captures)) = self.regexDownloadPlaylistCount.captures(line.as_str())
								{
									self.processOutput_playlistCount(captures, &resetHandler);
								}
								else if let Ok(Some(captures)) = self.regexDownloadTitle.captures(line.as_str())
								{
									self.processOutput_title(captures, &titleHandler);
								}
								else
								{
									self.processOutput_downloadProgress(line.to_owned(), &progressHandler);
								}
							}
							else if line.starts_with("[info]")
							{
								if let Ok(Some(captures)) = self.regexInfoFormats.captures(line.as_str())
								{
									self.processOutput_infoFormats(captures);
								}
								else if let Ok(Some(captures)) = self.regexInfoSubtitles.captures(line.as_str())
								{
									self.processOutput_infoSubtitles(captures);
								}
							}
						},
						
						Err(e) => error!("{}", e),
					}
				}
				
				self.processOutput_downloadStopped(&stoppedHandler);
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
						Ok(o) => error!("{}", o),
						Err(e) => error!("{}", e),
					}
				}
			},
			None => warn!("No ChildStderr"),
		}
	}
	
	fn processOutput_downloadProgress(&self, line: String, handler: &Box<dyn Fn(DownloadProgress) + Send>)
	{
		let mut payload = DownloadProgress::from(line.to_owned());
		if !payload.percentComplete.is_empty() || (!payload.size.is_empty() && !payload.time.is_empty() && !payload.transferRate.is_empty())
		{
			payload.label = self.currentDownloadLabel.to_owned();
			debug!("{}", payload);
			(handler)(payload);
		}
	}
	
	fn processOutput_downloadStopped(&self, handler: &Box<dyn Fn(DownloadStopped) + Send>)
	{
		let payload = DownloadStopped { label: self.currentDownloadLabel.to_owned(), ..Default::default() };
		debug!("{}", payload);
		(handler)(payload);
	}
	
	fn processOutput_infoFormats(&mut self, captures: Captures)
	{
		let m = captures.get(1).map_or(String::default(), |m| m.as_str().to_string());
		if !m.is_empty()
		{
			m.split(Separator_PartFormat)
				.for_each(|label| self.partLabels.push(label.to_owned()));
		}
	}
	
	fn processOutput_infoSubtitles(&mut self, captures: Captures)
	{
		let m = captures.get(1).map_or(String::default(), |m| m.as_str().to_string());
		if !m.is_empty()
		{
			m.split(Separator_Subtitle)
				.for_each(|label| self.partLabels.push(label.to_owned()));
		}
	}
	
	fn processOutput_playlistCount(&mut self, captures: Captures, handler: &Box<dyn Fn(DownloadReset) + Send>)
	{
		let firstMatch = captures.get(1).map_or(String::default(), |m| m.as_str().to_string());
		if let Ok(current) = firstMatch.parse::<usize>()
		{
			self.playlistCurrent = current;
		}
		
		let secondMatch = captures.get(2).map_or(String::default(), |m| m.as_str().to_string());
		if let Ok(max) = secondMatch.parse::<usize>()
		{
			self.playlistMax = max;
		}
		
		self.partLabels.clear();
		
		let payload = DownloadReset
		{
			label: self.currentDownloadLabel.to_owned(),
			playlistCurrent: self.playlistCurrent,
			playlistMax: self.playlistMax
		};
		debug!("{}", payload);
		(handler)(payload);
	}
	
	fn processOutput_title(&mut self, captures: Captures, handler: &Box<dyn Fn(DownloadTitle) + Send>)
	{
		let fullTitle = captures.get(1).map_or(String::default(), |m| m.as_str().to_string());
		let (title, partLabel) = self.parseTitle(fullTitle);
		
		self.updateCurrentDownloadLabel(partLabel.to_owned());
		
		let payload = DownloadTitle { title: title.to_owned() };
		debug!("{}", payload);
		(handler)(payload);
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
	
	fn updateCurrentDownloadLabel(&mut self, label: String)
	{
		if !label.is_empty()
		{
			self.currentDownloadLabel = label.to_string();
		}
		else
		{
			if self.playlistCurrent > 0
			{
				self.currentDownloadLabel = self.playlistCurrent.to_string();
			}
			else
			{
				self.currentDownloadLabel = String::default();
			}
		}
	}
}
