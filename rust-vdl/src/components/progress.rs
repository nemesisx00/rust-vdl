#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_atom_ref, use_read};
use futures::StreamExt;
use crate::{
	download::{DownloadProgress, VideoDownloader},
	state::{Binary, DownloaderOptions},
};

#[inline_props]
pub fn DownloadElement(cx: Scope, videoUrl: String) -> Element
{
	let binary = use_read(cx, Binary);
	let downloaderOptions = use_atom_ref(cx, DownloaderOptions);
	
	let downloadProcess = use_state(cx, || None);
	let progress = use_state(cx, || DownloadProgress::default());
	
	let p = progress.clone();
	let coroutineHandle = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadProgress>| async move
	{
		while let Some(dp) = recv.next().await
		{
			p.set(dp);
		}
	});
	
	startDownloader(cx, ||
	{
		to_owned![binary, videoUrl, coroutineHandle];
		let dlopts = downloaderOptions.read().clone();
		let handle = tokio::task::spawn(async move {
			let mut vdl = VideoDownloader::new(binary.into(), dlopts.to_owned());
			vdl.download(videoUrl.into(), Box::new(move |dp| coroutineHandle.send(dp))).await;
		});
		
		downloadProcess.set(Some(handle));
	});
	
	let btnString = match downloadProcess.is_some()
	{
		true => "Halt",
		false => "Start",
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "download",
			
			h3 { "{videoUrl}" }
			DownloadProgressBar { progress: progress.get().to_owned() }
			button
			{
				class: "haltResumeButton",
				onclick: move |_| {
					match downloadProcess.get()
					{
						Some(handle) => {
							handle.abort();
							downloadProcess.set(None);
						},
						None => {
							to_owned![videoUrl, binary, coroutineHandle];
							let dlopts2 = downloaderOptions.read().clone();
							let handle = tokio::task::spawn(async move {
								let mut vdl = VideoDownloader::new(binary.into(), dlopts2.to_owned());
								vdl.download(videoUrl.into(), Box::new(move |dp| coroutineHandle.send(dp))).await;
							});
							
							downloadProcess.set(Some(handle));
						},
					};
				},
				
				"{btnString}"
			}
		}
	});
}

// --------------------------------------------------

#[inline_props]
fn DownloadProgressBar(cx: Scope, progress: DownloadProgress) -> Element
{
	let percent = progress.percentComplete.as_str();
	let percentNumber = match percent.find("%").is_some()
	{
		true => &percent[..percent.len()-1],
		false => "0",
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "progress",
			
			progress { max: 100, value: percentNumber, "{percent}" }
			h4 { "{progress.percentComplete}" }
			
			if !progress.transferRate.is_empty() || !progress.estimatedSize.is_empty() || !progress.estimatedTime.is_empty()
			{
				rsx!
				{
					div
					{
						class: "progressDetails",
						
						h6 { "Transfer Rate: {progress.transferRate}" }
						h6 { "Estimated Size: {progress.estimatedSize}" }
						h6 { "Time Remaining: {progress.estimatedTime}" }
					}
				}
			}
		}
	});
}

// --------------------------------------------------

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
