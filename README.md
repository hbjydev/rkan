# RKAN

Like NetKAN, but in Rust, so... RKAN.

A NetKAN replacement for KSP modders to build and maintain their own CKAN
metadata files & repositories.

This tool takes configurations from the `configs/` directory and generates a
series of `.ckan` files for them in `../ckan/`.

## Installation

You can install it with `cargo`.

```bash
$ cargo install rkan
```

## Configuration Reference

The config files are TOML files. See [MyCoolMod.toml](./examples/MyCoolMod.toml)
for an example of the configuration syntax used to generate CKAN files.
