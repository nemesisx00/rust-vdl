#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::use_read;
use futures::StreamExt;
use crate::{
	download::{DownloadProgress, VideoDownloader},
	state::{Binary, FormatSearch, FormatTemplate, OutputDirectory, OutputTemplate},
};

#[inline_props]
pub fn DownloadElement(cx: Scope, videoUrl: String) -> Element
{
	let progress = use_state(cx, || DownloadProgress::default());
	
	let p = progress.clone();
	let cr = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadProgress>| async move
	{
		while let Some(dp) = recv.next().await
		{
			p.set(dp);
		}
	});
	
	startDownloader(cx, ||
	{
		let binary = use_read(cx, Binary);
		let formatSearch = use_read(cx, FormatSearch);
		let formatTemplate = use_read(cx, FormatTemplate);
		let outputDir = use_read(cx, OutputDirectory);
		let outputTemplate = use_read(cx, OutputTemplate);
		
		to_owned![videoUrl, binary, formatSearch, formatTemplate, outputDir, outputTemplate, cr];
		tokio::task::spawn(async
		{
			let vdl = generateDownloader(
				binary, formatTemplate, formatSearch, outputDir, outputTemplate,
				move |dp| cr.send(dp)
			);
			vdl.download(videoUrl.into());
		});
	});
	
	return cx.render(rsx!
	{
		div
		{
			class: "download",
			
			h1 { "{videoUrl}" }
			DownloadProgressBar { progress: progress.get().to_owned() }
		}
	});
}

#[inline_props]
fn DownloadProgressBar(cx: Scope, progress: DownloadProgress) -> Element
{
	return cx.render(rsx!
	{
		div
		{
			class: "row",
			
			h1 { "{progress.percentComplete}" }
			h3 { "{progress}" }
		}
	});
}

fn generateDownloader(binary: String, formatTemplate: String, formatSearch: String, outputDirectory: String, outputTemplate: String,
	handler: impl Fn(DownloadProgress) + 'static
) -> VideoDownloader
{
	let mut vdl = VideoDownloader::new(binary.into(), outputDirectory.into());
	vdl.outputTemplate.set(outputTemplate.into());
	vdl.formatTemplate = formatTemplate.into();
	vdl.formatSearch = formatSearch.into();
	vdl.onProgressUpdate = Box::new(handler);
	return vdl;
}

/// Hook to call a function only once within the given scope.
fn startDownloader<'a>(cx: Scope<'a, DownloadElementProps>, f: impl FnOnce())
{
	let run = cx.use_hook(|| true);
	if *run
	{
		f();
		*run = false;
	}
}
