#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod components;
mod constants;
mod download;

use dioxus_desktop::Config;
use dioxus_desktop::tao::{
	menu::{MenuBar, MenuItem},
	window::WindowBuilder,
};
use crate::{
	components::App,
	constants::{AppTitle, FileMenuLabel},
};

fn main()
{
	dioxus_desktop::launch_cfg(App, buildConfig());
}

fn buildConfig() -> Config
{
	let mut fileMenu = MenuBar::new();
	fileMenu.add_native_item(MenuItem::Quit);
	
	let mut menubarMenu = MenuBar::new();
	menubarMenu.add_submenu(FileMenuLabel, true, fileMenu);
	
	let win = WindowBuilder::default()
		.with_menu(menubarMenu)
		.with_title(AppTitle);
	
	return Config::new()
		.with_window(win);
}
