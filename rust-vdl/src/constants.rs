#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

pub const AppTitle: &'static str = "Rust Video Downloader";
pub const FileMenuLabel: &'static str = "&File";

pub const HtmlMain: &'static str = r#"
<!DOCTYPE html>
<html>
	<head>
		<title>Rust Video Downloader</title>
		<meta name="viewport" content="width=device-width, initial-scale=1.0" />
		<link rel="stylesheet" href="/static/app.css" />
	</head>
	<body>
		<div id="main"></div>
	</body>
</html>
"#;
