# mk-tools Specification

## 1. Overview

`mk-tools` is a Rust-based command-line tool for managing Markdown files in a code repository.

Primary capabilities:

1. **Code block synchronization**  
   - Update fenced code blocks in Markdown based on special HTML comments that refer to source files.
   - Ensure that embedded code snippets stay in sync with the actual source files.

2. **Table of contents (TOC) generation**  
   - Insert or update a table of contents in one or more Markdown files, based on a marker comment.
   - Can be applied across multiple directories.

3. **CI / GitHub integration**  
   - Designed to run in CI to verify that Markdown is up to date.
   - GitHub Actions workflow to build and release binaries for:
     - `aarch64-apple-darwin` (darwin arm64)
     - `x86_64-unknown-linux-gnu` (linux x86_64)

---

## 2. CLI Overview

### 2.1 Binary name

- Binary: `mk-tools`

### 2.2 Top-level CLI structure

```/dev/null/spec-cli.md#L1-60
mk-tools [GLOBAL_OPTIONS] <COMMAND> [ARGS...]

GLOBAL OPTIONS:
  -q, --quiet                 Less output
  -v, --verbose ...           Increase output verbosity (can be repeated)
      --dry-run               Do not write files, only report actions
      --color <when>          Color output: auto|always|never
  -C, --chdir <dir>           Change working directory before running

COMMANDS:
  codeblocks                  Sync code blocks in Markdown from source files
  toc                         Generate/update table of contents in Markdown
  check                       Run validations (codeblocks + toc) in check mode
  version                     Print version information
  help                        Show help for commands
```

The CLI will be implemented using `clap` (derive-based).

---

## 3. Code Blocks Synchronization (`codeblocks`)

### 3.1 Goals

- Keep code blocks in `.md` files synchronized with their source files.
- Use an HTML comment in Markdown to specify which source file (and optional range/options) drives a particular fenced code block.
- Allow partial extraction (by line) and basic formatting options.

### 3.2 Marker syntax

Code block regions are driven by a marker comment placed just above the code block.

**Marker format:**

```/dev/null/spec-codeblocks.md#L1-80
<!-- mk-code: <path> [options...] -->

<path>:
  - A path to a source file, typically relative to:
      - the Markdown file's directory, or
      - an explicit root specified via CLI (e.g. `--root`).
  - Examples: `./src/main.rs`, `../examples/app.py`, `scripts/tool.sh`

Options:
  lang=<language>      Override code fence language (e.g., `lang=python`)
  start=<n>            1-based inclusive line number to start extracting from
  end=<m>              1-based inclusive line number to stop extracting at
  dedent=<n>           Remove `n` leading spaces from each extracted line
  region=<name>        (Reserved for future named-region support, no-op for now)

Examples:
  <!-- mk-code: ./src/main.rs -->
  <!-- mk-code: ./src/main.rs lang=rust -->
  <!-- mk-code: ./scripts/example.py start=10 end=40 lang=python -->
  <!-- mk-code: ../lib/demo.sh dedent=2 -->
```

### 3.3 Markdown code block behavior

Following each `mk-code` marker, the tool expects either:

1. An existing fenced code block, which will be replaced; or  
2. No fenced block, in which case a new one will be inserted.

Supported fenced blocks:

- Backtick fences (preferred and default): ` ``` `
- Tilde fences may be supported later; initial implementation can focus on backticks.

**Example 1: Marker with automatically inserted block**

```/dev/null/spec-codeblocks.md#L82-128
<!-- mk-code: ./src/main.rs lang=rust -->

<!-- After running mk-tools codeblocks, this becomes: -->

<!-- mk-code: ./src/main.rs lang=rust -->
```rust
// Content of src/main.rs...
```
```

**Example 2: Marker with existing block that gets replaced**

```/dev/null/spec-codeblocks.md#L130-184
<!-- mk-code: ./src/main.rs lang=rust -->
```rust
// Old content – will be replaced
```

<!-- After running mk-tools codeblocks: -->

<!-- mk-code: ./src/main.rs lang=rust -->
```rust
// Up-to-date content from src/main.rs...
```
```

### 3.4 Fence language resolution

The language in the fence is determined by:

1. `lang=<language>` option in the marker, if present.
2. Otherwise, using a mapping based on the source file extension:
   - E.g. `.rs -> rust`, `.py -> python`, `.sh -> sh`, `.ts -> typescript`, etc.
   - Custom overrides can be passed via CLI: `--language-overrides py=python,rs=rust`.
3. If nothing is decided, the fence is created without a language identifier: ` ``` `.

### 3.5 Command-line interface for `codeblocks`

```/dev/null/spec-codeblocks-cli.md#L1-120
Usage:
  mk-tools codeblocks [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...    Files or directories to process.
                - If a file: must be a Markdown file (typically *.md).
                - If a directory: recursively process Markdown files matching
                  `--glob` within that directory.

Options:
      --root <path>               Base directory for resolving marker paths.
                                  Default: current directory (after `--chdir`).
      --glob <pattern>            Glob pattern for Markdown when PATHS
                                  include directories (default: `**/*.md`).
      --language-overrides <map>  Comma-separated list of ext=lang mappings.
                                  Example: `py=python,rs=rust`.
      --check                     Do NOT modify files; exit non-zero if any
                                  changes would be applied.
      --no-backup                 Do not write backup files (e.g. *.bak).
      --encoding <encoding>       File encoding for Markdown files.
                                  Initial support: UTF-8 only; non-UTF-8
                                  should error clearly.

Behavior:
  - Discover Markdown files from PATHS / glob.
  - For each Markdown file:
      - Parse content and find all `<!-- mk-code: ... -->` markers.
      - For each marker:
          - Parse the path and options.
          - Resolve the source file path relative to `--root` or the Markdown
            file's directory.
          - Read the source file content.
          - Apply slicing (start/end) and dedenting if specified.
          - Determine the fence language and fence style.
          - Locate an existing code fence immediately below the marker, if any.
          - Insert or replace the fenced code block.
      - If changes would be made:
          - With `--check`: do not write, but record that changes are needed.
          - Without `--check`:
              - Optionally create a backup (unless `--no-backup`).
              - Write the updated Markdown back to disk.
  - Exit codes:
      - 0: Success, no outstanding updates.
      - 1: `--check` mode detected outdated content.
      - >1: Errors (I/O, parse failures, etc.).
```

---

## 4. Table of Contents (`toc`)

### 4.1 Goals

- Automatically insert or update a table of contents (TOC) in Markdown files.
- TOC regions are explicitly marked in the Markdown so they can be regenerated repeatedly.
- Support customization of the heading levels included and list style.

### 4.2 TOC marker syntax

TOC regions are delimited by a start and end comment.

**Basic form:**

```/dev/null/spec-toc.md#L1-80
<!-- mk-toc:start [options...] -->
<!-- mk-toc:end -->
```

Everything between the start and end markers is considered managed content and will be replaced by `mk-tools`.

**Supported options in start marker:**

- `from-level=<n>` – minimum heading level to include (default: 2).
- `to-level=<n>` – maximum heading level to include (default: 6).
- `style=bullet|numbered` – list style (default: `bullet`).
- `root=<path>` – override base path for links (optional; mostly relevant for multi-file or cross-file TOCs in future extensions).

**Example:**

```/dev/null/spec-toc.md#L82-154
<!-- mk-toc:start from-level=2 to-level=4 style=bullet -->
<!-- mk-toc:end -->

After running `mk-tools toc`, the file might become:

<!-- mk-toc:start from-level=2 to-level=4 style=bullet -->
- [Introduction](#introduction)
- [Usage](#usage)
  - [Installation](#installation)
  - [Examples](#examples)
- [Internals](#internals)
<!-- mk-toc:end -->
```

### 4.3 Heading detection

Headings are detected using standard ATX Markdown headings:

- Lines starting with `#` characters followed by a space.
- Example: `## Usage`, `### Advanced Usage`.

Rules:

- Only lines matching regex like `^(#{1,6})\s+(.+?)\s*$` are considered.
- `level` = number of `#` characters.
- `text` = trimmed heading text (without trailing `#` if present).
- Heading inclusion is filtered via `from-level` and `to-level`.

### 4.4 Anchor generation

Anchors should be GitHub-like, so that `(#anchor)` links work in GitHub, GitLab, etc.

Basic algorithm:

1. Convert heading text to lowercase.
2. Remove leading and trailing whitespace.
3. Replace spaces with `-`.
4. Remove or normalize most punctuation characters.
5. Collapse multiple `-` into a single `-`.
6. If an anchor would duplicate an earlier one, append `-1`, `-2`, etc. for uniqueness (GitHub-style).

Example:

- `"Introduction"` → `introduction`
- `"Advanced Usage!"` → `advanced-usage`
- `"API v2.0"` → `api-v20`

### 4.5 List structure

The TOC is a nested list:

- Each heading becomes a list item linking to `#anchor`.
- Depth of nesting is derived from the heading level:
  - Indentation per level can be 2 spaces or 4 spaces (choose 2 for brevity).
  - Example: Level 2 heading is top-level, level 3 heading is indented under it, etc.

Example (bullet style):

```/dev/null/spec-toc.md#L156-196
- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Configuration](#configuration)
- [Reference](#reference)
  - [CLI](#cli)
  - [Configuration File](#configuration-file)
```

For numbered style, use:

```/dev/null/spec-toc.md#L198-230
1. [Overview](#overview)
1. [Getting Started](#getting-started)
   1. [Installation](#installation)
   1. [Configuration](#configuration)
```

(All items can be `1.` in Markdown; the renderer will renumber them.)

### 4.6 Command-line interface for `toc`

```/dev/null/spec-toc-cli.md#L1-120
Usage:
  mk-tools toc [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...    Files or directories to process.
                - If a file: must be *.md.
                - If a directory: recursively process Markdown files matching
                  `--glob`.

Options:
      --root <path>          Base directory for path-related logic (reserved
                             for future cross-file TOCs). For now mostly
                             informational.
      --glob <pattern>       Glob pattern for Markdown when PATHS include
                             directories (default: `**/*.md`).
      --check                Do NOT modify files; exit non-zero if any TOC
                             region would change.
      --no-backup            Do not write backup files.
      --encoding <encoding>  File encoding (UTF-8 only for now).

Behavior:
  - For each Markdown file:
      - Find all pairs of `<!-- mk-toc:start ... -->` and `<!-- mk-toc:end -->`.
        - If a start has no matching end (or vice versa), treat as an error.
      - For each TOC region:
          - Parse options from the start marker.
          - Scan the entire Markdown file for headings.
          - Filter headings by `from-level` / `to-level`.
          - Generate a nested list based on heading levels and `style`.
          - Replace the content between `mk-toc:start` and `mk-toc:end`
            with the newly generated TOC.
      - With `--check`: do not write; track whether any changes would occur.
      - Without `--check`:
          - Optionally create backup (unless `--no-backup`).
          - Write the updated file.

Exit codes:
  - 0: Success, no changes needed (or successfully written).
  - 1: `--check` mode detected one or more outdated TOCs.
  - >1: Errors (I/O, parse issues, unpaired markers, etc.).
```

---

## 5. `check` Command

### 5.1 Goals

- Provide a single command for CI to verify that Markdown files are consistent:
  - `codeblocks` are up to date.
  - `toc` regions are up to date.

### 5.2 Command-line interface

```/dev/null/spec-check-cli.md#L1-120
Usage:
  mk-tools check [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...    Files or directories to process, same semantics as for
                `codeblocks` and `toc`.

Options:
      --glob <pattern>   Glob for Markdown within directories (default: `**/*.md`).
      --root <path>      Base directory for resolving marker paths.

Behavior:
  - Internally runs `codeblocks` and `toc` in `--check` mode over the same
    set of discovered files.
  - Reports a combined result:
      - If either `codeblocks` or `toc` would make changes, `check` exits
        with status code 1.
      - If both are clean, `check` exits with status 0.
      - If errors occur in either component, exit with >1.
```

---

## 6. Internal Architecture

### 6.1 Crate layout

Proposed structure:

```/dev/null/spec-architecture.md#L1-80
src/
  main.rs           // program entrypoint
  cli.rs            // clap-based argument parsing
  logging.rs        // logging setup (env_logger or similar)
  fs/
    mod.rs          // common filesystem utilities (read/write/backup)
    path_utils.rs   // path resolution, root handling
  markdown/
    mod.rs
    codeblocks.rs   // codeblock parsing & transformation
    toc.rs          // TOC parsing & transformation
  domain/
    codeblock.rs    // domain types related to codeblocks
    toc.rs          // domain types related to TOCs
```

### 6.2 Domain models (high-level)

**Codeblocks:**

```/dev/null/spec-architecture.md#L82-160
struct CodeblockSpec {
    marker_span: TextSpan,             // Byte offsets of the marker comment
    marker_line: usize,                // Line number where the marker is
    source_path: std::path::PathBuf,   // Resolved source file path
    lang: Option<String>,              // Fence language (explicit or inferred)
    line_range: Option<LineRange>,     // Optional start/end from marker
    dedent: Option<usize>,             // Dedent value from marker
    existing_block_span: Option<TextSpan>, // Span of existing fenced block
}

struct TextSpan {
    start: usize,
    end: usize,
}

struct LineRange {
    start: usize, // 1-based inclusive
    end: usize,   // 1-based inclusive
}
```

**TOC:**

```/dev/null/spec-architecture.md#L162-246
struct TocRegionSpec {
    start_span: TextSpan,   // Span of mk-toc:start comment
    end_span: TextSpan,     // Span of mk-toc:end comment
    options: TocOptions,    // Parsed from the start comment
}

struct TocOptions {
    from_level: u8,         // default: 2
    to_level: u8,           // default: 6
    style: TocStyle,        // Bullet or Numbered
    root: Option<PathBuf>,  // Optional base path
}

enum TocStyle {
    Bullet,
    Numbered,
}

struct Heading {
    level: u8,         // 1-6
    text: String,      // Raw heading text
    anchor: String,    // Generated anchor
    line: usize,       // Line number in file
}
```

### 6.3 Processing algorithms

**Codeblocks (per file):**

1. Read the entire Markdown content as a string.
2. Find all `mk-code` markers using a parser or regex.
3. For each marker:
   - Parse path and options.
   - Locate the following fenced code block (if any).
   - Construct `CodeblockSpec`.
4. For each `CodeblockSpec`:
   - Resolve the source file path.
   - Read the source file (UTF-8).
   - Extract lines as per `line_range` and apply `dedent`.
   - Build a new fenced code block text with the correct language.
5. Apply changes in reverse document order by replacing existing spans (or inserting new ones after markers).
6. If `--check`:
   - Compare final content to original; if different, mark file as needing update.
7. Else:
   - Optionally write a backup file.
   - Write modified content.

**TOC (per file):**

1. Read the entire Markdown content.
2. Find all `mk-toc:start` / `mk-toc:end` pairs; report error if unpaired.
3. Parse options from each `mk-toc:start`.
4. Scan the full document for headings and build a list of `Heading` entries.
5. For each TOC region:
   - Filter headings by `from-level` and `to-level`.
   - Generate a nested list string with anchors as links.
   - Replace the content between `start_span.end` and `end_span.start` with the generated TOC.
6. In `--check` mode, compare and note differences but do not write.
7. Otherwise, write backups and updated files.

---

## 7. GitHub Actions Build & Release

### 7.1 Goals

- Automatically build and publish releases when tags are pushed (e.g. `v1.0.0`).
- Build static or standard binaries for:
  - `aarch64-apple-darwin` (darwin arm64)
  - `x86_64-unknown-linux-gnu` (linux x86_64)
- Attach built binaries to the GitHub Release.

### 7.2 Workflow: `.github/workflows/release.yml`

The repository will contain a workflow similar to:

```/.github/workflows/release.yml#L1-200
name: Release

on:
  push:
    tags:
      - "v*.*.*"

permissions:
  contents: write

jobs:
  build:
    name: Build binaries
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: aarch64-apple-darwin
            os: macos-latest

    runs-on: ${{ matrix.os }}

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@v1
        with:
          toolchain: stable
          targets: ${{ matrix.target }}

      - name: Build (release)
        run: cargo build --release --target ${{ matrix.target }}

      - name: Prepare artifacts
        run: |
          mkdir -p dist
          BIN_NAME="mk-tools"
          TARGET="${{ matrix.target }}"
          cp "target/${TARGET}/release/${BIN_NAME}" "dist/${BIN_NAME}-${TARGET}"
          ls -l dist

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: mk-tools-${{ matrix.target }}
          path: dist/*

  release:
    name: Create GitHub Release
    needs: build
    runs-on: ubuntu-latest

    steps:
      - name: Download artifacts
        uses: actions/download-artifact@v4
        with:
          path: dist

      - name: List artifacts
        run: ls -R dist

      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            dist/**/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

Behavior:

- On push of a tag `vX.Y.Z`:
  - Both targets are built.
  - Resulting binaries are named:
    - `mk-tools-x86_64-unknown-linux-gnu`
    - `mk-tools-aarch64-apple-darwin`
  - Binaries are attached to the GitHub Release for that tag.

---

## 8. Non-Goals / Future Extensions

Non-goals for the initial version (may be added later):

- Named regions within source files (e.g. `// region: example` / `// endregion`) mapped via `region=<name>`.
- Support for non-UTF-8 encodings beyond a clear error.
- Advanced Markdown parsers for edge-case syntax (initial implementation can rely on line-based parsing and simple regexes).
- Cross-file TOCs (e.g., a single TOC that indexes headings across multiple Markdown files).

These can be revisited once the core `codeblocks` and `toc` functionality is stable.
