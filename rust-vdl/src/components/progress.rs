#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use fermi::{use_atom_ref, use_read};
use futures::StreamExt;
use crate::download::{DownloadProgress, DownloadReset, DownloadStopped, DownloadTitle, VideoDownloader};
use crate::state::{Binary, DownloaderOptions, UrlList};

#[inline_props]
pub fn DownloadElement(cx: Scope, indexKey: usize, videoUrl: String) -> Element
{
	let binary = use_read(cx, Binary);
	let downloaderOptions = use_atom_ref(cx, DownloaderOptions);
	let urlList = use_atom_ref(cx, UrlList);
	
	let downloadProcess = use_state(cx, || None);
	let downloadStopped = use_state(cx, || false);
	let playlistCurrent = use_state(cx, || 0 as usize);
	let playlistMax = use_state(cx, || 0 as usize);
	let progressBars = use_ref(cx, || Vec::<(String, DownloadProgress)>::default());
	let shouldReset = use_ref(cx, || false);
	let title = use_state(cx, || videoUrl.to_owned());
	
	let vt = title.clone();
	let titleCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadTitle>| async move
	{
		while let Some(instance) = recv.next().await
		{
			if !instance.title.is_empty() && !vt.eq(&instance.title)
			{
				vt.set(instance.title.to_owned());
			}
		}
	});
	
	let dpr = progressBars.clone();
	let sr1 = shouldReset.clone();
	let progressCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadProgress>| async move
	{
		while let Some(instance) = recv.next().await
		{
			let mut list = dpr.write();
			
			let mut resetFlag = sr1.write();
			if *resetFlag
			{
				list.clear();
				*resetFlag = false;
			}
			
			if let Some((_, prog)) = list.iter_mut().find(|(label, _)| label == &instance.label)
			{
				if prog.percentComplete != "100%"
				{
					prog.update(instance.to_owned());
				}
			}
			else
			{
				list.push((instance.label.to_owned(), instance.to_owned()));
			}
		}
	});
	
	let pc = playlistCurrent.clone();
	let pm = playlistMax.clone();
	let sr2 = shouldReset.clone();
	let resetCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadReset>| async move
	{
		while let Some(instance) = recv.next().await
		{
			*sr2.write() = true;
			pc.set(instance.playlistCurrent);
			pm.set(instance.playlistMax);
		}
	});
	
	let dst = downloadStopped.clone();
	let stoppedCoroutine = use_coroutine(cx, |mut recv: UnboundedReceiver<DownloadStopped>| async move
	{
		while let Some(_) = recv.next().await
		{
			dst.set(true);
		}
	});
	
	startDownloader(cx, ||
	{
		to_owned![binary, videoUrl, progressCoroutine, resetCoroutine, stoppedCoroutine, titleCoroutine];
		let dlopts = downloaderOptions.read().clone();
		let handle = tokio::task::spawn(async move {
			let mut vdl = VideoDownloader::new(binary.into(), dlopts.to_owned());
			vdl.download(videoUrl.into(),
				Box::new(move |dp| progressCoroutine.send(dp)),
				Box::new(move |dr| resetCoroutine.send(dr)),
				Box::new(move |ds| stoppedCoroutine.send(ds)),
				Box::new(move |dt| titleCoroutine.send(dt))
			).await;
		});
		
		downloadProcess.set(Some(handle));
	});
	
	let finished = !progressBars.read().is_empty()
						&& progressBars.read()
							.iter()
							.all(|(_, prog)| prog.percentComplete == "100%");
	
	let btnString = match *downloadStopped.get()
	{
		true => "Start",
		false => "Halt",
	};
	
	let playlistText = match *playlistCurrent.get() > 0 && *playlistMax.get() > 0
	{
		true => format!("[{} of {}]: ", playlistCurrent, playlistMax),
		false => "".to_string(),
	};
	
	let removeClass = match !finished
	{
		true => "removeButton",
		false => "removeButton centerMe",
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "download",
			
			h4 { "{playlistText}{title}" }
			
			for (i, (dpl, dp)) in progressBars.read().iter().enumerate()
			{
				rsx!
				{
					DownloadProgressBar { key: "{i}", label: dpl.to_owned(), progress: dp.to_owned() }
				}
			}
			
			div
			{
				class: "buttonRow",
				
				(!finished).then(|| rsx!
				{
					button
					{
						class: "haltResumeButton",
						
						onclick: move |_| {
							match downloadProcess.get()
							{
								Some(handle) => {
									handle.abort();
									downloadProcess.set(None);
									downloadStopped.set(true);
								},
								None => {
									to_owned![binary, videoUrl, progressCoroutine, resetCoroutine, stoppedCoroutine, titleCoroutine];
									let dlopts2 = downloaderOptions.read().clone();
									let handle = tokio::task::spawn(async move {
										let mut vdl = VideoDownloader::new(binary.into(), dlopts2.to_owned());
										vdl.download(videoUrl.into(),
											Box::new(move |dp| progressCoroutine.send(dp)),
											Box::new(move |dr| resetCoroutine.send(dr)),
											Box::new(move |ds| stoppedCoroutine.send(ds)),
											Box::new(move |dt| titleCoroutine.send(dt))
										).await;
									});
									
									downloadStopped.set(false);
									downloadProcess.set(Some(handle));
								},
							};
						},
						
						"{btnString}"
					}
				})
				
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
			}
		}
	});
}

// --------------------------------------------------

#[inline_props]
fn DownloadProgressBar(cx: Scope, label: String, progress: DownloadProgress) -> Element
{
	let percent = progress.percentComplete.as_str();
	
	let mut percentNumber = match percent.find("%").is_some()
	{
		true => &percent[..percent.len()-1],
		false => "0",
	};
	
	let mut percentDisplay = match progress.percentComplete != "0%"
	{
		true => progress.percentComplete.to_owned(),
		false => "0%".to_string(),
	};
	
	if percent.is_empty()
	{
		percentNumber = "100";
		percentDisplay = "?".to_string();
	}
	
	return cx.render(rsx!
	{
		div
		{
			class: "progress",
			
			div
			{
				class: "barRow",
				
				h5 { "{label}" }
				progress { max: 100, value: percentNumber, "{percentDisplay}" }
				h5 { "{percentDisplay}" }
			}
			
			(!progress.transferRate.is_empty() || !progress.size.is_empty() || !progress.time.is_empty()).then(|| rsx!
			{
				div
					{
						class: "progressDetails",
						
						h6 { "Rate: {progress.transferRate}" }
						h6 { "Size: {progress.size}" }
						h6 { "Time: {progress.time}" }
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
