#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::Scope;
use fermi::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::download::VideoDownloaderOptions;
use crate::dir::getOptionsPath;

const DefaultBinary: &'static str = "yt-dlp";

pub static Binary: Atom<String> = |_| DefaultBinary.to_string();
pub static DownloaderOptions: AtomRef<VideoDownloaderOptions> = |_| VideoDownloaderOptions::default();
pub static UrlList: AtomRef<BTreeMap<usize, String>> = |_| BTreeMap::<usize, String>::default();

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct OptionsData
{
	pub binary: String,
	pub downloaderOptions: VideoDownloaderOptions,
}

pub fn loadOptions(cx: Scope)
{
	let setBinary = use_set(cx, Binary);
	let downloaderOptions = use_atom_ref(cx, DownloaderOptions);
	
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
					*downloaderOptions.write() = data.downloaderOptions;
					println!("Options loaded!")
				}
			}
		}
	}
}

pub fn saveOptions(cx: Scope)
{
	let binary = use_read(cx, Binary);
	let downloaderOptions = use_atom_ref(cx, DownloaderOptions);
	
	let data = OptionsData
	{
		binary: binary.into(),
		downloaderOptions: downloaderOptions.read().to_owned(),
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
