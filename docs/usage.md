# Usage

RKAN generates `.ckan` files from TOML configuration files and GitHub releases.
Start with the [README quick start](../README.md#quick-start) to configure your
first mod. The [configuration reference](./configuration.md) describes every
supported field.

## Generate metadata

From the directory that contains `configs/`, run:

```bash
rkan generate
```

RKAN searches `./configs` and its subdirectories for files with a `.toml`
extension. For each mod, it requests the latest release from GitHub, downloads
the selected ZIP files, and generates metadata. The command does not accept
a release tag or a local ZIP file.

To use different input and output directories, run:

```bash
rkan generate --configs-dir ./mod-configs --out-dir ./metadata
```

Paths are relative to the directory where you run the command.
The input directory must exist. RKAN creates the output directories as needed.

## Select mods

Filters match the `identifier` field exactly, including letter case.
They do not match filenames or variant identifiers. To select one mod, run:

```bash
rkan generate MyCoolMod
```

For configurations named `MyCoolMod` and `AnotherMod`, either form selects both:

```bash
rkan generate MyCoolMod AnotherMod
rkan generate MyCoolMod,AnotherMod
```

Selecting a mod generates all of its configured variants, the alternative
packages for that mod. RKAN parses every TOML file before it applies the filter.
A malformed file can therefore stop a filtered run. A filter with no matches
returns success without generating files.

## Command reference

All generation flags work before or after `generate`.
The `--jobs` value controls how many mods RKAN processes at once.
Use a positive value. Variants within a mod run concurrently as well.

| Flag                     | Short form | Default                | Purpose                                                     |
| ------------------------ | ---------- | ---------------------- | ----------------------------------------------------------- |
| `--configs-dir <PATH>`   | `-c`       | `./configs`            | Directory containing TOML configuration files.              |
| `--out-dir <PATH>`       | `-o`       | `./ckan`               | Directory for generated metadata.                           |
| `--jobs <NUMBER>`        | `-j`       | `4`                    | Maximum number of mods processed at once.                   |
| `--github-token <TOKEN>` | `-g`       | `GITHUB_TOKEN`, if set | Token for GitHub API requests.                              |
| `--help`                 | `-h`       | None                   | Show help for the command.                                  |
| `--version`              | `-V`       | None                   | Show the RKAN version. Use with `rkan`, without `generate`. |

For example, process mods one at a time with:

```bash
rkan generate --jobs 1
```

## Authentication and logs

Public repositories work without a token, subject to GitHub API limits.
RKAN reads a token from the `GITHUB_TOKEN` environment variable, a value supplied
by your shell or CI runner. An explicit `--github-token` value
overrides that variable.

In CI, supply `GITHUB_TOKEN` through the runner environment or its secret store.
The token authenticates API requests. RKAN does not attach it to ZIP downloads,
so it does not fully support private release assets.

RKAN shows informational logs by default. To include debug logs, run this
command in a POSIX shell such as Bash or Zsh:

```bash
RUST_LOG=rkan=debug rkan generate MyCoolMod
```

## Generated files

RKAN writes one file per mod, or one file per variant when variants exist.
The output path uses the base identifier, the generated identifier, and the
release tag:

```text
ckan/<base-identifier>/<generated-identifier>-<release-tag>.ckan
```

For a mod named `MyCoolMod`, a `4k` variant, and a `v1.0.0` release, the path is
`ckan/MyCoolMod/MyCoolMod-4k-v1.0.0.ckan`.
Without variants, the generated identifier equals the base identifier.

The metadata includes download URLs, file sizes, and SHA-1 and SHA-256 hashes,
the values used to identify file contents. RKAN downloads and extracts ZIP
files into temporary storage during generation. Each run downloads the assets
again.

A successful run returns exit status `0`. A failed run returns a nonzero status.
Files from successful mods can remain after another mod fails.
For the same release tag, a rerun overwrites the corresponding files.
RKAN leaves files from older releases in place.

## Run the repository examples

From the root of a clone of this repository, run:

```bash
cargo run --locked -- generate --configs-dir tests/e2e --out-dir target/e2e/ckan
```

These examples use live GitHub releases and download large texture archives.
They generate metadata for Sol Core and the configured Sol Uranus System variant.
The files appear under `target/e2e/ckan/`.

## Troubleshooting

| Symptom                                 | Action                                                                                                               |
| --------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `Failed to load mod configs`            | Make sure that the input directory exists and all TOML files contain the required fields.                            |
| `No asset matching`                     | Make sure that `asset_match` matches a ZIP filename attached to the latest release.                                  |
| Missing files or directories in the ZIP | Make sure that each `install.file` matches a path inside the ZIP, including letter case.                             |
| Invalid installation path               | Use a supported `install_to` value from the [configuration reference](./configuration.md#installation-paths).        |
| Invalid identifier or tags              | Use the identifier format and tags described in the [configuration reference](./configuration.md#metadata).          |
| GitHub API or network error             | Make sure that the repository and release exist. Make sure that GitHub is reachable and any supplied token is valid. |
| Success with no output files            | Make sure that the input directory contains `.toml` files and the filter matches a base identifier.                  |

For more detail about a failure, use the `RUST_LOG=rkan=debug` command above.
