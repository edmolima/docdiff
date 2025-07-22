
## [1.2.0] - 2025-07-23

### Added
- Homebrew formula for easy installation on macOS/Linux.
- New command: `docdiff info` for program metadata and version.
- Improved CLI help and error messages.

### Changed
- Enhanced progress bar for large file comparisons.
- Updated dependencies for security and performance.

### Fixed
- Fixed edge case in document distance calculation for empty files.

### Security
- No known security issues in this release.

## [1.1.0] - 2025-07-23

### Added
- Colorized output for verdicts (identical, similar, different).
- Integration tests for CLI commands.
- Support for Windows pre-built binaries.

### Changed
- Refactored CLI for better modularity and maintainability.

### Fixed
- Fixed bug with file encoding detection.

### Security
- No known security issues in this release.

## [1.0.0] - 2025-07-23

### Added
- Initial public release of docdiff.
- Fast and robust document distance algorithm (cosine similarity).
- CLI with friendly feedback and verdicts (identical, similar, different).
- Multiplatform support (macOS, Linux, Windows).
- Automated release workflow with changelog and GitHub Release integration.

### Changed
- Modular codebase for scalability and maintainability.

### Fixed
- Lint and formatting issues resolved for Rust 2024 edition.

### Security
- No known security issues in this release.
