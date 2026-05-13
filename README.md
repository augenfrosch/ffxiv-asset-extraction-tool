# ffxiv-asset-extraction-tool

A WIP tool for extracting FFXIV assets.

The main goal is to offer a (partial) alternative for the now effectively defunct `SaintCoinach.Cmd`, based on [boilmaster](https://github.com/ackwell/boilmaster) / [ironworks](https://github.com/ackwell/ironworks) which power [XIVAPI v2](https://v2.xivapi.com/).


> [!IMPORTANT]
> The current implementation is very much work in progress. Only the `ui`/`uihd` & `maps` commands from `SaintCoinach.Cmd` have (rough) equivalents. However, it is not meant as a 1:1 replacement, the CLI argument & output folder structure are not compatible.

For Excel data / Game sheet data exports in CSV format, refer to [XIVData Oxidizer](https://github.com/skyborn-industries/xiv-data-oxidizer).

## Download

See the [latest release](https://github.com/augenfrosch/ffxiv-asset-extraction-tool/releases/latest). The help text can be shown using the `--help` argument, and should give a good enough overview of the available options.

## Building from source

The [Rust](https://rust-lang.org/) toolchain is required to build the tool. The minimum-supported rust version (MSRV) is `1.95.0`. TODO:!! Set in `Cargo.toml`s

1. Clone the repository including the submodule:
```
git clone --recurse-submodules https://github.com/augenfrosch/ffxiv-asset-extraction-tool.git
cd ffxiv-asset-extraction-tool
```
2. build and run the CLI:
```
cargo run --release -- --help
```
