#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::BTreeMap;
use std::process::{Child, Command, ChildStdout, Stdio};
use std::io::{self, BufRead, BufReader};
use crate::download::template::TemplateBuilder;

const DefaultFormatTemplate: &'static str = "res:480,vcodec:av1,acodec:opus";
const DefaultOutputTemplate: &'static str = "%(release_timestamp)s-%(title)s.%(ext)s";

pub struct VideoDownloader
{
	binary: String,
	outputDirectory: String,
	outputTemplate: TemplateBuilder,
	processes: BTreeMap<String, Child>,
}

impl VideoDownloader
{
	pub fn new(binary: &'static str, outDir: &'static str) -> Self
	{
		return Self
		{
			binary: binary.into(),
			outputDirectory: outDir.into(),
			outputTemplate: TemplateBuilder::new(DefaultOutputTemplate.to_owned()),
			processes: BTreeMap::<String, Child>::new(),
		};
	}
	
	pub fn setBinary(&mut self, binary: &'static str)
	{
		self.binary = binary.into();
	}
	
	pub fn cancel(&mut self, video: &str)
	{
		if let Some(_pair) = self.processes.get(video)
		{
			//Fuck that process up
		}
	}
	
	pub fn download(&mut self, video: &str)
	{
		let proc = self.spawnCommand(vec![
			"-S", DefaultFormatTemplate,
			"-f", "bv*+ba",
			"-P", self.outputDirectory.as_str(),
			"-o", self.outputTemplate.get().as_str(),
			video
		]);
		
		match proc
		{
			Ok(mut child) => self.writeOutput(child.stdout.take().unwrap()),
			Err(e) => println!("Error downloading video: {} -> {}", video, e)
		};
	}
	
	pub fn listFormats(&mut self, video: &str)
	{
		let proc = self.spawnCommand(vec!["-F", video]);
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
