#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::process::{Child, Command, ChildStderr, ChildStdout, Stdio};
use std::io::{self, BufRead, BufReader};
use crate::{
	download::template::OutputTemplateBuilder,
};

#[derive(Clone, Default)]
pub struct VideoDownloader
{
	pub binary: String,
	pub formatTemplate: String,
	pub formatSearch: String,
	pub outputDirectory: String,
	pub outputTemplate: OutputTemplateBuilder,
}

impl VideoDownloader
{
	pub fn new(binary: String, outDir: String) -> Self
	{
		return Self
		{
			binary: binary.into(),
			outputDirectory: outDir.into(),
			..Default::default()
		};
	}
	
	pub fn download(&self, video: String)
	{
		let proc = self.spawnCommand(vec![
			"-S", self.formatTemplate.as_str(),
			"-f", self.formatSearch.as_str(),
			"-P", self.outputDirectory.as_str(),
			"-o", self.outputTemplate.get().as_str(),
			video.as_str(),
		].as_mut());
		
		match proc
		{
			Ok(mut child) => self.processOutput(child.stdout.take(), child.stderr.take()),
			Err(e) => println!("Error downloading video: {} -> {}", video, e)
		};
	}
	
	pub fn listFormats(&self, video: String)
	{
		let proc = self.spawnCommand(vec!["-F", video.as_str()].as_mut());
		
		match proc
		{
			Ok(mut child) => self.processOutput(child.stdout.take(), child.stderr.take()),
			Err(e) => println!("Error getting list of available formats for video: {} -> {}", video, e)
		};
	}
	
	fn spawnCommand(&self, args: &mut Vec<&str>) -> io::Result<Child>
	{
		let mut finalArgs = vec!["--newline"];
		finalArgs.append(args);
		
		return Command::new(self.binary.to_owned())
			.stderr(Stdio::piped())
			.stdout(Stdio::piped())
			.args(finalArgs)
			.spawn();
	}
	
	fn processOutput(&self, stdout: Option<ChildStdout>, stderr: Option<ChildStderr>)
	{
		match stdout
		{
			Some(so) => {
				
				let reader = BufReader::new(so);
				reader.lines()
					.filter_map(|l| l.ok())
					.for_each(|l| {
						if l.starts_with("[download]")
						{
							let progress = DownloadProgress::from(l.to_owned());
							if progress.isValid()
							{
								println!("{}", progress);
							}
						}
						else
						{
							println!("{}", l);
						}
					});
			},
			None => println!("No ChildStdout"),
		};
		
		match stderr
		{
			Some(se) => {
				let reader = BufReader::new(se);
				reader.lines()
					.for_each(|l| match l {
						Ok(o) => println!("{}", o),
						Err(e) => println!("{}", e),
					});
			},
			None => println!("No ChildStderr"),
		}
	}
}

// --------------------------------------------------

#[derive(Clone, Debug, Default)]
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
	fn isValid(&self) -> bool
	{
		return !self.percentComplete.is_empty();
	}
	
	fn toString(&self) -> String
	{
		return format!("{} {} {} {}", self.transferRate, self.estimatedSize, self.estimatedTime, self.fragmentStatus);
	}
}

impl std::fmt::Display for DownloadProgress
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		return f.write_str(self.toString().as_str());
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
	fn into(self) -> String { return self.toString(); }
}
