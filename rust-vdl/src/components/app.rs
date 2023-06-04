#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::use_atom_ref;
use crate::components::{DownloadElement, Options};
use crate::hooks::useOnce;
use crate::state::{loadOptions, UrlList};

pub fn App(cx: Scope) -> Element
{
	fermi::use_init_atom_root(cx);
	
	let urlList = use_atom_ref(cx, UrlList);
	
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
						if !videoUrl.is_empty()
						{
							let len = urlList.read().len();
							let mut urls = urlList.write();
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
				
				for (key, url) in &*urlList.read()
				{
					DownloadElement { key: "{key}", indexKey: *key, videoUrl: url.to_owned() }
				}
			}
		}
	});
}
