# RKAN

[![Crates.io Version](https://img.shields.io/crates/v/rkan)](https://crates.io/crates/rkan)

Like NetKAN, but in Rust, so... RKAN.

A NetKAN replacement for KSP modders to build and maintain their own CKAN
metadata files & repositories.

![RKAN generates CKAN metadata from the E2E files](./docs/demo/rkan-e2e.gif)

## Configuration Reference

The config files are TOML files. See [MyCoolMod.toml](./examples/MyCoolMod.toml)
for an example of the configuration syntax used to generate CKAN files.

## Installation

### Binaries

You can either install RKAN by downloading a binary from
[GitHub Releases](https://github.com/hbjydev/rkan/releases/latest) for your
platform, or by using `cargo-binstall`:

```bash
$ cargo binstall rkan
```

### From Source

You can install it with `cargo` directly from [crates.io](https://crates.io/crates/rkan).

```bash
$ cargo install rkan
```

Or, you can clone this repository and use [Mise](https://mise.jdx.dev) to build
the project:

```bash
$ git clone https://github.com/hbjydev/rkan --depth 1
$ cd rkan
$ mise trust
$ mise install

# to generate a debug build in target/debug/rkan:
$ mise run build

# to generate a release build in target/release/rkan:
$ mise run build-release
```
