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
		]);
		
		match proc
		{
			Ok(mut child) => self.writeOutput(child.stdout.take(), child.stderr.take()),
			Err(e) => println!("Error downloading video: {} -> {}", video, e)
		};
	}
	
	pub fn listFormats(&self, video: String)
	{
		let proc = self.spawnCommand(vec!["-F", video.as_str()]);
		
		match proc
		{
			Ok(mut child) => self.writeOutput(child.stdout.take(), child.stderr.take()),
			Err(e) => println!("Error getting list of available formats for video: {} -> {}", video, e)
		};
	}
	
	fn spawnCommand(&self, args: Vec<&str>) -> io::Result<Child>
	{
		return Command::new(self.binary.to_owned())
			.stderr(Stdio::piped())
			.stdout(Stdio::piped())
			.args(args)
			.spawn();
	}
	
	fn writeOutput(&self, stdout: Option<ChildStdout>, stderr: Option<ChildStderr>)
	{
		match stdout
		{
			Some(so) => {
				let lines = BufReader::new(so).lines();
				for line in lines
				{
					match line
					{
						Ok(s) =>  println!("{}", s),
						Err(e) => println!("{}", e),
					}
				}
			},
			None => println!("No ChildStdout"),
		};
		
		match stderr
		{
			Some(se) => {
				let lines = BufReader::new(se).lines();
				for line in lines
				{
					match line
					{
						Ok(s) =>  println!("{}", s),
						Err(e) => println!("{}", e),
					}
				}
			},
			None => println!("No ChildStderr"),
		}
	}
}
