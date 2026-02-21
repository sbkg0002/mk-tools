# mk-tools Project Setup Guide

## What is mk-tools?

mk-tools is a Rust-based CLI tool for managing Markdown files. It keeps documentation in sync with source code by:

1. **Synchronizing code blocks** - Automatically updates code examples in Markdown from actual source files
2. **Generating tables of contents** - Creates and maintains TOC sections based on document headings

## Project Structure

```
mk-tools/
├── .github/
│   └── workflows/
│       ├── ci.yml              # Continuous integration (tests, lint, format)
│       └── release.yml         # Automated releases for Linux & macOS
├── docs/
│   ├── spec.md                 # Complete technical specification
│   ├── SUMMARY.md              # Project overview and architecture
│   └── QUICK_REFERENCE.md      # Command and syntax reference
├── examples/
│   ├── example.md              # Demonstrates mk-tools features
│   └── hello.rs                # Sample source code
├── src/
│   ├── main.rs                 # Entry point and command handlers
│   ├── cli.rs                  # CLI argument parsing with clap
│   ├── logging.rs              # Logging configuration
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── codeblock.rs        # Codeblock domain models
│   │   └── toc.rs              # TOC domain models
│   ├── fs/
│   │   ├── mod.rs              # File system operations
│   │   └── path_utils.rs       # Path resolution utilities
│   └── markdown/
│       ├── mod.rs
│       ├── codeblocks.rs       # Codeblock parsing and processing
│       └── toc.rs              # TOC generation and processing
├── Cargo.toml                  # Rust project manifest
├── Makefile                    # Development task shortcuts
├── README.md                   # User-facing documentation
├── CHANGELOG.md                # Version history
├── CONTRIBUTING.md             # Contribution guidelines
├── BUILD_STATUS.md             # Current build status
├── LICENSE-MIT                 # MIT License
├── LICENSE-APACHE              # Apache 2.0 License
└── .gitignore                  # Git ignore rules
```

## Quick Start

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs)
- Git
- Make (optional, but recommended)

### Setup

```bash
# Clone the repository
git clone https://github.com/sbkg0002/mk-tools.git
cd mk-tools

# Build the project
cargo build

# Run tests
cargo test

# Build release version
cargo build --release
```

### Try It Out

```bash
# Run on the example files
cargo run -- codeblocks examples/example.md
cargo run -- toc examples/example.md

# Check the results
cat examples/example.md

# Verify everything is in sync
cargo run -- check examples/
```

## Development Workflow

### Using Make (Recommended)

```bash
# See all available commands
make help

# Run all checks (format, lint, test)
make check

# Build release binary
make release

# Run on examples
make run-example

# Install to system
make install
```

### Using Cargo Directly

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build (optimized)

# Test
cargo test                     # Run all tests
cargo test -- --nocapture      # Show test output

# Format & Lint
cargo fmt                      # Format code
cargo clippy                   # Run linter

# Run
cargo run -- --help            # Show help
cargo run -- codeblocks FILE   # Process a file
```

## Architecture Overview

### Layered Design

```
┌─────────────────────────────────────┐
│         CLI Layer (cli.rs)          │  ← Command-line parsing
├─────────────────────────────────────┤
│    Command Handlers (main.rs)       │  ← Business logic coordination
├─────────────────────────────────────┤
│   Domain Models (domain/)           │  ← Pure data structures
├─────────────────────────────────────┤
│  Processing Layer (markdown/)       │  ← Markdown parsing & transformation
├─────────────────────────────────────┤
│  Infrastructure Layer (fs/)         │  ← File I/O & path utilities
└─────────────────────────────────────┘
```

### Key Components

**CLI Layer** (`src/cli.rs`):
- Defines all commands and arguments using `clap` derive macros
- Validates user input
- Routes to command handlers

**Command Handlers** (`src/main.rs`):
- `handle_codeblocks()` - Orchestrates code block synchronization
- `handle_toc()` - Orchestrates TOC generation
- `handle_check()` - Validates both codeblocks and TOC
- Manages file discovery, processing loops, and error reporting

**Domain Layer** (`src/domain/`):
- `codeblock.rs` - Models: `CodeblockSpec`, `TextSpan`, `LineRange`, `CodeblockOptions`
- `toc.rs` - Models: `TocRegionSpec`, `TocOptions`, `Heading`, `TocStyle`
- Pure data structures with no I/O dependencies

**Processing Layer** (`src/markdown/`):
- `codeblocks.rs` - Parse markers, extract code, generate fences
- `toc.rs` - Parse TOC regions, extract headings, generate TOC lists
- Text transformation and manipulation logic

**Infrastructure Layer** (`src/fs/`):
- `mod.rs` - File discovery, reading, writing, backups
- `path_utils.rs` - Path resolution, extension mapping

## How It Works

### Code Block Synchronization

1. User adds a marker comment in Markdown:
   ```markdown
   <!-- mk-code: ./src/main.rs -->
   ```

2. mk-tools parses the Markdown file and finds all `mk-code` markers

3. For each marker:
   - Resolves the source file path
   - Reads the source file content
   - Applies any options (line range, dedenting)
   - Determines the code fence language
   - Inserts or updates the code block below the marker

4. Writes the updated Markdown file (with backup)

### Table of Contents Generation

1. User adds TOC markers in Markdown:
   ```markdown
   <!-- mk-toc:start -->
   <!-- mk-toc:end -->
   ```

2. mk-tools parses the file and finds all TOC regions

3. For each region:
   - Scans the document for headings (`#`, `##`, `###`, etc.)
   - Filters by configured level range
   - Generates GitHub-style anchors
   - Builds a nested list
   - Replaces content between start and end markers

4. Writes the updated file (with backup)

## Testing Strategy

### Unit Tests

Located in `#[cfg(test)]` modules within each source file:
- Domain model behavior
- Parsing logic
- Text transformations
- Path resolution
- Anchor generation

Run with: `cargo test`

### Integration Tests

Tests using the actual binary with `assert_cmd`:
- Command-line argument handling
- File processing workflows
- Error scenarios

### Test Files

Use `tempfile` crate for filesystem tests:
```rust
use tempfile::tempdir;

#[test]
fn test_something() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.md");
    // ... test implementation
}
```

## Dependencies

### Production Dependencies

```toml
clap = "4.5"           # CLI argument parsing (derive API)
anyhow = "1.0"         # Error handling with context
thiserror = "1.0"      # Custom error types
regex = "1.10"         # Pattern matching
glob = "0.3"           # File glob patterns
walkdir = "2.5"        # Directory traversal
env_logger = "0.11"    # Logging implementation
log = "0.4"            # Logging facade
```

### Development Dependencies

```toml
tempfile = "3.10"      # Temporary directories for tests
assert_cmd = "2.0"     # CLI testing utilities
predicates = "3.1"     # Test assertions
```

All dependencies are:
- Actively maintained
- Widely used in the Rust ecosystem
- Stable and production-ready
- Permissively licensed

## GitHub Actions

### CI Workflow (`.github/workflows/ci.yml`)

Runs on every push and PR:
- Tests on Ubuntu and macOS
- Checks code formatting
- Runs clippy lints
- Ensures builds succeed

### Release Workflow (`.github/workflows/release.yml`)

Triggers on version tags (e.g., `v0.1.0`):
- Builds optimized binaries for:
  - `x86_64-unknown-linux-gnu` (Linux x86_64)
  - `aarch64-apple-darwin` (macOS ARM64)
- Strips binaries for smaller size
- Creates tarballs with checksums
- Publishes GitHub Release with binaries attached

## Making a Release

### Version Bump

1. Update version in `Cargo.toml`:
   ```toml
   version = "0.2.0"
   ```

2. Update `CHANGELOG.md`:
   ```markdown
   ## [0.2.0] - 2024-XX-XX
   ### Added
   - New feature
   ```

3. Commit changes:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: bump version to 0.2.0"
   ```

### Create Release

```bash
# Create and push tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main
git push origin v0.2.0
```

GitHub Actions will automatically:
- Build binaries for all platforms
- Create GitHub Release
- Attach binaries and checksums

## Common Development Tasks

### Adding a New Feature

1. Create a feature branch:
   ```bash
   git checkout -b feature/my-feature
   ```

2. Write code and tests:
   ```rust
   pub fn my_feature() -> Result<()> {
       // Implementation
   }

   #[cfg(test)]
   mod tests {
       #[test]
       fn test_my_feature() {
           // Test implementation
       }
   }
   ```

3. Run checks:
   ```bash
   make check
   ```

4. Commit and push:
   ```bash
   git add .
   git commit -m "feat: add my feature"
   git push origin feature/my-feature
   ```

5. Create Pull Request on GitHub

### Debugging

Enable verbose logging:

```bash
# With cargo
RUST_LOG=debug cargo run -- codeblocks file.md

# With binary
RUST_LOG=debug mk-tools -vv codeblocks file.md
```

### Performance Testing

```bash
# Time execution
time cargo run --release -- codeblocks docs/

# With verbose output
time cargo run --release -- -v codeblocks docs/
```

## Configuration Files

### Cargo.toml

Key sections:
- `[package]` - Metadata, version, description
- `[dependencies]` - Runtime dependencies
- `[dev-dependencies]` - Test-only dependencies
- `[profile.release]` - Optimization settings (strip, LTO, codegen-units)

### .gitignore

Ignores:
- Build artifacts (`/target/`)
- Backup files (`*.bak`)
- IDE files (`.vscode/`, `.idea/`)
- OS-specific files (`.DS_Store`, etc.)

## Code Style

### Formatting

Use `rustfmt` (automatic):
```bash
cargo fmt
```

Config (if needed) in `rustfmt.toml`.

### Linting

Use `clippy`:
```bash
cargo clippy
```

### Naming Conventions

- **Modules**: `snake_case` (e.g., `path_utils`)
- **Types**: `PascalCase` (e.g., `CodeblockSpec`)
- **Functions**: `snake_case` (e.g., `find_markers`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_LEVEL`)

## Error Handling

Use `anyhow::Result` for most functions:

```rust
use anyhow::{Context, Result};

pub fn read_and_process(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read: {}", path.display()))?;
    
    // Process content
    Ok(content)
}
```

Provide context with `.context()` or `.with_context()` for better error messages.

## Logging

Use the `log` crate:

```rust
use log::{debug, info, warn, error};

pub fn my_function() {
    debug!("Detailed debugging info");
    info!("General information");
    warn!("Warning message");
    error!("Error occurred");
}
```

Levels controlled by:
- CLI flags: `-v` (info), `-vv` (debug), `-vvv` (trace)
- Environment: `RUST_LOG=debug`

## Documentation

### Code Documentation

Use doc comments for public APIs:

```rust
/// Parse markdown content and find all codeblock markers.
///
/// # Arguments
///
/// * `content` - The markdown file content
/// * `file_path` - Path to the markdown file
///
/// # Returns
///
/// A vector of `CodeblockSpec` containing all found markers.
///
/// # Errors
///
/// Returns an error if parsing fails or file paths cannot be resolved.
pub fn find_codeblock_markers(
    content: &str,
    file_path: &Path,
) -> Result<Vec<CodeblockSpec>> {
    // Implementation
}
```

### User Documentation

Update these files for user-facing changes:
- `README.md` - Main documentation
- `docs/QUICK_REFERENCE.md` - Quick syntax reference
- `CHANGELOG.md` - Version history
- Examples in `examples/`

## Troubleshooting

### Build Failures

```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
```

### Test Failures

```bash
# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run single-threaded (for debugging)
cargo test -- --test-threads=1
```

### Clippy Issues

```bash
# See all warnings
cargo clippy

# Auto-fix some issues
cargo clippy --fix
```

## Resources

### Project Links

- Repository: https://github.com/sbkg0002/mk-tools
- Issues: https://github.com/sbkg0002/mk-tools/issues
- Releases: https://github.com/sbkg0002/mk-tools/releases

### Documentation

- Full specification: `docs/spec.md`
- Quick reference: `docs/QUICK_REFERENCE.md`
- Contributing: `CONTRIBUTING.md`

### Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [clap Documentation](https://docs.rs/clap/)
- [anyhow Documentation](https://docs.rs/anyhow/)

## FAQ

**Q: How do I add support for a new language?**

A: Edit `src/fs/path_utils.rs` and add to the `extension_to_language()` function's default map, or use `--language-overrides` at runtime.

**Q: How do I add a new CLI option?**

A: Edit `src/cli.rs` and add the option to the appropriate Args struct, then use it in the corresponding handler in `src/main.rs`.

**Q: Where should I add new tests?**

A: Add unit tests in `#[cfg(test)]` modules at the bottom of the file being tested. Add integration tests in a `tests/` directory if needed.

**Q: How do I debug a specific file?**

A: Use verbose logging:
```bash
RUST_LOG=trace cargo run -- -vvv codeblocks your-file.md
```

**Q: Can I use mk-tools on itself?**

A: Yes! Run `make update-docs` to sync code blocks and TOCs in the project's own documentation.

## Performance Considerations

### Optimization Settings

The `Cargo.toml` includes aggressive optimization for releases:

```toml
[profile.release]
strip = true           # Remove debug symbols
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization (slower compile)
```

### Scalability

- Processes files in a single pass where possible
- Uses efficient regex compilation (compiled once, reused)
- Minimal memory allocation
- No unnecessary file system traversals

## Security

### File Operations

- All file operations are validated
- Path traversal attacks prevented
- No arbitrary code execution
- No network access required

### Dependencies

- All dependencies audited regularly
- Use `cargo audit` to check for vulnerabilities:
  ```bash
  cargo install cargo-audit
  make audit
  ```

## CI/CD Integration

### In Your Project

Add to `.github/workflows/docs-check.yml`:

```yaml
name: Documentation Check

on:
  pull_request:
    paths:
      - '**.md'
      - 'src/**'

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install mk-tools
        run: |
          curl -L https://github.com/sbkg0002/mk-tools/releases/latest/download/mk-tools-x86_64-unknown-linux-gnu.tar.gz | tar xz
          sudo mv mk-tools /usr/local/bin/
      
      - name: Check docs
        run: mk-tools check .
```

### Local Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
set -e

echo "Checking documentation..."
mk-tools check . || {
    echo "❌ Documentation needs updating!"
    echo "Run: mk-tools codeblocks . && mk-tools toc ."
    exit 1
}

echo "✅ Documentation is up-to-date"
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Next Steps

1. **Read the specification**: `docs/spec.md` for detailed design
2. **Try the examples**: Run `make run-example` to see it in action
3. **Read the code**: Start with `src/main.rs` and follow the flow
4. **Make changes**: Pick an issue or enhancement and implement it
5. **Submit PR**: Follow guidelines in `CONTRIBUTING.md`

## Support

- **Questions?** Open a discussion on GitHub
- **Bug reports?** Create an issue with details
- **Feature requests?** Open an issue with your proposal
- **Contributing?** See `CONTRIBUTING.md`

## License

This project is dual-licensed under MIT or Apache-2.0. You may choose either license.

---

**Ready to contribute?** Check out `CONTRIBUTING.md` for detailed guidelines!

**Need help?** Open an issue on GitHub: https://github.com/sbkg0002/mk-tools/issues
