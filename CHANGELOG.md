# Changelog

## [0.3.6](https://github.com/hbjydev/rkan/compare/v0.3.5...v0.3.6) (2026-11-17)


### Bug Fixes

* **binstall:** override cargo-binstall metadata to ensure source builds arent needed ([6dcd5b7](https://github.com/hbjydev/rkan/commit/6dcd5b74722bb048476811e8d39af3f8fe0e07a5))

## [0.3.5](https://github.com/hbjydev/rkan/compare/v0.3.4...v0.3.5) (2026-11-16)


### Documentation

* add rkan crate metadata ([411d4d3](https://github.com/hbjydev/rkan/commit/411d4d32872429ab3b637f8d044ce4371644de6f))
* add usage and config guides ([b0b5e61](https://github.com/hbjydev/rkan/commit/b0b5e6191c42f751d454a359601e885502957fd4))
* demo in the readme, updated by github actions ([d4be7eb](https://github.com/hbjydev/rkan/commit/d4be7eb366ac4f87f66bb02c761d4f4ce1b6014c))


### Continuous Integration

* **release:** include target in release binary name ([f6991dc](https://github.com/hbjydev/rkan/commit/f6991dc9b684d81a992722eb0eb179cef73a4d52))

## [0.3.4](https://github.com/hbjydev/rkan/compare/v0.3.3...v0.3.4) (2026-11-16)


### Continuous Integration

* **release:** use bash in all mise tasks ([9ecf2cc](https://github.com/hbjydev/rkan/commit/9ecf2cc197ca8281773a3b058a7e7c2683dacdb8))

## [0.3.3](https://github.com/hbjydev/rkan/compare/v0.3.2...v0.3.3) (2026-11-16)


### Continuous Integration

* **release:** shell=bash for all tasks ([47e09ae](https://github.com/hbjydev/rkan/commit/47e09ae790d4cccc3fb20d489b5b44b39aa67146))

## [0.3.2](https://github.com/hbjydev/rkan/compare/v0.3.1...v0.3.2) (2026-11-16)


### Continuous Integration

* **release:** cargo-deny isn't needed for release ([eb2d48a](https://github.com/hbjydev/rkan/commit/eb2d48a35cee8a87bcffdff1085be716571a6d0f))
* **release:** hk isn't needed for build ([52298ef](https://github.com/hbjydev/rkan/commit/52298ef660943819736240f47a43a1ce86d9303b))
* **release:** remove macos intel target ([534b40a](https://github.com/hbjydev/rkan/commit/534b40ac57258a6f1c6f1fa68fdb06ed7a0743d6))

## [0.3.1](https://github.com/hbjydev/rkan/compare/v0.3.0...v0.3.1) (2026-11-16)


### Features

* **rust:** update crate indexmap (2.13.1 → 2.14.2) ([#18](https://github.com/hbjydev/rkan/issues/18)) ([318eeb8](https://github.com/hbjydev/rkan/commit/318eeb8e30042f79d069c782c6d97b90a1ca1958))
* **rust:** update crate regex (1.12.3 → 1.13.1) ([#19](https://github.com/hbjydev/rkan/issues/19)) ([2cd3acc](https://github.com/hbjydev/rkan/commit/2cd3accbc7e82fd3bd73896018cb2fc130159a1b))
* **rust:** update crate toml (1.1.2+spec-1.1.0 → 1.2.1) ([#20](https://github.com/hbjydev/rkan/issues/20)) ([0973418](https://github.com/hbjydev/rkan/commit/09734183ae9c4bcfb20b0779367643a055d09b65))


### Bug Fixes

* **deps:** patch dependency security advisories ([#7](https://github.com/hbjydev/rkan/issues/7)) ([4375591](https://github.com/hbjydev/rkan/commit/4375591b2c9bd6bfec064d8f2c7a443c627b1722))
* **rust:** update crate chrono (0.4.44 → 0.4.45) ([#5](https://github.com/hbjydev/rkan/issues/5)) ([8a464a7](https://github.com/hbjydev/rkan/commit/8a464a7fabbeae9a97124bbad0565a683b0c2a4e))
* **rust:** update crate clap (4.6.0 → 4.6.7) ([#8](https://github.com/hbjydev/rkan/issues/8)) ([f4a05af](https://github.com/hbjydev/rkan/commit/f4a05afd74d933ad71e53f7e86a6ec3db1bec78e))
* **rust:** update crate futures-util (0.3.32 → 0.3.34) ([#10](https://github.com/hbjydev/rkan/issues/10)) ([e97f8f4](https://github.com/hbjydev/rkan/commit/e97f8f41ecfe0366f97df51c864b55d901d96bf0))
* **rust:** update crate indicatif (0.18.4 → 0.18.6) ([#11](https://github.com/hbjydev/rkan/issues/11)) ([5f77b06](https://github.com/hbjydev/rkan/commit/5f77b06e9d96e5968c1798267a64983d5ad515c6))
* **rust:** update crate reqwest (0.13.2 → 0.13.5) ([#13](https://github.com/hbjydev/rkan/issues/13)) ([fe95d75](https://github.com/hbjydev/rkan/commit/fe95d75902b627fd84288592a60daf15cfbc1594))
* **rust:** update crate serde (1.0.228 → 1.0.229) ([#14](https://github.com/hbjydev/rkan/issues/14)) ([01ec57e](https://github.com/hbjydev/rkan/commit/01ec57eae7d34bfcdc61991866898140401fc495))
* **rust:** update crate serde_json (1.0.149 → 1.0.154) ([#15](https://github.com/hbjydev/rkan/issues/15)) ([80a9de1](https://github.com/hbjydev/rkan/commit/80a9de10f50e0070829bd6efad16110ffd026c8c))
* **rust:** update crate sha2 (0.11.0 → 0.11.1) ([#16](https://github.com/hbjydev/rkan/issues/16)) ([de7ebbb](https://github.com/hbjydev/rkan/commit/de7ebbb661d17618e261fc76a5279893a881ddbd))
* **rust:** update crate thiserror (2.0.18 → 2.0.21) ([#17](https://github.com/hbjydev/rkan/issues/17)) ([3ffcfc6](https://github.com/hbjydev/rkan/commit/3ffcfc60f8f19e631d019345546863c8dc421443))


### Code Refactoring

* **github:** replace octorust with octocrab ([#9](https://github.com/hbjydev/rkan/issues/9)) ([5c05805](https://github.com/hbjydev/rkan/commit/5c05805a9c0c45402b53f0dde35214a57b0c86e5))
* **validation:** replace `lazy_static` with `std::sync::LazyLock` ([#21](https://github.com/hbjydev/rkan/issues/21)) ([4a510b5](https://github.com/hbjydev/rkan/commit/4a510b503f613be54982db7a22a269610c29df62))


### Continuous Integration

* overhaul checks and release automation ([#2](https://github.com/hbjydev/rkan/issues/2)) ([8d074db](https://github.com/hbjydev/rkan/commit/8d074db52da0753aee33fd5ffccd590adee0b1a9))
