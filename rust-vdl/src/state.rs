#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use directories::UserDirs;
use fermi::prelude::*;

const DefaultBinary: &'static str = "yt-dlp";
const DefaultFormatTemplate: &'static str = "vcodec:av1,acodec:opus";
const DefaultFormatSearch: &'static str = "bv*+ba/b";
const DefaultOutputDirectory: &'static str = ".";
const DefaultOutputTemplate: &'static str = "%(upload_date)s - %(title)s.%(ext)s";

pub static Binary: Atom<String> = |_| DefaultBinary.to_string();
pub static FormatSearch: Atom<String> = |_| DefaultFormatSearch.to_string();
pub static FormatTemplate: Atom<String> = |_| DefaultFormatTemplate.to_string();
pub static OutputDirectory: Atom<String> = |_| getUserDownloadsDir();
pub static OutputTemplate: Atom<String> = |_| DefaultOutputTemplate.to_string();

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
