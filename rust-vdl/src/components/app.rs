#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use crate::{
	constants::{DefaultBinary, DefaultFormatTemplate, DefaultOutputDirectory, DefaultOutputTemplate},
	download::VideoDownloader
};

#[inline_props]
fn SimpleInput<'a>(cx: Scope,
	label: String, name: String, value: String,
	onInput: EventHandler<'a, FormEvent>
) -> Element<'a>
{
	return cx.render(rsx!
	{
		div
		{
			class: "row",
			label { r#for: "{name}", "{label}:" }
			input { r#type: "text", name: "{name}", value: "{value}", oninput: move |evt| onInput.call(evt) }
		}
	});
}

pub fn App(cx: Scope) -> Element
{
	let binary = use_state(cx, || DefaultBinary.to_string());
	let outputDir = use_state(cx, || DefaultOutputDirectory.to_string());
	let outputTemplate = use_state(cx, || DefaultOutputTemplate.to_string());
	let formatTemplate = use_state(cx, || DefaultFormatTemplate.to_string());
	let videoUrl = use_state(cx, || String::default());
	
	let mut vdl1 = VideoDownloader::new(binary.to_string(), outputDir.to_string());
	vdl1.outputTemplate.set(outputTemplate.to_string());
	vdl1.formatTemplate = formatTemplate.to_string();
	let vdl2 = vdl1.clone();
	
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
					let mut vdl = vdl1.clone();
					cx.spawn(async {
						let _ = tokio::task::spawn(async move {
							vdl.listFormats(url.to_string());
						}).await;
					})
				},
				"List Formats"
			}
			
			button
			{
				onclick: move |_| {
					let url = videoUrl.to_string();
					let mut vdl = vdl2.clone();
					cx.spawn(async {
						let _ = tokio::task::spawn(async move {
							vdl.download(url.to_string());
						}).await;
					})
				},
				"Download"
			}
		}
	});
}
