#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_atom_ref, use_read, use_set};
use crate::{
	components::input::{LabelInputRow, ToggleRow},
	state::{saveOptions, Binary, DownloaderOptions},
};

pub fn Options(cx: Scope) -> Element
{
	let binary = use_read(cx, Binary);
	let setBinary = use_set(cx, Binary);
	let downloaderOptions = use_atom_ref(cx, DownloaderOptions);
	
	return cx.render(rsx!
	{
		div
		{
			class: "optionsOverlay",
			
			div
			{
				class: "options",
				
				h1 { "Options" }
				
				LabelInputRow
				{
					label: "Binary".into(),
					name: "binary".into(),
					value: binary.into(),
					onInput: move |evt: FormEvent| {
						setBinary(evt.value.to_owned());
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Age Limit".into(),
					name: "ageLimit".into(),
					value: downloaderOptions.read().ageLimit.to_string(),
					onInput: move |evt: FormEvent| {
						if let Ok(val) = evt.value.parse::<i64>()
						{
							downloaderOptions.write().ageLimit = val.to_owned();
							saveOptions(cx);
						}
					}
				}
				
				LabelInputRow
				{
					label: "Convert Subtitles".into(),
					name: "convertSubs".into(),
					value: downloaderOptions.read().convertSubs.to_string(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().convertSubs = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Convert Thumbnails".into(),
					name: "convertThumbnails".into(),
					value: downloaderOptions.read().convertThumbnails.to_string(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().convertThumbnails = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				ToggleRow
				{
					label: "Download Playlist".into(),
					name: "downloadPlaylist".into(),
					value: downloaderOptions.read().downloadPlaylist.to_owned(),
					onInput: move |evt: FormEvent| {
						if let Ok(val) = evt.value.parse::<bool>()
						{
							downloaderOptions.write().downloadPlaylist = val;
							saveOptions(cx);
						}
					}
				}
				
				ToggleRow
				{
					label: "Embed Metadata".into(),
					name: "embedMetadata".into(),
					value: downloaderOptions.read().embedMetadata.to_owned(),
					onInput: move |evt: FormEvent| {
						if let Ok(val) = evt.value.parse::<bool>()
						{
							downloaderOptions.write().embedMetadata = val;
							saveOptions(cx);
						}
					}
				}
				
				LabelInputRow
				{
					label: "ffmpeg Location".into(),
					name: "ffmpegLocation".into(),
					value: downloaderOptions.read().ffmpegLocation.to_string(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().ffmpegLocation = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Format".into(),
					name: "format".into(),
					value: downloaderOptions.read().format.to_owned(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().format = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Format Sort".into(),
					name: "formatSort".into(),
					value: downloaderOptions.read().formatSort.to_owned(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().formatSort = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Limit Rate".into(),
					name: "limitRate".into(),
					value: downloaderOptions.read().limitRate.to_string(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().limitRate = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Output".into(),
					name: "output".into(),
					value: downloaderOptions.read().output.to_owned(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().output = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Output Path".into(),
					name: "outputPath".into(),
					value: downloaderOptions.read().outputPath.to_owned(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().outputPath = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				ToggleRow
				{
					label: "Prefer Free Formats".into(),
					name: "preferFreeFormats".into(),
					value: downloaderOptions.read().preferFreeFormats.to_owned(),
					onInput: move |evt: FormEvent| {
						if let Ok(val) = evt.value.parse::<bool>()
						{
							downloaderOptions.write().preferFreeFormats = val;
							saveOptions(cx);
						}
					}
				}
				
				LabelInputRow
				{
					label: "Subtitle Format".into(),
					name: "subFormat".into(),
					value: downloaderOptions.read().subFormat.to_string(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().subFormat = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Subtitle Languages".into(),
					name: "subLangs".into(),
					value: downloaderOptions.read().subLangs.to_string(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().subLangs = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				LabelInputRow
				{
					label: "Username".into(),
					name: "username".into(),
					value: downloaderOptions.read().username.to_string(),
					onInput: move |evt: FormEvent| {
						downloaderOptions.write().username = evt.value.to_owned();
						saveOptions(cx);
					}
				}
				
				ToggleRow
				{
					label: "Write Auto Subtitles".into(),
					name: "writeAutoSubs".into(),
					value: downloaderOptions.read().writeAutoSubs.to_owned(),
					onInput: move |evt: FormEvent| {
						if let Ok(val) = evt.value.parse::<bool>()
						{
							downloaderOptions.write().writeAutoSubs = val;
							saveOptions(cx);
						}
					}
				}
				
				ToggleRow
				{
					label: "Write Subtitles".into(),
					name: "writeSubs".into(),
					value: downloaderOptions.read().writeSubs.to_owned(),
					onInput: move |evt: FormEvent| {
						if let Ok(val) = evt.value.parse::<bool>()
						{
							downloaderOptions.write().writeSubs = val;
							saveOptions(cx);
						}
					}
				}
				
				ToggleRow
				{
					label: "Write Thumbnail".into(),
					name: "writeThumbnail".into(),
					value: downloaderOptions.read().writeThumbnail.to_owned(),
					onInput: move |evt: FormEvent| {
						if let Ok(val) = evt.value.parse::<bool>()
						{
							downloaderOptions.write().writeThumbnail = val;
							saveOptions(cx);
						}
					}
				}
			}
		}
	});
}
