#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_init_atom_root, use_read};
use crate::{
	components::Options,
	download::VideoDownloader,
	hooks::useOnce,
	state::{loadOptions, Binary, FormatSearch, FormatTemplate, OutputDirectory, OutputTemplate}
};

pub fn App(cx: Scope) -> Element
{
	use_init_atom_root(cx);
	
	let binary = use_read(cx, Binary);
	let formatSearch = use_read(cx, FormatSearch);
	let formatTemplate = use_read(cx, FormatTemplate);
	let outputDir = use_read(cx, OutputDirectory);
	let outputTemplate = use_read(cx, OutputTemplate);
	
	let videoUrl = use_state(cx, || String::default());
	let showOptions = use_state(cx, || false);
	
	useOnce(cx, || loadOptions(cx));
	
	return cx.render(rsx!
	{
		if **showOptions
		{
			rsx!(Options {})
		}
		
		div
		{
			class: "app",
			
			div
			{
				class: "inputRow video",
				input
				{
					r#type: "text",
					placeholder: "Enter the video URL or ID here",
					value: "{videoUrl}",
					oninput: move |evt: FormEvent| videoUrl.set(evt.value.to_owned())
				}
				
				button
				{
					id: "showOptions",
					onclick: move |_| showOptions.set(!showOptions),
					
					img { alt: "Options", src: "./assets/options.png" }
				}
			}
			
			div
			{
				class: "row",
				
				button
				{
					onclick: move |_|
					{
						let url = videoUrl.to_string();
						to_owned![binary, formatSearch, formatTemplate, outputDir, outputTemplate];
						cx.spawn(async {
							let _ = tokio::task::spawn(async {
								let vdl = generateDownloader(binary, formatTemplate, formatSearch, outputDir, outputTemplate);
								vdl.listFormats(url);
							}).await;
						})
					},
					
					"List Formats"
				}
				
				button
				{
					onclick: move |_|
					{
						let url = videoUrl.to_string();
						to_owned![binary, formatSearch, formatTemplate, outputDir, outputTemplate];
						cx.spawn(async {
							let _ = tokio::task::spawn(async {
								let vdl = generateDownloader(binary, formatTemplate, formatSearch, outputDir, outputTemplate);
								vdl.download(url);
							}).await;
						})
					},
					
					"Download"
				}
			}
			
			hr {}
			
			div
			{
				id: "currentDownloads",
			}
		}
	});
}

fn generateDownloader(binary: String, formatTemplate: String, formatSearch: String, outputDirectory: String, outputTemplate: String) -> VideoDownloader
{
	let mut vdl = VideoDownloader::new(binary.into(), outputDirectory.into());
	vdl.outputTemplate.set(outputTemplate.into());
	vdl.formatTemplate = formatTemplate.into();
	vdl.formatSearch = formatSearch.into();
	return vdl;
}
