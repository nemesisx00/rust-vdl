#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use crate::download::VideoDownloader;

pub fn App(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		h1 { "DESKTOP" }
		button
		{
			onclick: |_| async move {
				doListFormats();
			},
			"List Formats"
		}
	});
}

fn doListFormats()
{
	let mut vdl = VideoDownloader::new("yt-dlp", "V:/Downloads/ytdl");
	let video = "https://youtu.be/n3onSHukoIU";
	vdl.listFormats(video);
	//vdl.download(video);
}
