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
	onValueChanged: EventHandler<'a, FormEvent>
) -> Element<'a>
{
	return cx.render(rsx!
	{
		div
		{
			class: "row",
			label { r#for: "{name}", "{label}:" }
			input { r#type: "text", name: "{name}", value: "{value}", oninput: move |evt| onValueChanged.call(evt) }
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
	
	let mut vdl1 = VideoDownloader::new(binary.get().into(), outputDir.get().into());
	let mut vdl2 = VideoDownloader::new(binary.get().into(), outputDir.get().into());
	vdl2.outputTemplate.set(outputTemplate.get().into());
	
	return cx.render(rsx!
	{
		div
		{
			class: "form",
			
			SimpleInput { label: "Binary".into(), name: "binary".into(), value: binary.get().into(), onValueChanged: move |evt: FormEvent| binary.set(evt.value.to_owned()) }
			SimpleInput { label: "Output".into(), name: "output".into(), value: outputDir.get().into(), onValueChanged: move |evt: FormEvent| outputDir.set(evt.value.to_owned()) }
			SimpleInput { label: "Output Template".into(), name: "otemplate".into(), value: outputTemplate.get().into(), onValueChanged: move |evt: FormEvent| outputTemplate.set(evt.value.to_owned()) }
			SimpleInput { label: "Format Template".into(), name: "ftemplate".into(), value: formatTemplate.get().into(), onValueChanged: move |evt: FormEvent| formatTemplate.set(evt.value.to_owned()) }
			SimpleInput { label: "Video".into(), name: "video".into(), value: videoUrl.get().into(), onValueChanged: move |evt: FormEvent| videoUrl.set(evt.value.to_owned()) }
			
			button
			{
				onclick: move |_| vdl1.listFormats(videoUrl.get().into()),
				"List Formats"
			}
			
			button
			{
				onclick: move |_| vdl2.download(videoUrl.get().into()),
				"Download"
			}
		}
	});
}
