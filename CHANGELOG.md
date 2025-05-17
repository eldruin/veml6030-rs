# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Added

- The `IntegrationTime` enum now has `as_ms` and `as_us` methods
- `async` support via `maybe-async`

### Changed

- bump `embedded-hal` to 1.0.0
- bump `libm` to 0.2
- bump `linux-embedded-hal` to 0.4

### Changed

- Upgrade MSRV to 1.75.0

## [0.1.1] - 2019-12-21

Note compatibility with VEML7700 in the documentation.

## [0.1.0] - 2019-12-20

This is the initial release to crates.io of the feature-complete driver. There
may be some API changes in the future, in case I decide that something can be
further improved.

<!-- next-url -->
[Unreleased]: https://github.com/eldruin/veml6030-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/eldruin/veml6030-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/eldruin/veml6030-rs/releases/tag/v0.1.0
