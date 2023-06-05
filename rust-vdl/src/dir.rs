#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use directories::{ProjectDirs, UserDirs};
use std::fs::create_dir_all;

pub fn getConfigDir(create: bool) -> Option<String>
{
	let mut path = None;
	if let Some(dirs) = ProjectDirs::from("", "", "rust-vdl")
	{
		let pathStr = dirs.config_dir().to_str().unwrap().to_string();
		if create
		{
			let _ = create_dir_all(pathStr.clone());
		}
		path = Some(pathStr);
	}
	
	return path;
}

pub fn getOptionsPath(create: bool) -> Option<String>
{
	return match getConfigDir(create)
	{
		Some(path) => Some(path.clone() + "\\options.json"),
		None => None,
	};
}

pub fn getUserDownloadsDir() -> String
{
	let mut defaultOutput = String::default();
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
