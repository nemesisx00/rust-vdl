#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use std::collections::BTreeMap;
use crate::{
	components::{DownloadElement, Options},
	hooks::useOnce,
	state::loadOptions,
};

pub fn App(cx: Scope) -> Element
{
	fermi::use_init_atom_root(cx);
	
	let videoUrl = use_state(cx, || String::default());
	let showOptions = use_state(cx, || false);
	let downloadUrls = use_ref(cx, || BTreeMap::<usize, String>::default());
	
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
						if !videoUrl.is_empty()
						{
							let len = downloadUrls.read().len();
							let mut urls = downloadUrls.write();
							urls.values()
								.find(|v| **v == videoUrl.to_string())
								.is_none()
								.then(|| {
									urls.insert(len, videoUrl.to_string());
									videoUrl.set(String::default());
								});
						}
					},
					
					"Download"
				}
			}
			
			hr {}
			
			div
			{
				id: "downloads",
				
				for (key, url) in &*downloadUrls.read()
				{
					DownloadElement { key: "{key}", videoUrl: url.to_owned() }
				}
			}
		}
	});
}
