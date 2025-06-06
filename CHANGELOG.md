# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2025-06-06

### Added

- Added `Default` to `ProcResult`, `unix::WaitStatus`, and `windows::ExitCode`.

### Changed

- `impl From<std::process::ExitCode> for ProcResult` is now conditional on
  `unix` or `windows`.

## [0.2.1] - 2025-06-01

### Changed

- Updated examples in `README.md` to reflect the new API.

## [0.2.0] - 2025-06-01

### Added

- Added `impl From<&std::proccess::ExitStatus>` for the new `ProcResult` enum.

### Changed

- The top level `ProcResult` type is now an enum of possible exit statuses.
- Conversions to/from `ExitStatus` are now owned instead of a reference.

### Removed

- `Error` and `ToProcResult` was removed.

## [0.1.0] - 2025-05-31

### Added

- Initial release.
