#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus_desktop::tao::dpi::LogicalSize;

pub const AppTitle: &str = "Rust Video Downloader";
pub const FileMenuLabel: &str = "&File";
pub const Log4rsConfigFileName_Debug: &str = "config/log4rs-debug.yml";
pub const Log4rsConfigFileName_Release: &str = "config/log4rs.yml";
pub const MinimumWindowSize: LogicalSize<u32> = LogicalSize::new(500, 500);

pub const HtmlMain: &str = r#"
<!DOCTYPE html>
<html>
	<head>
		<title>Rust Video Downloader</title>
		<meta name="viewport" content="width=device-width, initial-scale=1.0" />
		<link rel="stylesheet" href="./assets/app.css" />
	</head>
	<body>
		<div id="main"></div>
	</body>
</html>
"#;
