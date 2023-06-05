<h1 align="center">rust-vdl</h1>

<p align="center">
	A desktop GUI frontend for <a href="https://github.com/yt-dlp/yt-dlp">yt-dlp</a>
	written in <a href="https://www.rust-lang.org/">Rust</a>, relying on
	<a href="https://dioxuslabs.com/">Dioxus</a> and <a href="https://tokio.rs/">Tokio</a>.
</p>

<div align="center" width="100%">
	<img alt="GitHub" src="https://img.shields.io/github/license/nemesisx00/rust-vdl" />
	<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/nemesisx00/rust-vdl/build.yml" />
</div>

&nbsp;

## Installation

This application does not come packaged with [yt-dlp](https://github.com/yt-dlp/yt-dlp)
so you will need to install it yourself. You can find everything you need here:
[https://github.com/yt-dlp/yt-dlp](https://github.com/yt-dlp/yt-dlp)

&nbsp;

## Development

### Pre-requisites

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Dioxus CLI](https://github.com/DioxusLabs/cli)

### Compiling

Once you have the pre-requisites installed and ready to go, compiling rust-vdl
is very simple. Navigate to the `rust-vdl` directory and run the following command:

```
dioxus build --release
```

The compiled project will be placed in the newly created `rust-vdl/dist` directory.
