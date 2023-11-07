# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
 - Derived methods for obtaining references to inner values:
   - `.{variant_name}_ref()`
   - `.{variant_name}_ref_or()`
   - `.{variant_name}_ref_or_else()`

## [0.2.0] - 2021-01-12
### Added
- Derived methods for producing Options & Results:
    - `.{variant_name}()`
    - `.{variant_name}_or()`
    - `.{variant_name}_or_else()`
- CHANGELOG.md
### Changed
- Improved documentation in README.md & base lib.rs
### Deprecated
- Derived methods that begin with `.ok` including:
    - `.ok_{variant_name}()`
    - `.ok_or_{variant_name}()`
    - `.ok_or_else{variant_name}()`

## [0.1.0] - 2020-12-29
### Added
- Initial Implementation, License & README.md

[Unreleased]: https://github.com/luker-os/variantly/releases/tag/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/luker-os/variantly/releases/tag/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/luker-os/variantly/releases/tag/v0.1.0