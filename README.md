# mk-tools

A powerful CLI tool for managing Markdown files with automatic code block synchronization and table of contents generation.

[![CI](https://github.com/sbkg0002/mk-tools/workflows/CI/badge.svg)](https://github.com/sbkg0002/mk-tools/actions)
[![Release](https://github.com/sbkg0002/mk-tools/workflows/Release/badge.svg)](https://github.com/sbkg0002/mk-tools/actions)

## Features

- **Code Block Synchronization**: Keep code blocks in your Markdown files in sync with actual source files
- **Table of Contents Generation**: Automatically generate and update table of contents based on headings
- **Cross-File TOC**: Generate a single TOC in one file from headings across multiple files in a directory
- **CI/CD Integration**: Verify documentation is up-to-date in your CI pipeline
- **Flexible Configuration**: Customize behavior with extensive options
- **Fast & Reliable**: Written in Rust for performance and safety

## Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/sbkg0002/mk-tools/releases).

#### Linux (x86_64)

```bash
curl -L https://github.com/sbkg0002/mk-tools/releases/latest/download/mk-tools-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv mk-tools /usr/local/bin/
```

#### macOS (ARM64)

```bash
curl -L https://github.com/sbkg0002/mk-tools/releases/latest/download/mk-tools-aarch64-apple-darwin.tar.gz | tar xz
sudo mv mk-tools /usr/local/bin/
```

### Build from Source

```bash
git clone https://github.com/sbkg0002/mk-tools.git
cd mk-tools
cargo build --release
sudo cp target/release/mk-tools /usr/local/bin/
```

## Quick Start

### Syncing Code Blocks

Add a marker comment in your Markdown file:

```markdown
# My Project

Here's the main function:

<!-- mk-code: ./src/main.rs -->
```

Run mk-tools:

```bash
mk-tools codeblocks README.md
```

The code block will be automatically inserted/updated:

````markdown
# My Project

Here's the main function:

<!-- mk-code: ./src/main.rs -->

```rust
fn main() {
    println!("Hello, world!");
}
```
````

````

### Generating Table of Contents

Add TOC markers in your Markdown:

```markdown
# Documentation

<!-- mk-toc:start -->
<!-- mk-toc:end -->

## Getting Started
### Installation
### Configuration

## Advanced Usage
````

Run mk-tools:

```bash
mk-tools toc README.md
```

The TOC will be generated:

```markdown
# Documentation

<!-- mk-toc:start -->

- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Configuration](#configuration)
- [Advanced Usage](#advanced-usage)
<!-- mk-toc:end -->

## Getting Started

### Installation

### Configuration

## Advanced Usage
```

## Usage

### Global Options

```
mk-tools [OPTIONS] <COMMAND>

Global Options:
  -q, --quiet              Reduce output verbosity
  -v, --verbose            Increase output verbosity (can be repeated: -v, -vv, -vvv)
      --dry-run            Show what would be done without writing files
      --color <WHEN>       Control colored output [default: auto] [possible values: auto, always, never]
  -C, --chdir <DIR>        Change working directory before running
  -h, --help               Print help
  -V, --version            Print version
```

### Commands

#### `codeblocks` - Sync Code Blocks

Synchronize code blocks in Markdown files with source files.

```bash
mk-tools codeblocks [OPTIONS] [PATHS]...
```

**Options:**

- `--root <PATH>` - Base directory for resolving source file paths (default: Markdown file's directory)
- `--glob <PATTERN>` - Glob pattern for Markdown files when processing directories (default: `**/*.md`)
- `--language-overrides <MAP>` - Override language mappings (e.g., `py=python,rs=rust`)
- `--check` - Don't modify files; exit with error if updates are needed (useful for CI)
- `--no-backup` - Don't create `.bak` backup files
- `--encoding <ENC>` - File encoding (default: `utf-8`)

**Examples:**

```bash
# Process a single file
mk-tools codeblocks README.md

# Process all Markdown files in a directory
mk-tools codeblocks docs/

# Process all Markdown files in current directory (default)
mk-tools codeblocks

# Process with custom root directory
mk-tools codeblocks --root src docs/

# Check if files are up-to-date (CI mode)
mk-tools codeblocks --check docs/

# Dry run to see what would change
mk-tools --dry-run codeblocks README.md
```

#### `toc` - Generate Table of Contents

Generate or update table of contents in Markdown files.

```bash
mk-tools toc [OPTIONS] [PATHS]...
```

**Options:**

- `--root <PATH>` - Base directory for resolving paths
- `--glob <PATTERN>` - Glob pattern for Markdown files (default: `**/*.md`)
- `--from-dir <DIR>` - Generate cross-file TOC from all files in this directory
- `--add` - Automatically add TOC markers below the first H1 heading if not present
- `--check` - Don't modify files; exit with error if updates are needed
- `--no-backup` - Don't create `.bak` backup files
- `--encoding <ENC>` - File encoding (default: `utf-8`)

**Examples:**

```bash
# Generate TOC in a single file
mk-tools toc README.md

# Update TOC in all documentation files
mk-tools toc docs/

# Process all Markdown files in current directory (default)
mk-tools toc

# Check if TOCs are up-to-date
mk-tools toc --check docs/

# Generate cross-file TOC in README.md from all files in docs/
mk-tools toc README.md --from-dir docs/

# Automatically add TOC markers to a file that doesn't have them yet
mk-tools toc docs/guide.md --add
```

#### `check` - Validate All

Run both codeblocks and toc validations without modifying files.

```bash
mk-tools check [OPTIONS] [PATHS]...
```

**Options:**

- `--glob <PATTERN>` - Glob pattern for Markdown files (default: `**/*.md`)
- `--root <PATH>` - Base directory for resolving paths

**Examples:**

```bash
# Check all Markdown files in current directory (default)
mk-tools check

# Check specific directory
mk-tools check docs/

# Check specific files
mk-tools check README.md CONTRIBUTING.md
```

## Marker Syntax

### Code Block Markers

Code block markers tell mk-tools which source file to sync into a Markdown code block.

**Basic syntax:**

```markdown
<!-- mk-code: <path> [options] -->
```

**Options:**

- `lang=<language>` - Override the code fence language
- `start=<n>` - Start at line number (1-based, inclusive)
- `end=<n>` - End at line number (1-based, inclusive)
- `dedent=<n>` - Remove N leading spaces from each line
- `region=<name>` - Named region (reserved for future use)

**Examples:**

```markdown
<!-- mk-code: ./src/main.rs -->

<!-- mk-code: ./src/main.rs lang=rust -->

<!-- mk-code: ./examples/demo.py start=10 end=40 -->

<!-- mk-code: ./lib/utils.js dedent=4 -->

<!-- mk-code: ../shared/config.toml lang=toml start=5 end=15 -->
```

### TOC Markers

TOC markers define regions where table of contents should be generated.

**Basic syntax:**

```markdown
<!-- mk-toc:start [options] -->
<!-- mk-toc:end -->
```

**Options:**

- `from-level=<n>` - Minimum heading level to include (default: 2)
- `to-level=<n>` - Maximum heading level to include (default: 6)
- `style=bullet|numbered` - List style (default: bullet)
- `root=<path>` - Base path for links (optional)

**Examples:**

```markdown
<!-- mk-toc:start -->
<!-- mk-toc:end -->

<!-- mk-toc:start from-level=1 to-level=3 -->
<!-- mk-toc:end -->

<!-- mk-toc:start style=numbered -->
<!-- mk-toc:end -->

<!-- mk-toc:start from-level=2 to-level=4 style=bullet -->
<!-- mk-toc:end -->
```

## CI/CD Integration

### GitHub Actions

Verify documentation is up-to-date in your CI pipeline:

```yaml
name: Docs Check

on:
  pull_request:
    paths:
      - "**.md"
      - "src/**"

jobs:
  check-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Download mk-tools
        run: |
          curl -L https://github.com/sbkg0002/mk-tools/releases/latest/download/mk-tools-x86_64-unknown-linux-gnu.tar.gz | tar xz
          sudo mv mk-tools /usr/local/bin/

      - name: Check documentation
        run: mk-tools check .
```

### Pre-commit Hook

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
set -e

echo "Checking Markdown documentation..."
mk-tools check . || {
    echo "Documentation is out of date. Run 'mk-tools codeblocks .' and 'mk-tools toc .'."
    exit 1
}
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

## Configuration

### Language Mappings

mk-tools automatically detects code fence languages based on file extensions. You can override these with `--language-overrides`:

```bash
mk-tools codeblocks --language-overrides "py=python3,jsx=javascript" docs/
```

Default mappings include:

- `rs` → `rust`
- `py` → `python`
- `js` → `javascript`
- `ts` → `typescript`
- `go` → `go`
- `java` → `java`
- `sh` → `bash`
- And many more...

## Examples

### Example 1: Documentation with Live Code Examples

Keep your documentation in sync with working examples:

**README.md:**

```markdown
# My Library

## Usage Example

Here's how to use the library:

<!-- mk-code: ./examples/basic.rs start=5 end=15 -->

## Configuration

The default configuration looks like this:

<!-- mk-code: ./examples/config.toml -->
```

Run: `mk-tools codeblocks` (or `mk-tools codeblocks README.md` for just that file)

### Example 2: Multi-file Documentation

**docs/README.md:**

```markdown
<!-- mk-toc:start from-level=2 -->
<!-- mk-toc:end -->

## API Reference

## Getting Started

## Examples
```

**docs/api.md:**

```markdown
<!-- mk-toc:start from-level=2 to-level=3 -->
<!-- mk-toc:end -->

## Functions

### authenticate()

### getData()
```

Run: `mk-tools toc` (processes all .md files in current directory and subdirectories)

### Example 3: Cross-File Table of Contents

Generate a single TOC that indexes multiple files:

**README.md:**

```markdown
# Project Documentation

<!-- mk-toc:start -->
<!-- mk-toc:end -->
```

**docs/api.md, docs/guide.md, etc.**

Run: `mk-tools toc README.md --from-dir docs/`

This generates a TOC in `README.md` with links to all headings in files within `docs/`:

```markdown
<!-- mk-toc:start -->

- [API Overview](docs/api.md#api-overview)
- [Authentication](docs/api.md#authentication)

- [Getting Started](docs/guide.md#getting-started)
- [Installation](docs/guide.md#installation)
<!-- mk-toc:end -->
```

### Example 4: Quick TOC Setup with --add

Automatically add TOC markers to an existing file:

**Before (guide.md):**

```markdown
# User Guide

Introduction to the system.

## Getting Started

Instructions here.

## Advanced Topics

More details.
```

Run: `mk-tools toc guide.md --add`

**After (guide.md):**

```markdown
# User Guide

<!-- mk-toc:start -->

- [Getting Started](#getting-started)
- [Advanced Topics](#advanced-topics)
<!-- mk-toc:end -->

Introduction to the system.

## Getting Started

Instructions here.

## Advanced Topics

More details.
```

The `--add` option:

- Detects if TOC markers are missing
- Automatically inserts them after the first H1 heading
- If no H1 heading exists, inserts at the beginning
- Generates the TOC immediately
- Does nothing if markers already exist

### Example 5: Partial Code Extraction

Extract just the important parts of a file:

**tutorial.md:**

```markdown
Here's the key algorithm:

<!-- mk-code: ./src/algorithm.rs start=45 end=67 dedent=4 -->
```

This extracts lines 45-67 from `algorithm.rs` and removes 4 spaces of indentation.

## Troubleshooting

### "Failed to read source file"

- Ensure the path in your `mk-code` marker is correct relative to the Markdown file or `--root`
- Check file permissions
- Verify the file exists

### "Found mk-toc:start without matching mk-toc:end"

- Ensure every `mk-toc:start` has a corresponding `mk-toc:end`
- Check for typos in the marker comments

### Encoding Issues

Currently, only UTF-8 encoding is supported. Ensure your files are UTF-8 encoded.

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Verbose Logging

```bash
mk-tools -vv codeblocks README.md
```

Or set the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug mk-tools codeblocks README.md
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [regex](https://github.com/rust-lang/regex) for pattern matching
- Inspired by various documentation tools in the Rust ecosystem
