# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.6](https://github.com/FlippingBinaryLLC/wait-rs/compare/v0.2.5...v0.2.6) - 2025-01-21

### Fixed

- cannot find function `block_in_place` in module `tokio::task`

## [0.2.5](https://github.com/FlippingBinaryLLC/wait-rs/compare/v0.2.4...v0.2.5) - 2024-11-09

### Fixed

- It should not panic if run from a `tokio` runtime without I/O or timer drivers

### Other

- Relax tokio dependency version to improve flexibility
- Switch to tokio/rt instead of tokio/r-multi-thread
- Remove ambiguity around "by default" phrase

## [0.2.4](https://github.com/FlippingBinaryLLC/wait-rs/compare/v0.2.3...v0.2.4) - 2024-11-08

### Fixed

- It should not panic if used inside an existing tokio runtime

### Other

- Clarify whose function is `async`

## [0.2.3](https://github.com/FlippingBinaryLLC/wait-rs/compare/v0.2.2...v0.2.3) - 2024-11-08

### Fixed

- Doc-test should ignore the incomplete example in the `README.md`
- Ensure `tokio` feature is enabled when building example that requires it

### Other

- Simplify the main docs
- Add link to GitHub repository in `README.md`
- Improve clarity of prelude documentation
- Add favicon
- Add clarification in example

## [0.2.2](https://github.com/FlippingBinaryLLC/wait-rs/compare/v0.2.1...v0.2.2) - 2024-11-03

### Added

- Add support for `async` functions that require a `tokio` runtime

### Other

- Add example and documentation of support for `tokio`-dependent `async` functions
- Cleanup some style issues
- Split `wait_block_on` into `std` and `nostd` versions
- Move blocking logic into separate function
- Cleanup install instructions

## [0.2.1](https://github.com/FlippingBinaryLLC/wait-rs/compare/v0.2.0...v0.2.1) - 2024-11-01

### Other

- Update version number

## [0.2.0](https://github.com/FlippingBinaryLLC/wait-rs/compare/v0.1.1...v0.2.0) - 2024-11-01

### Added

- Add the new `std` default feature flag

### Fixed

- Linting error

### Other

- Update `README.md` to describe building with `no_std`
- Relax the linter's restriction on unsafe code
- Remove deprecated module name
