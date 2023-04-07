#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod components;
mod constants;
mod download;
mod state;

use dioxus_desktop::{
	Config,
	tao::{
		menu::{MenuBar, MenuItem},
		window::WindowBuilder,
	},
};
use crate::{
	components::App,
	constants::{AppTitle, FileMenuLabel, HtmlMain},
};

fn main()
{
	dioxus_desktop::launch_cfg(App, mainWindowConfig());
}

fn mainWindowConfig() -> Config
{
	let win = WindowBuilder::default()
		.with_menu(mainMenu())
		.with_title(AppTitle);
	
	return Config::new()
		.with_custom_index(HtmlMain.into())
		.with_window(win);
}

fn mainMenu() -> MenuBar
{
	let mut fileMenu = MenuBar::new();
	fileMenu.add_native_item(MenuItem::Quit);
	
	let mut menubarMenu = MenuBar::new();
	menubarMenu.add_submenu(FileMenuLabel, true, fileMenu);
	
	return menubarMenu;
}
