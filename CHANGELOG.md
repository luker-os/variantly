# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2023-11-27
### Added
 - Derived methods for obtaining mutable references to inner values:
    - `.{variant_name}_mut()`
    - `.{variant_name}_mut_or()`
    - `.{variant_name}_mut_or_else()`

### Fixed
  - All items from `std::prelude` now use fully qualified syntax so as to avoid conflicting with other items of the same name that are in scope. This will allow the use of the derive macro in situations where a custom Result or similar is defined.

### Breaking
    - Newly derived methods could potentially conflict with manually derived implementations of the same name.
      - Example:
        - Given a Color enum with an `HSV` variant and a manually implemented method named `hsv_mut`,the newly derived `.{variant_name}_mut()` will conflict with the manual implementation causing a compilation error.
      - Resolution:
        - If the manually derived methods provide the same functionality as the derived one, you can remove the manual implementation. Otherwise, consider [renaming](https://docs.rs/variantly/0.3.0/variantly/#renaming-methods) the derived methods.


## [0.3.0] - 2023-11-07
### Added
 - Derived methods for obtaining references to inner values:
    - `.{variant_name}_ref()`
    - `.{variant_name}_ref_or()`
    - `.{variant_name}_ref_or_else()`

### Breaking
    - Newly derived methods could potentially conflict with manually derived implementations of the same name.
      - Example:
        - Given a Color enum with an `HSV` variant and a manually implemented method named `hsv_ref`,the newly derived `.{variant_name}_ref()` will conflict with the manual implementation causing a compilation error.
      - Resolution:
        - If the manually derived methods provide the same functionality as the derived one, you can remove the manual implementation. Otherwise, consider [renaming](https://docs.rs/variantly/0.2.0/variantly/#renaming-methods) the derived methods.

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

[Unreleased]: https://github.com/luker-os/variantly/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/luker-os/variantly/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/luker-os/variantly/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/luker-os/variantly/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/luker-os/variantly/releases/tag/v0.1.0