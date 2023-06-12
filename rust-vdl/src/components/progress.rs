#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_atom_ref, use_read};
use futures::StreamExt;
use crate::download::{DownloadFormats, DownloadProgress, DownloadStopped, DownloadSubtitles, DownloadTitle, VideoDownloader};
use crate::state::{Binary, DownloaderOptions, UrlList};

#[inline_props]
pub fn DownloadElement(cx: Scope, indexKey: usize, videoUrl: String) -> Element
{
	let binary = use_read(cx, Binary);
	let downloaderOptions = use_atom_ref(cx, DownloaderOptions);
	let urlList = use_atom_ref(cx, UrlList);
	
	let displayRemove = use_state(cx, || false);
	let downloadFormats = use_ref(cx, || vec![]);
	let downloadProcess = use_state(cx, || None);
	let downloadSubtitles = use_ref(cx, || vec![]);
	let downloadStopped = use_state(cx, || false);
	let progress = use_ref(cx, || vec![]);
	let title = use_state(cx, || videoUrl.to_owned());
	
	let vt = title.clone();
	let vu = videoUrl.clone();
	let titleCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadTitle>| async move
	{
		while let Some(instance) = recv.next().await
		{
			if vt == vu && !instance.title.is_empty()
			{
				vt.set(instance.title.to_owned());
			}
		}
	});
	
	let df = downloadFormats.clone();
	let formatsCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadFormats>| async move
	{
		while let Some(instance) = recv.next().await
		{
			if df.read().is_empty()
			{
				df.with_mut(|list| list.append(&mut instance.formats.to_owned()));
			}
		}
	});
	
	let ds = downloadSubtitles.clone();
	let dopts = downloaderOptions.clone();
	let subtitlesCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadSubtitles>| async move
	{
		while let Some(instance) = recv.next().await
		{
			if dopts.read().writeSubs && ds.read().is_empty()
			{
				ds.with_mut(|list| list.append(&mut instance.languages.to_owned()));
			}
		}
	});
	
	let dpr = progress.clone();
	let df2 = downloadFormats.clone();
	let ds2 = downloadSubtitles.clone();
	let progressCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadProgress>| async move
	{
		while let Some(instance) = recv.next().await
		{
			if dpr.read().is_empty()
			{
				let addCount = df2.read().len() + ds2.read().len();
				if addCount > 0
				{
					let mut list = dpr.write();
					(0..addCount).for_each(|_| list.push(DownloadProgress { ..Default::default() }));
				}
			}
			
			if !dpr.read().is_empty()
			{
				if let Some(prog) = dpr.write().iter_mut().find(|existing| existing.percentComplete != "100%".to_string())
				{
					prog.update(instance.to_owned())
				}
			}
		}
	});
	
	let dsp = progress.clone();
	let dst = downloadStopped.clone();
	let stoppedCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadStopped>| async move
	{
		while let Some(instance) = recv.next().await
		{
			if instance.forceStop
			{
				dsp.write()
					.iter_mut()
					.for_each(|dp| dp.percentComplete = "100%".to_owned());
				
				dst.set(true);
			}
			else if instance.stopped
			{
				dst.set(true);
			}
		}
	});
	
	startDownloader(cx, ||
	{
		to_owned![binary, videoUrl, formatsCoroutine, subtitlesCoroutine, progressCoroutine, stoppedCoroutine, titleCoroutine];
		let dlopts = downloaderOptions.read().clone();
		let handle = tokio::task::spawn(async move {
			let mut vdl = VideoDownloader::new(binary.into(), dlopts.to_owned());
			vdl.download(videoUrl.into(),
				Box::new(move |df| formatsCoroutine.send(df)),
				Box::new(move |dp| progressCoroutine.send(dp)),
				Box::new(move |ds| stoppedCoroutine.send(ds)),
				Box::new(move |ds| subtitlesCoroutine.send(ds)),
				Box::new(move |dt| titleCoroutine.send(dt))
			).await;
		});
		
		downloadProcess.set(Some(handle));
	});
	
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
	
	let removeClass = match downloadStopped.get()
	{
		true => "removeButton centerMe",
		false => "removeButton"
	};
	
	let shouldDisplayRemove = **displayRemove || *downloadStopped.get();
	
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
				
				(!downloadStopped).then(|| rsx!
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
									to_owned![binary, videoUrl, formatsCoroutine, subtitlesCoroutine, progressCoroutine, stoppedCoroutine, titleCoroutine];
									let dlopts2 = downloaderOptions.read().clone();
									let handle = tokio::task::spawn(async move {
										let mut vdl = VideoDownloader::new(binary.into(), dlopts2.to_owned());
										vdl.download(videoUrl.into(),
											Box::new(move |df| formatsCoroutine.send(df)),
											Box::new(move |dp| progressCoroutine.send(dp)),
											Box::new(move |ds| stoppedCoroutine.send(ds)),
											Box::new(move |ds| subtitlesCoroutine.send(ds)),
											Box::new(move |dt| titleCoroutine.send(dt))
										).await;
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
