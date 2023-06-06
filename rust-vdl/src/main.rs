#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod components;
mod constants;
mod download;
mod dir;
mod hooks;
mod state;

use dioxus_desktop::Config;
use dioxus_desktop::tao::menu::{MenuBar, MenuItem};
use dioxus_desktop::tao::window::WindowBuilder;
use crate::components::App;
use crate::constants::{AppTitle, FileMenuLabel, Log4rsConfigFileName_Debug,
	Log4rsConfigFileName_Release, MinimumWindowSize, HtmlMain};

fn main()
{
	match cfg!(debug_assertions)
	{
		true => log4rs::init_file(Log4rsConfigFileName_Debug.to_owned(), Default::default()).unwrap(),
		false => log4rs::init_file(Log4rsConfigFileName_Release.to_owned(), Default::default()).unwrap(),
	}
	
	dioxus_desktop::launch_cfg(App, mainWindowConfig());
}

fn mainWindowConfig() -> Config
{
	let win = WindowBuilder::default()
		.with_inner_size(MinimumWindowSize)
		.with_menu(mainMenu())
		.with_min_inner_size(MinimumWindowSize)
		.with_title(AppTitle);
	
	return Config::new()
		.with_custom_index(HtmlMain.into())
		.with_window(win);
}

fn mainMenu() -> MenuBar
{
	let mut fileMenu = MenuBar::new();
	fileMenu.add_native_item(MenuItem::Quit);
	
	let mut mainMenu = MenuBar::new();
	mainMenu.add_submenu(FileMenuLabel, true, fileMenu);
	
	return mainMenu;
}
