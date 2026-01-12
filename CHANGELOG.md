# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-01-12

### Added

- **Syntax highlighting**: Code blocks now include language identifiers (e.g., `rust`, `typescript`)
- **50+ file types**: Extended support for many programming languages including Java, C/C++, Ruby, PHP, Swift, Kotlin, Scala, Haskell, and more
- **.gitignore support**: Automatically respects `.gitignore`, global gitignore, and `.git/info/exclude` patterns
- GitHub Actions workflow for automated releases

### Changed

- Improved README with badges, feature list, and comprehensive file type documentation
- Updated LICENSE author name
- Replaced `walkdir` with `ignore` crate for gitignore support
- Function signatures now use `&Path` instead of `&PathBuf` (more idiomatic)

## [0.2.0] - 2025-01-12

### Changed

- Output format changed to Markdown

## [0.1.0] - 2025-01-12

### Added

- Initial release
- CLI tool for collecting code files for code review
- Support for Rust (`.rs`), TypeScript (`.ts`), JavaScript (`.js`), Python (`.py`), and Go (`.go`) files
- Recursive directory traversal
- Output to stdout or file via `--output` option
- Cross-platform support (Linux, macOS, Windows)

[0.2.1]: https://github.com/kyuki3rain/cr-prep/releases/tag/v0.2.1
[0.2.0]: https://github.com/kyuki3rain/cr-prep/releases/tag/v0.2.0
[0.1.0]: https://github.com/kyuki3rain/cr-prep/releases/tag/v0.1.0
