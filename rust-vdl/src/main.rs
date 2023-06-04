#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod components;
mod constants;
mod download;
mod hooks;
mod state;

use dioxus_desktop::Config;
use dioxus_desktop::tao::menu::{MenuBar, MenuItem};
use dioxus_desktop::tao::window::WindowBuilder;
use crate::components::App;
use crate::constants::{AppTitle, FileMenuLabel, MinimumWindowSize, HtmlMain};

fn main()
{
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
