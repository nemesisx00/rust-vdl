#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::process::{Child, Command, ChildStdout, Stdio};
use std::io::{self, BufRead, BufReader};
use crate::{
	constants::{DefaultBinary, DefaultFormatTemplate, DefaultOutputDirectory},
	download::template::OutputTemplateBuilder,
};

#[derive(Clone)]
pub struct VideoDownloader
{
	pub formatTemplate: String,
	pub outputTemplate: OutputTemplateBuilder,
	
	binary: String,
	outputDirectory: String,
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
	
	pub fn setBinary(&mut self, binary: String)
	{
		self.binary = binary.into();
	}
	
	pub fn setOutputDirectory(&mut self, outDir: String)
	{
		self.outputDirectory = outDir.into();
	}
	
	pub fn download(&mut self, video: String)
	{
		let proc = self.spawnCommand(vec![
			"-S", self.formatTemplate.as_str(),
			"-f", "bv*+ba",
			"-P", self.outputDirectory.as_str(),
			"-o", self.outputTemplate.get().as_str(),
			video.as_str(),
		]);
		
		match proc
		{
			Ok(mut child) => self.writeOutput(child.stdout.take().unwrap()),
			Err(e) => println!("Error downloading video: {} -> {}", video, e)
		};
	}
	
	pub fn listFormats(&mut self, video: String)
	{
		let proc = self.spawnCommand(vec!["-F", video.as_str()]);
		//self.processes.insert(video.to_owned(), proc.unwrap());
		
		match proc
		{
			Ok(mut child) => self.writeOutput(child.stdout.take().unwrap()),
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
	
	fn writeOutput(&self, stdout: ChildStdout)
	{
		let lines = BufReader::new(stdout).lines();
		for line in lines
		{
			match line
			{
				Ok(s) =>  println!("{}", s),
				Err(e) => println!("{:?}", e),
			}
		}
	}
}

impl Default for VideoDownloader
{
    fn default() -> Self
	{
		return Self
		{
			formatTemplate: DefaultFormatTemplate.into(),
			outputTemplate: Default::default(),
			
			binary: DefaultBinary.into(),
			outputDirectory: DefaultOutputDirectory.into(),
		};
    }
}
