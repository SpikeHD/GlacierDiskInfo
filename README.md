<div align="center">
  <h1>GlacierDiskInfo</h1>
  <span>A familiar-looking, customizable SMART disk info tool for Linux</span>

  <div align="center">
    <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/GlacierDiskInfo/build.yml" />
    <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/GlacierDiskInfo/format.yml?label=code quality" />
    <img src="https://img.shields.io/github/repo-size/SpikeHD/GlacierDiskInfo" />
  </div>
</div>

![preview](https://github.com/user-attachments/assets/18515e3b-7d97-4931-aa45-459afb552040)

> [!IMPORTANT]
> This project is in no way associated with the [CrystalDiskInfo](https://github.com/hiyohiyo/CrystalDiskInfo) project.

# Table of Contents
* [Installation](#installation)
* [Theming](theming)
  * [Building Themes](#building-themes)
* [Building](#building)
  * [Requirements](#requirements)
  * [Build Steps](#build-steps)
* [TODO](#todo)
* [Attributions](#attributions)

# Installation

Releases are available via [GitHub Actions](https://github.com/SpikeHD/GlacierDiskInfo/actions/workflows/build.yml) artifacts. Actual release builds will be made eventually.

> [!NOTE]
> Maintaining GlacierDiskInfo in a package repository? [Let me know](https://github.com/SpikeHD/GlacierDiskInfo/issues/new) and I will make a list!

# Theming

To add a theme, click the "Theme" menu item, then click "Add theme". This will take you to your themes folder, where you can put any `*.css` files.
Then, restart and the theme should be somewhere under the "Theme" menu item. Hover it, and click "Apply".

## Building Themes

Themes are built in regular ol' CSS. The easiest way to build themes is with the assistance of devtools, which are only available when developing, so you may want to consider [cloning the project](#building).
You can, of course, also just reference the CSS files themselves, located in the `gui/assets` directory.

Example themes can be found in the `themes` directory. Below is a screenshot of `kurei_kei_unofficial.css`:
![kurei_preview](https://github.com/user-attachments/assets/21675890-108a-4a31-a280-d4a2f894053b)

# Building

## Requirements
* [Rust and Cargo](https://www.rust-lang.org/tools/install)
* `dioxus-cli`
  * This can be installed with `cargo install dioxus-cli`
* `libatasmart-dev`
* Dioxus [prerequisites](https://dioxuslabs.com/learn/0.6/getting_started/#linux)

## Build Steps
1. Clone the repository
2. Run `dx build --package glacierdisk-gui --release`
3. The build will be in `target/dx/glacierdisk-gui/release`

# TODO

- `libglacierdisk`
  - [ ] Publish on [crates.io](https://crates.io)
  - [ ] Fix attribute fetching
  - [ ] Support more unix-like platforms (FreeBSD, OpenBSD, etc.)
    - This would probably entail expanding the `libatasmart` library to support more platforms
  - [ ] Support more data
    - [ ] Transfer mode
    - [ ] HDD Rotation Rate
    - [ ] Interface (eg. SATA, NVME, etc.)
- `glacierdisk-gui`
  - [x] Theming
  - [ ] Move to Dioxus [blitz](https://github.com/DioxusLabs/blitz) (whenever that's out/stable-ish)
- `glacierdisk-cli`
  - [ ] All of it
- `glaciermark-gui`
  - [ ] All of it
- `glaciermark-cli`
  - [ ] All of it

# Attributions

The following image files were sourced from [CrystalDiskInfo](https://github.com/hiyohiyo/CrystalDiskInfo):
* `gui/assets/img/good.ico`
* `gui/assets/img/caution.ico`
* `gui/assets/img/bad.ico`

Colors, design, etc. are also (purposefully) heavily inspired by the CrystalDiskInfo project.
