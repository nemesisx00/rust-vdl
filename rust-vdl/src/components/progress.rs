#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_atom_ref, use_read};
use futures::StreamExt;
use crate::download::{DownloadProgress, VideoDownloader};
use crate::state::{Binary, DownloaderOptions, UrlList};

#[inline_props]
pub fn DownloadElement(cx: Scope, indexKey: usize, videoUrl: String) -> Element
{
	let binary = use_read(cx, Binary);
	let downloaderOptions = use_atom_ref(cx, DownloaderOptions);
	let urlList = use_atom_ref(cx, UrlList);
	
	let displayRemove = use_state(cx, || false);
	let downloadProcess = use_state(cx, || None);
	let progress = use_state(cx, || DownloadProgress::default());
	let title = use_state(cx, || videoUrl.to_owned());
	
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
	
	(!progress.videoTitle.is_empty())
		.then(|| title.set(progress.videoTitle.to_owned()));
	
	let btnString = match downloadProcess.is_some()
	{
		true => "Halt",
		false => "Start",
	};
	
	let haltResumeClass = match displayRemove.get()
	{
		true => "haltResumeButton",
		false => "haltResumeButton centerMe",
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "download",
			
			h4 { "{title}" }
			DownloadProgressBar { progress: progress.get().to_owned() }
			
			div
			{
				class: "buttonRow",
				
				button
				{
					class: "{haltResumeClass}",
					
					onclick: move |_| {
						match downloadProcess.get()
						{
							Some(handle) => {
								handle.abort();
								downloadProcess.set(None);
								displayRemove.set(true);
							},
							None => {
								to_owned![videoUrl, binary, coroutineHandle];
								let dlopts2 = downloaderOptions.read().clone();
								let handle = tokio::task::spawn(async move {
									let mut vdl = VideoDownloader::new(binary.into(), dlopts2.to_owned());
									vdl.download(videoUrl.into(), Box::new(move |dp| coroutineHandle.send(dp))).await;
								});
								
								displayRemove.set(false);
								downloadProcess.set(Some(handle));
							},
						};
					},
					
					"{btnString}"
				}
				
				displayRemove.then(|| rsx!
				{
					button
					{
						class: "removeButton",
						
						onclick: move |_| {
							if let Some(handle) = downloadProcess.get()
							{
								handle.abort();
								downloadProcess.set(None);
							}
							
							urlList.write().remove_entry(indexKey);
						},
						
						"Remove"
					}
				})
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
	
	let percentDisplay = match progress.percentComplete != "0%"
	{
		true => progress.percentComplete.to_owned(),
		false => "0%".to_string(),
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "progress",
			
			progress { max: 100, value: percentNumber, "{percentDisplay}" }
			h4 { "{percentDisplay}" }
			
			(!progress.transferRate.is_empty() || !progress.estimatedSize.is_empty() || !progress.estimatedTime.is_empty()).then(|| rsx!
			{
				div
					{
						class: "progressDetails",
						
						h6 { "Transfer Rate: {progress.transferRate}" }
						h6 { "Estimated Size: {progress.estimatedSize}" }
						h6 { "Time Remaining: {progress.estimatedTime}" }
					}
			})
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
