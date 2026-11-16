# RKAN

[![Crates.io Version](https://img.shields.io/crates/v/rkan)](https://crates.io/crates/rkan)

Like NetKAN, but in Rust, so... RKAN.

A NetKAN replacement for KSP modders to build and maintain their own CKAN
metadata files & repositories.

![RKAN generates CKAN metadata from the E2E files](./docs/demo/rkan-e2e.gif)

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

# to generate a debug build in target/<target-triple>/debug/rkan:
$ mise run build

# to generate a release build in target/<target-triple>/release/rkan:
$ mise run build-release
```

## Quick start

RKAN reads TOML configuration files and generates CKAN metadata, the information
that CKAN uses to download and install mods. Each mod needs a GitHub release
with a ZIP file attached.

After installation, create a directory for your configuration files:

```bash
mkdir -p configs
```

Save the following as `configs/MyCoolMod.toml`.
Replace the example values with your mod details.
Set `file` to a path inside your ZIP file.

```toml
identifier = "MyCoolMod"
name = "My Cool Mod"
repo = "YOUR-ACCOUNT/YOUR-MOD"
abstract = "A short description of the mod."
authors = ["Your Name"]
tags = ["plugin"]
license = "MIT"
ksp_version = "1.12"
asset_match = '^MyCoolMod-.*\.zip$'

[resources]

[[install]]
file = "GameData/MyCoolMod"
install_to = "GameData"
```

From the directory that contains `configs/`, run:

```bash
rkan generate
```

RKAN downloads the matching ZIP from the latest release and makes sure that the
metadata and installation paths meet its requirements. For a release tagged
`v1.0.0`, it writes `ckan/MyCoolMod/MyCoolMod-v1.0.0.ckan`.
RKAN creates the output directories. It does not publish the files or install
the mod into KSP.

To generate only this mod, use its identifier:

```bash
rkan generate MyCoolMod
```

See the [usage guide](./docs/usage.md) for command flags, authentication, output
behavior, and troubleshooting. See the [configuration reference](./docs/configuration.md)
for dependencies, installation paths, and variants.
