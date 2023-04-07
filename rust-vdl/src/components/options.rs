#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_read, use_set};
use crate::{
	components::input::SimpleInput,
	state::{Binary, FormatSearch, FormatTemplate, OutputDirectory, OutputTemplate}
};

pub fn Options(cx: Scope) -> Element
{
	let binary = use_read(cx, Binary);
	let formatSearch = use_read(cx, FormatSearch);
	let formatTemplate = use_read(cx, FormatTemplate);
	let outputDir = use_read(cx, OutputDirectory);
	let outputTemplate = use_read(cx, OutputTemplate);
	
	let setBinary = use_set(cx, Binary);
	let setFormatSearch = use_set(cx, FormatSearch);
	let setFormatTemplate = use_set(cx, FormatTemplate);
	let setOutputDir = use_set(cx, OutputDirectory);
	let setOutputTemplate = use_set(cx, OutputTemplate);
	
	return cx.render(rsx!
	{
		div
		{
			class: "optionsOverlay",
			
			div
			{
				class: "options",
				
				h1 { "Options" }
				
				SimpleInput { label: "Binary".into(), name: "binary".into(), value: binary.into(), onInput: move |evt: FormEvent| setBinary(evt.value.to_owned()) }
				SimpleInput { label: "Format Search".into(), name: "format".into(), value: formatSearch.into(), onInput: move |evt: FormEvent| setFormatSearch(evt.value.to_owned()) }
				SimpleInput { label: "Format Template".into(), name: "ftemplate".into(), value: formatTemplate.into(), onInput: move |evt: FormEvent| setFormatTemplate(evt.value.to_owned()) }
				SimpleInput { label: "Output".into(), name: "output".into(), value: outputDir.into(), onInput: move |evt: FormEvent| setOutputDir(evt.value.to_owned()) }
				SimpleInput { label: "Output Template".into(), name: "otemplate".into(), value: outputTemplate.into(), onInput: move |evt: FormEvent| setOutputTemplate(evt.value.to_owned()) }
			}
		}
	});
}
