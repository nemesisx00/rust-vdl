#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_init_atom_root, use_read};
use crate::{
	components::{SimpleInput, Options},
	download::VideoDownloader,
	state::{Binary, FormatSearch, FormatTemplate, OutputDirectory, OutputTemplate}
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
	
	return cx.render(rsx!
	{
		button { id: "showOptions", onclick: move |_| showOptions.set(!showOptions), "Opt" }
		
		if **showOptions
		{
			rsx!(Options {})
		}
		
		div
		{
			class: "app",
			
			SimpleInput { label: "Video".into(), name: "video".into(), value: videoUrl.to_string(), onInput: move |evt: FormEvent| videoUrl.set(evt.value.to_owned()) }
			
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
