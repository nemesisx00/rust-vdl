# rust-vdl

 A desktop GUI frontend for [yt-dlp](https://github.com/yt-dlp/yt-dlp) written
 in [Rust](https://www.rust-lang.org/) using [Dioxus](https://dioxuslabs.com/).

## Installation

This application does not come packaged with [yt-dlp](https://github.com/yt-dlp/yt-dlp) so you will need to install it yourself. You can find everything you need here: [https://github.com/yt-dlp/yt-dlp](https://github.com/yt-dlp/yt-dlp#installation)

&nbsp;

## Getting Started

### Pre-requisites

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Dioxus CLI](https://github.com/DioxusLabs/cli)

### Compiling

Once you have the pre-requisites installed and ready to go, compiling rust-vdl is very simple. Navigate to the `rust-vdl` directory and run the following command:

```
dioxus build --release
```

The compiled project will be placed in the newly created `rust-vdl/dist` directory.
