#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

pub const AppTitle: &'static str = "Rust Video Downloader";
pub const FileMenuLabel: &'static str = "&File";

pub const DefaultBinary: &'static str = "yt-dlp";
pub const DefaultOutputDirectory: &'static str = ".";

pub const DefaultFormatTemplate: &'static str = "res:480,vcodec:av1,acodec:opus";
pub const DefaultOutputTemplate: &'static str = "%(release_timestamp)s-%(title)s.%(ext)s";
