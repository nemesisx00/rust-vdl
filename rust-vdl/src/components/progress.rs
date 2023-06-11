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
	let progress = use_ref(cx, || vec![]);
	let title = use_state(cx, || videoUrl.to_owned());
	
	let p = progress.clone();
	let coroutineHandle = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadProgress>| async move
	{
		while let Some(dp) = recv.next().await
		{
			let mut list = p.write();
			
			if !dp.formatParts.is_empty()
			{
				dp.formatParts.iter()
					.for_each(|_| list.push(DownloadProgress::default()));
			}
			
			if !dp.subtitleLanguages.is_empty()
			{
				dp.subtitleLanguages.iter()
					.for_each(|_| list.push(DownloadProgress::default()));
			}
			
			if !list.is_empty()
			{
				if let Some(prog) = list.iter_mut().find(|existing| existing.percentComplete != "100%".to_string())
				{
					match dp.downloadStopped
					{
						//Update the last progress in order to keep the percent/rate/etc. when it displays as complete
						true => prog.downloadStopped = true,
						false => prog.update(dp.to_owned()),
					}
				}
				else
				{
					list.iter_mut()
						.for_each(|prog| prog.downloadStopped = true);
				}
			}
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
	
	if let Some(dpWithTitle) = progress.read().iter().find(|dp| !dp.videoTitle.is_empty())
	{
		title.set(dpWithTitle.videoTitle.to_owned());
	}
	
	let downloadHasStopped = progress.read().iter().all(|dp| dp.downloadStopped == true);
	
	let btnString = match displayRemove.get()
	{
		true => "Start",
		false => "Halt",
	};
	
	let haltResumeClass = match displayRemove.get()
	{
		true => "haltResumeButton",
		false => "haltResumeButton centerMe",
	};
	
	let removeClass = match downloadHasStopped
	{
		true => "removeButton centerMe",
		false => "removeButton"
	};
	
	let shouldDisplayRemove = **displayRemove || downloadHasStopped;
	
	return cx.render(rsx!
	{
		div
		{
			class: "download",
			
			h4 { "{title}" }
			
			for (i, dp) in progress.read().iter().enumerate()
			{
				rsx!
				{
					DownloadProgressBar { key: "{i}", progress: dp.to_owned() }
				}
			}
			
			div
			{
				class: "buttonRow",
				
				(!downloadHasStopped).then(|| rsx!
				{
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
				})
				
				shouldDisplayRemove.then(|| rsx!
				{
					button
					{
						class: "{removeClass}",
						
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
