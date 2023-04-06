#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use crate::{
	components::input::SimpleInput,
	constants::{DefaultBinary, DefaultFormatTemplate, DefaultOutputDirectory, DefaultOutputTemplate},
	download::VideoDownloader,
};

pub fn App(cx: Scope) -> Element
{
	let binary = use_state(cx, || DefaultBinary.to_string());
	let outputDir = use_state(cx, || DefaultOutputDirectory.to_string());
	let outputTemplate = use_state(cx, || DefaultOutputTemplate.to_string());
	let formatTemplate = use_state(cx, || DefaultFormatTemplate.to_string());
	let videoUrl = use_state(cx, || String::default());
	
	return cx.render(rsx!
	{
		div
		{
			class: "form",
			
			SimpleInput { label: "Binary".into(), name: "binary".into(), value: binary.to_string(), onInput: move |evt: FormEvent| binary.set(evt.value.to_owned()) }
			SimpleInput { label: "Output".into(), name: "output".into(), value: outputDir.to_string(), onInput: move |evt: FormEvent| outputDir.set(evt.value.to_owned()) }
			SimpleInput { label: "Output Template".into(), name: "otemplate".into(), value: outputTemplate.to_string(), onInput: move |evt: FormEvent| outputTemplate.set(evt.value.to_owned()) }
			SimpleInput { label: "Format Template".into(), name: "ftemplate".into(), value: formatTemplate.to_string(), onInput: move |evt: FormEvent| formatTemplate.set(evt.value.to_owned()) }
			SimpleInput { label: "Video".into(), name: "video".into(), value: videoUrl.to_string(), onInput: move |evt: FormEvent| videoUrl.set(evt.value.to_owned()) }
			
			button
			{
				onclick: move |_| {
					let url = videoUrl.to_string();
					let bin = binary.to_string();
					let dir = outputDir.to_string();
					let oTemplate = outputTemplate.to_string();
					let fTemplate = formatTemplate.to_string();
					
					cx.spawn(async {
						let _ = tokio::task::spawn(async move {
							let vdl = generateDownloader(bin, dir, oTemplate, fTemplate);
							vdl.listFormats(url.into());
						}).await;
					})
				},
				
				"List Formats"
			}
			
			button
			{
				onclick: move |_| {
					let url = videoUrl.to_string();
					let bin = binary.to_string();
					let dir = outputDir.to_string();
					let oTemplate = outputTemplate.to_string();
					let fTemplate = formatTemplate.to_string();
					
					cx.spawn(async {
						let _ = tokio::task::spawn(async move {
							let vdl = generateDownloader(bin, dir, oTemplate, fTemplate);
							vdl.download(url.into());
						}).await;
					})
				},
				
				"Download"
			}
		}
	});
}

fn generateDownloader(binary: String, outputDirectory: String, outputTemplate: String, formatTemplate: String) -> VideoDownloader
{
	let mut vdl1 = VideoDownloader::new(binary.into(), outputDirectory.into());
	vdl1.outputTemplate.set(outputTemplate.into());
	vdl1.formatTemplate = formatTemplate.into();
	return vdl1;
}
