<div align="center">
  <h1>GlacierDiskInfo</h1>
  <span>A familiar-looking SMART disk info tool for Linux</span>
</div>

> [!IMPORTANT]
> This project is in no way associated with the [CrystalDiskInfo](https://github.com/hiyohiyo/CrystalDiskInfo) project.

# Table of Contents
* [Installation](#installation)
* [Building](#building)
  * [Requirements](#requirements)
  * [Build Steps](#build-steps)
* [TODO](#todo)
* [Attributions](#attributions)

# Installation

Releases are available via [GitHub Actions](https://github.com/SpikeHD/GlacierDiskInfo/actions/workflows/build.yml) artifacts. Actual release builds will be made eventually.

> [!NOTE]
> Maintaining GlacierDiskInfo in a package repository? [Let me know](https://github.com/SpikeHD/GlacierDiskInfo/issues/new) and I will make a list!

# Building

## Requirements
* [Rust and Cargo](https://www.rust-lang.org/tools/install)
* `dioxus-cli`
  * This can be installed with `cargo install dioxus-cli`
* `libatasmart-dev`

## Build Steps
1. Clone the repository
2. Run `dx build --package glacierdisk-gui --release`
3. The build will be in `target/dx/glacierdisk-gui/release`

# TODO

- `libglacierdisk`
  - [ ] Support more unix-like platforms (FreeBSD, OpenBSD, etc.)
    - This would probably entail expanding the `libatasmart` library to support more platforms
- `glacierdisk-gui`
  - [ ] Theming
  - [ ] Move to Dioxus [blitz](https://github.com/DioxusLabs/blitz) (whenever that's out/stable-ish)
- `glacierdisk-cli`
  - [ ] TUI

# Attributions

The following image files were sourced from [CrystalDiskInfo](https://github.com/hiyohiyo/CrystalDiskInfo):
* `gui/assets/img/good.ico`
* `gui/assets/img/caution.ico`
* `gui/assets/img/bad.ico`

Colors, design, etc. are also (purposefully) heavily inspired by the CrystalDiskInfo project.