# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Cross-File TOC Generation**: New `--from-dir` option for `toc` command
  - Generate a single table of contents in one file from headings across multiple files
  - `mk-tools toc README.md --from-dir docs/` creates a TOC in README.md with links to all files in docs/
  - Automatically generates relative links with file paths (e.g., `[Heading](docs/file.md#heading)`)
  - Useful for creating master TOC pages that index entire documentation directories
- **Automatic TOC Marker Insertion**: New `--add` option for `toc` command
  - Automatically inserts TOC markers (`<!-- mk-toc:start -->` / `<!-- mk-toc:end -->`) into files that don't have them
  - `mk-tools toc file.md --add` adds markers below the first H1 heading and generates the TOC
  - If no H1 heading exists, markers are inserted at the beginning of the file
  - Only adds markers if they don't already exist - safe to run multiple times
  - Immediately generates the TOC after adding markers

### Changed

- Commands now default to current directory when no paths are specified
  - `mk-tools codeblocks` processes all Markdown files in current directory
  - `mk-tools toc` processes all Markdown files in current directory
  - `mk-tools check` processes all Markdown files in current directory

### Deprecated

- Nothing yet

### Removed

- Nothing yet

### Fixed

- Nothing yet

### Security

- Nothing yet

## [0.1.0] - 2024-01-XX

### Added

- Initial release of mk-tools
- **Code Block Synchronization**: Automatically sync code blocks in Markdown files with source files
  - Support for `mk-code` markers with HTML comment syntax
  - Line range extraction with `start` and `end` options
  - Automatic language detection based on file extension
  - Custom language override with `lang` option
  - Indentation removal with `dedent` option
  - Configurable language mappings via `--language-overrides`
- **Table of Contents Generation**: Automatically generate and update TOC in Markdown files
  - Support for `mk-toc:start` and `mk-toc:end` markers
  - Configurable heading level filtering with `from-level` and `to-level`
  - Bullet and numbered list styles
  - GitHub-style anchor generation with automatic deduplication
- **CLI Interface**: User-friendly command-line interface built with clap
  - `codeblocks` command for code synchronization
  - `toc` command for TOC generation
  - `check` command for CI/CD validation
  - Global options: `--quiet`, `--verbose`, `--dry-run`, `--color`, `--chdir`
  - Per-command options: `--check`, `--no-backup`, `--root`, `--glob`
- **File Discovery**: Automatic discovery of Markdown files
  - Support for processing individual files or entire directories
  - Configurable glob patterns for file matching
  - Recursive directory traversal
- **Backup Creation**: Automatic backup files (`.bak`) before modifications
  - Can be disabled with `--no-backup` flag
- **CI/CD Integration**: Check mode for verifying documentation is up-to-date
  - Non-zero exit codes when updates are needed
  - Suitable for use in GitHub Actions and other CI systems
- **Error Handling**: Comprehensive error messages with context
  - Clear reporting of which files need updates
  - Detailed error messages for common issues
- **Logging**: Configurable logging levels
  - Support for `-v`, `-vv`, `-vvv` for increased verbosity
  - Respect for `RUST_LOG` environment variable
  - Colored output with auto-detection
- **GitHub Actions Workflows**:
  - Release workflow for automated binary builds (Linux x86_64, macOS ARM64)
  - CI workflow with tests, formatting checks, and clippy lints
- **Documentation**:
  - Comprehensive README with usage examples
  - Detailed specification document
  - Working examples in the `examples/` directory
  - Inline code documentation and tests

### Technical Details

- Written in Rust for performance and reliability
- Uses regex-based parsing for marker detection
- Preserves file structure and formatting
- UTF-8 encoding support
- Cross-platform compatibility (Linux, macOS, Windows)

### Known Limitations

- Only UTF-8 encoding is supported
- Named regions (`region` option) are reserved for future use
- Cross-file TOCs are not yet supported

[Unreleased]: https://github.com/sbkg0002/mk-tools/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sbkg0002/mk-tools/releases/tag/v0.1.0
