# mk-tools Project Summary

## Overview

**mk-tools** is a command-line tool written in Rust for managing Markdown documentation files. It provides two main features:

1. **Code Block Synchronization**: Automatically sync code blocks in Markdown files with actual source code files
2. **Table of Contents Generation**: Automatically generate and update table of contents based on document headings

The tool is designed to keep documentation up-to-date with code changes and can be integrated into CI/CD pipelines to verify documentation accuracy.

## What We've Built

### Complete Rust Application

A fully functional CLI application with:
- Argument parsing using `clap`
- Comprehensive error handling with `anyhow` and `thiserror`
- Logging with `env_logger`
- Regular expression parsing for markers
- File system operations with backup support
- Cross-platform compatibility

### Core Features Implemented

#### 1. Code Block Synchronization
- HTML comment markers: `<!-- mk-code: <path> [options] -->`
- Options supported:
  - `lang=<language>` - Override code fence language
  - `start=<n>` and `end=<m>` - Extract specific line ranges
  - `dedent=<n>` - Remove leading indentation
  - `region=<name>` - Reserved for future use
- Automatic language detection from file extensions
- Custom language overrides via CLI
- Intelligent insertion/updating of code blocks

#### 2. Table of Contents Generation
- Marker syntax: `<!-- mk-toc:start [options] -->` ... `<!-- mk-toc:end -->`
- Options supported:
  - `from-level=<n>` - Minimum heading level (default: 2)
  - `to-level=<n>` - Maximum heading level (default: 6)
  - `style=bullet|numbered` - List style
  - `root=<path>` - Base path for links
- GitHub-style anchor generation
- Automatic deduplication of anchors
- Nested list generation based on heading hierarchy

#### 3. CLI Commands
- `codeblocks` - Sync code blocks
- `toc` - Generate/update table of contents
- `check` - Validate without modifying (CI mode)
- `version` - Show version information

#### 4. Global Options
- `-q, --quiet` - Reduce verbosity
- `-v, --verbose` - Increase verbosity (repeatable)
- `--dry-run` - Preview changes without writing
- `--color` - Control colored output
- `-C, --chdir` - Change working directory

### Project Structure

```
mk-tools/
├── .github/
│   └── workflows/
│       ├── ci.yml                 # CI workflow (tests, formatting, clippy)
│       └── release.yml            # Release workflow (builds for Linux & macOS)
├── docs/
│   ├── spec.md                    # Complete technical specification
│   └── SUMMARY.md                 # This file
├── examples/
│   ├── example.md                 # Example Markdown with markers
│   └── hello.rs                   # Example source code
├── src/
│   ├── cli.rs                     # CLI argument definitions (clap)
│   ├── logging.rs                 # Logging configuration
│   ├── main.rs                    # Entry point and command handlers
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── codeblock.rs          # Codeblock domain models
│   │   └── toc.rs                # TOC domain models
│   ├── fs/
│   │   ├── mod.rs                # File system operations
│   │   └── path_utils.rs         # Path resolution and utilities
│   └── markdown/
│       ├── mod.rs
│       ├── codeblocks.rs         # Codeblock parsing and processing
│       └── toc.rs                # TOC parsing and generation
├── Cargo.toml                     # Rust dependencies and metadata
├── README.md                      # User-facing documentation
├── CHANGELOG.md                   # Version history
├── CONTRIBUTING.md                # Contribution guidelines
├── LICENSE-MIT                    # MIT license
├── LICENSE-APACHE                 # Apache 2.0 license
└── .gitignore                     # Git ignore rules
```

### Technology Stack

**Core:**
- **Rust** (2021 edition) - Systems programming language
- **clap** 4.5 - Command-line argument parsing
- **anyhow** 1.0 - Error handling
- **regex** 1.10 - Pattern matching
- **log** + **env_logger** - Logging infrastructure

**File System:**
- **walkdir** 2.5 - Directory traversal
- **glob** 0.3 - Pattern matching

**Testing:**
- **tempfile** 3.10 - Temporary file creation for tests
- **assert_cmd** 2.0 - CLI testing utilities
- **predicates** 3.1 - Test assertions

**CI/CD:**
- GitHub Actions for automated testing and releases
- Multi-platform builds (Linux x86_64, macOS ARM64)

### Architecture

#### Layered Design

1. **CLI Layer** (`cli.rs`)
   - Argument parsing and validation
   - Command routing

2. **Command Handlers** (`main.rs`)
   - Business logic for each command
   - Error handling and reporting
   - File discovery and processing

3. **Domain Layer** (`domain/`)
   - Pure data structures
   - No I/O or external dependencies
   - Testable business logic

4. **Processing Layer** (`markdown/`)
   - Markdown parsing
   - Code block and TOC manipulation
   - Text transformation

5. **Infrastructure Layer** (`fs/`)
   - File system operations
   - Path resolution
   - Backup management

#### Key Design Patterns

- **Builder Pattern**: Used for constructing domain objects with optional fields
- **Strategy Pattern**: Different TOC styles (bullet vs numbered)
- **Result Pattern**: Comprehensive error handling throughout
- **Command Pattern**: CLI commands as first-class objects

### Test Coverage

The project includes **43 passing tests** covering:
- Domain model creation and validation
- Markdown parsing (codeblocks and TOC markers)
- File system operations
- Path resolution and language mapping
- Option parsing
- Anchor generation and deduplication
- Code extraction with ranges and dedenting
- TOC generation with various configurations

### Documentation

1. **README.md** - Comprehensive user guide with:
   - Installation instructions
   - Quick start examples
   - Complete command reference
   - Marker syntax documentation
   - CI/CD integration examples
   - Troubleshooting guide

2. **docs/spec.md** - Technical specification covering:
   - Detailed design decisions
   - Internal architecture
   - Data structures and algorithms
   - GitHub Actions workflows

3. **CONTRIBUTING.md** - Developer guide with:
   - Setup instructions
   - Development workflow
   - Coding standards
   - Testing guidelines
   - Commit message conventions

4. **CHANGELOG.md** - Version history and release notes

5. **Inline Documentation** - Comprehensive doc comments on all public APIs

### Build and Release

#### CI Pipeline
- Runs on every push and PR
- Tests on Ubuntu and macOS
- Checks formatting with `cargo fmt`
- Lints with `cargo clippy`
- Runs full test suite

#### Release Pipeline
- Triggered on version tags (e.g., `v0.1.0`)
- Builds optimized binaries for:
  - Linux x86_64 (ubuntu-latest)
  - macOS ARM64 (macos-latest)
- Creates GitHub Release with:
  - Compiled binaries (stripped)
  - SHA256 checksums
  - Release notes
  - Installation instructions

### Current Status

✅ **Complete and Functional**
- Core features fully implemented
- Comprehensive test coverage
- Documentation complete
- CI/CD pipelines configured
- Ready for initial release

### Usage Example

```bash
# Sync code blocks in documentation
mk-tools codeblocks docs/

# Generate table of contents
mk-tools toc README.md

# Check if docs are up-to-date (CI mode)
mk-tools check . || exit 1

# Preview changes without writing files
mk-tools --dry-run codeblocks docs/

# Verbose output for debugging
mk-tools -vv codeblocks README.md
```

### Next Steps

**For v0.1.0 Release:**
1. Tag the release: `git tag v0.1.0`
2. Push the tag: `git push origin v0.1.0`
3. GitHub Actions will automatically build and release

**Future Enhancements (v0.2.0+):**
1. Named region support (`region=<name>` option)
2. Cross-file TOC generation
3. Additional encodings beyond UTF-8
4. Configuration file support (`.mk-tools.toml`)
5. Watch mode for live updates
6. Markdown link validation
7. Custom anchor generation strategies
8. Support for more code fence styles (tildes `~~~`)
9. Integration with more CI systems
10. Performance optimizations for large codebases

### Dependencies Overview

**Production Dependencies:**
- `clap` - CLI parsing (well-maintained, stable)
- `anyhow` - Error handling (industry standard)
- `regex` - Pattern matching (official Rust regex)
- `walkdir` - Directory traversal (widely used)
- `glob` - File pattern matching (standard utility)
- `env_logger` + `log` - Logging (standard logging facade)
- `thiserror` - Custom error types (ergonomic error handling)

**Development Dependencies:**
- `tempfile` - Test utilities (de-facto standard for temp files)
- `assert_cmd` - CLI testing (from assert_cmd/assert_fs suite)
- `predicates` - Test assertions (flexible test predicates)

All dependencies are:
- Well-maintained with active development
- Widely used in the Rust ecosystem
- Have stable APIs
- Licensed permissively (MIT/Apache-2.0)

### Code Quality

**Metrics:**
- **Lines of Code**: ~3,500 lines (including tests and docs)
- **Test Coverage**: 43 unit tests, all passing
- **Clippy Warnings**: Addressed (minor unused code warnings only)
- **Format**: Consistently formatted with `rustfmt`
- **Documentation**: All public APIs documented
- **Error Handling**: Comprehensive with context

### Performance Characteristics

- **Fast**: Rust's zero-cost abstractions mean minimal overhead
- **Memory Safe**: No unsafe code, all memory safety guaranteed by Rust
- **Efficient**: Processes files in a single pass where possible
- **Scalable**: Handles large codebases and documentation sets

### Platform Support

**Tier 1 (Tested and Supported):**
- Linux x86_64 (ubuntu-latest)
- macOS ARM64 (macos-latest)

**Tier 2 (Should Work):**
- macOS x86_64
- Windows (builds in CI)
- Other Unix-like systems

### Security Considerations

- No network access required
- Reads only specified files
- Creates backups by default
- No external code execution
- All file operations are validated
- Path traversal protections in place

### License

Dual-licensed under MIT OR Apache-2.0 (user's choice), following Rust ecosystem conventions.

---

## Getting Started as a Developer

```bash
# Clone and setup
git clone https://github.com/sbkg0002/mk-tools.git
cd mk-tools

# Build
cargo build

# Run tests
cargo test

# Try it out on examples
cargo run -- codeblocks examples/example.md
cargo run -- toc examples/example.md

# Build release version
cargo build --release

# The binary will be at: target/release/mk-tools
```

## Getting Started as a User

```bash
# Download latest release
curl -L https://github.com/sbkg0002/mk-tools/releases/latest/download/mk-tools-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv mk-tools /usr/local/bin/

# Use it
mk-tools --help
mk-tools codeblocks README.md
mk-tools toc docs/
```

---

**Project Status**: Ready for v0.1.0 release
**Maintainer**: sbkg0002
**Repository**: https://github.com/sbkg0002/mk-tools
