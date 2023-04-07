#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use directories::{ProjectDirs, UserDirs};
use dioxus::prelude::Scope;
use fermi::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};

const DefaultBinary: &'static str = "yt-dlp";
const DefaultFormatSearch: &'static str = "bv*+ba/b";
const DefaultFormatTemplate: &'static str = "vcodec:av1,acodec:opus";
const DefaultOutputDirectory: &'static str = ".";
const DefaultOutputTemplate: &'static str = "%(upload_date)s - %(title)s.%(ext)s";

pub static Binary: Atom<String> = |_| DefaultBinary.to_string();
pub static FormatSearch: Atom<String> = |_| DefaultFormatSearch.to_string();
pub static FormatTemplate: Atom<String> = |_| DefaultFormatTemplate.to_string();
pub static OutputDirectory: Atom<String> = |_| getUserDownloadsDir();
pub static OutputTemplate: Atom<String> = |_| DefaultOutputTemplate.to_string();

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct OptionsData
{
	pub binary: String,
	pub formatSearch: String,
	pub formatTemplate: String,
	pub outputDirectory: String,
	pub outputTemplate: String,
}

pub fn loadOptions(cx: Scope)
{
	//Dioxus hooks need to be accessed in the same order every time, even if you're not using them all.
	let _ = use_read(cx, Binary);
	let _ = use_read(cx, FormatSearch);
	let _ = use_read(cx, FormatTemplate);
	let _ = use_read(cx, OutputDirectory);
	let _ = use_read(cx, OutputTemplate);
	
	let setBinary = use_set(cx, Binary);
	let setFormatSearch = use_set(cx, FormatSearch);
	let setFormatTemplate = use_set(cx, FormatTemplate);
	let setOutputDir = use_set(cx, OutputDirectory);
	let setOutputTemplate = use_set(cx, OutputTemplate);
	
	if let Some(path) = getOptionsPath(false)
	{
		if let Ok(mut file) = File::open(&path)
		{
			let mut json = String::new();
			if let Ok(_) = file.read_to_string(&mut json)
			{
				if let Ok(data) = serde_json::from_str::<OptionsData>(json.as_str())
				{
					setBinary(data.binary);
					setFormatSearch(data.formatSearch);
					setFormatTemplate(data.formatTemplate);
					setOutputDir(data.outputDirectory);
					setOutputTemplate(data.outputTemplate);
					println!("Options loaded!")
				}
			}
		}
	}
}

pub fn saveOptions(cx: Scope)
{
	let binary = use_read(cx, Binary);
	let formatSearch = use_read(cx, FormatSearch);
	let formatTemplate = use_read(cx, FormatTemplate);
	let outputDirectory = use_read(cx, OutputDirectory);
	let outputTemplate = use_read(cx, OutputTemplate);
	
	let data = OptionsData
	{
		binary: binary.into(),
		formatSearch: formatSearch.into(),
		formatTemplate: formatTemplate.into(),
		outputDirectory: outputDirectory.into(),
		outputTemplate: outputTemplate.into(),
	};
	
	if let Some(path) = getOptionsPath(true)
	{
		if let Ok(json) = serde_json::to_string(&data)
		{
			if let Ok(mut file) = File::create(&path)
			{
				match file.write_all(json.as_bytes())
				{
					Ok(_) => println!("Options saved!"),
					Err(e) => println!("{}", e),
				}
			}
		}
	}
}

fn getConfigDir(create: bool) -> Option<String>
{
	let mut path = None;
	if let Some(dirs) = ProjectDirs::from("", "", "rust-vdl")
	{
		let pathStr = dirs.config_dir().to_str().unwrap().to_string();
		if create
		{
			let _ = fs::create_dir_all(pathStr.clone());
		}
		path = Some(pathStr);
	}
	
	return path;
}

fn getOptionsPath(create: bool) -> Option<String>
{
	return match getConfigDir(create)
	{
		Some(path) => Some(path.clone() + "\\options.json"),
		None => None,
	};
}

fn getUserDownloadsDir() -> String
{
	let mut defaultOutput = DefaultOutputDirectory.to_string();
	if let Some(dirs) = UserDirs::new()
	{
		if let Some(dl) = dirs.download_dir()
		{
			if dl.exists() && dl.is_dir()
			{
				defaultOutput = dl.to_str().unwrap().to_string();
			}
		}
	}
	
	return defaultOutput;
}
