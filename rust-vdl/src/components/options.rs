#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_read, use_set};
use crate::{
	components::input::LabelInputRow,
	state::{saveOptions, Binary, FormatSort, FormatTemplate, OutputDirectory, OutputTemplate}
};

pub fn Options(cx: Scope) -> Element
{
	let binary = use_read(cx, Binary);
	let formatSort = use_read(cx, FormatSort);
	let formatTemplate = use_read(cx, FormatTemplate);
	let outputDir = use_read(cx, OutputDirectory);
	let outputTemplate = use_read(cx, OutputTemplate);
	
	let setBinary = use_set(cx, Binary);
	let setFormatSort = use_set(cx, FormatSort);
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
				
				LabelInputRow { label: "Binary".into(), name: "binary".into(), value: binary.into(), onInput: move |evt: FormEvent| { setBinary(evt.value.to_owned()); saveOptions(cx); } }
				LabelInputRow { label: "Format Sort".into(), name: "format".into(), value: formatSort.into(), onInput: move |evt: FormEvent| { setFormatSort(evt.value.to_owned()); saveOptions(cx); } }
				LabelInputRow { label: "Format Template".into(), name: "ftemplate".into(), value: formatTemplate.into(), onInput: move |evt: FormEvent| { setFormatTemplate(evt.value.to_owned()); saveOptions(cx); } }
				LabelInputRow { label: "Output".into(), name: "output".into(), value: outputDir.into(), onInput: move |evt: FormEvent| { setOutputDir(evt.value.to_owned()); saveOptions(cx); } }
				LabelInputRow { label: "Output Template".into(), name: "otemplate".into(), value: outputTemplate.into(), onInput: move |evt: FormEvent| { setOutputTemplate(evt.value.to_owned()); saveOptions(cx); } }
			}
		}
	});
}
