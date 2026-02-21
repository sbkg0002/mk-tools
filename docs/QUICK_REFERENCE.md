# mk-tools Quick Reference

## Installation

```bash
# Linux x86_64
curl -L https://github.com/sbkg0002/mk-tools/releases/latest/download/mk-tools-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv mk-tools /usr/local/bin/

# macOS ARM64
curl -L https://github.com/sbkg0002/mk-tools/releases/latest/download/mk-tools-aarch64-apple-darwin.tar.gz | tar xz
sudo mv mk-tools /usr/local/bin/
```

## Commands

| Command                       | Description                          |
| ----------------------------- | ------------------------------------ |
| `mk-tools codeblocks [FILES]` | Sync code blocks with source files   |
| `mk-tools toc [FILES]`        | Generate/update table of contents    |
| `mk-tools check [FILES]`      | Validate without modifying (CI mode) |
| `mk-tools version`            | Show version information             |

## Global Options

| Option           | Short | Description                                    |
| ---------------- | ----- | ---------------------------------------------- |
| `--quiet`        | `-q`  | Reduce output verbosity                        |
| `--verbose`      | `-v`  | Increase verbosity (repeatable: -v, -vv, -vvv) |
| `--dry-run`      |       | Preview changes without writing                |
| `--color <when>` |       | Control colored output (auto/always/never)     |
| `--chdir <dir>`  | `-C`  | Change working directory                       |

## Codeblock Command Options

| Option                       | Description                             | Example                                  |
| ---------------------------- | --------------------------------------- | ---------------------------------------- |
| `--root <path>`              | Base directory for resolving paths      | `--root src`                             |
| `--glob <pattern>`           | File pattern for directories            | `--glob "*.md"`                          |
| `--language-overrides <map>` | Custom language mappings                | `--language-overrides py=python,rs=rust` |
| `--check`                    | Don't modify, exit non-zero if outdated | `--check`                                |
| `--no-backup`                | Don't create .bak files                 | `--no-backup`                            |

## TOC Command Options

| Option              | Description                                     | Example            |
| ------------------- | ----------------------------------------------- | ------------------ |
| `--root <path>`     | Base directory for paths                        | `--root docs`      |
| `--glob <pattern>`  | File pattern for directories                    | `--glob "**/*.md"` |
| `--from-dir <path>` | Generate cross-file TOC from files in directory | `--from-dir docs/` |
| `--add`             | Automatically add TOC markers if not present    | `--add`            |
| `--check`           | Don't modify, exit non-zero if outdated         | `--check`          |
| `--no-backup`       | Don't create .bak files                         | `--no-backup`      |

## Codeblock Marker Syntax

### Basic Format

```markdown
<!-- mk-code: <path> [options] -->
```

### Options

| Option            | Description                     | Example          |
| ----------------- | ------------------------------- | ---------------- |
| `lang=<language>` | Override code fence language    | `lang=rust`      |
| `start=<n>`       | Start line (1-based, inclusive) | `start=10`       |
| `end=<n>`         | End line (1-based, inclusive)   | `end=25`         |
| `dedent=<n>`      | Remove N leading spaces         | `dedent=4`       |
| `region=<name>`   | Named region (reserved)         | `region=example` |

### Examples

```markdown
<!-- Full file -->
<!-- mk-code: ./src/main.rs -->

<!-- With language override -->
<!-- mk-code: ./src/main.rs lang=rust -->

<!-- Line range -->
<!-- mk-code: ./lib/utils.js start=15 end=30 -->

<!-- With dedenting -->
<!-- mk-code: ./examples/demo.py start=5 end=20 dedent=4 -->

<!-- Multiple options -->
<!-- mk-code: ./config.toml lang=toml start=1 end=10 -->
```

## TOC Marker Syntax

### Basic Format

```markdown
<!-- mk-toc:start [options] -->
<!-- mk-toc:end -->
```

### Options

| Option                   | Description                      | Default |
| ------------------------ | -------------------------------- | ------- |
| `from-level=<n>`         | Minimum heading level to include | 2       |
| `to-level=<n>`           | Maximum heading level to include | 6       |
| `style=bullet\|numbered` | List style                       | bullet  |
| `root=<path>`            | Base path for links (optional)   | -       |

### Examples

```markdown
<!-- Default settings (levels 2-6, bullet style) -->
<!-- mk-toc:start -->
<!-- mk-toc:end -->

<!-- Include level 1 headings -->
<!-- mk-toc:start from-level=1 -->
<!-- mk-toc:end -->

<!-- Limit depth -->
<!-- mk-toc:start from-level=2 to-level=3 -->
<!-- mk-toc:end -->

<!-- Numbered list -->
<!-- mk-toc:start style=numbered -->
<!-- mk-toc:end -->

<!-- All options -->
<!-- mk-toc:start from-level=1 to-level=4 style=bullet -->
<!-- mk-toc:end -->
```

## Common Use Cases

### Sync all documentation

```bash
# Process specific directory
mk-tools codeblocks docs/
mk-tools toc docs/

# Process current directory and subdirectories (default)
mk-tools codeblocks
mk-tools toc
```

### Check in CI

```bash
# Check current directory (default)
mk-tools check || exit 1

# Or explicitly check current directory
mk-tools check . || exit 1
```

### Preview changes

```bash
mk-tools --dry-run codeblocks README.md
```

### Verbose output for debugging

```bash
mk-tools -vv codeblocks docs/
```

### Process all files in current directory

```bash
# Default behavior - processes all .md files recursively
mk-tools codeblocks
mk-tools toc
```

### Process specific files

```bash
mk-tools codeblocks README.md CONTRIBUTING.md
```

### Custom root directory

```bash
mk-tools codeblocks --root src docs/
```

### Generate cross-file TOC

```bash
# Create a TOC in README.md from all files in docs/
mk-tools toc README.md --from-dir docs/

# This will generate links like:
# - [Heading](docs/file1.md#heading)
# - [Another](docs/file2.md#another)
```

### Automatically add TOC markers

```bash
# Add TOC markers to a file that doesn't have them
mk-tools toc guide.md --add

# Markers are inserted below the first H1 heading
# If no H1 exists, inserted at the beginning
# Then TOC is generated immediately
```

## Language Mappings (Default)

| Extension                     | Language   |
| ----------------------------- | ---------- |
| `.rs`                         | rust       |
| `.py`                         | python     |
| `.js`                         | javascript |
| `.ts`                         | typescript |
| `.go`                         | go         |
| `.java`                       | java       |
| `.c`, `.h`                    | c          |
| `.cpp`, `.cc`, `.cxx`, `.hpp` | cpp        |
| `.sh`, `.bash`                | bash       |
| `.rb`                         | ruby       |
| `.php`                        | php        |
| `.cs`                         | csharp     |
| `.swift`                      | swift      |
| `.kt`                         | kotlin     |
| `.scala`                      | scala      |
| `.md`                         | markdown   |
| `.json`                       | json       |
| `.yaml`, `.yml`               | yaml       |
| `.toml`                       | toml       |
| `.xml`                        | xml        |
| `.html`, `.htm`               | html       |
| `.css`                        | css        |

Override with: `--language-overrides ext=lang,ext2=lang2`

## Exit Codes

| Code | Meaning                                |
| ---- | -------------------------------------- |
| 0    | Success, no changes needed             |
| 1    | Files need updates (in `--check` mode) |
| >1   | Error occurred (I/O, parsing, etc.)    |

## GitHub Actions Example

```yaml
name: Check Documentation

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
        run: mk-tools check
```

## Troubleshooting

| Issue                                            | Solution                                              |
| ------------------------------------------------ | ----------------------------------------------------- |
| "Failed to read source file"                     | Check path is correct relative to MD file or `--root` |
| "Found mk-toc:start without matching mk-toc:end" | Ensure markers are paired correctly                   |
| "Only UTF-8 encoding is currently supported"     | Convert files to UTF-8 encoding                       |
| Code block not updating                          | Verify marker syntax, check file exists               |
| TOC not generating                               | Check heading levels match `from-level`/`to-level`    |

## Environment Variables

| Variable   | Description                                       |
| ---------- | ------------------------------------------------- |
| `RUST_LOG` | Override logging level (debug, info, warn, error) |

Example:

```bash
RUST_LOG=debug mk-tools codeblocks README.md
```

## Tips & Best Practices

1. **Always commit before running** to easily revert changes if needed
2. **Use `--dry-run`** to preview changes before applying
3. **Enable backups** (default) for safety, use `--no-backup` only when confident
4. **Add to CI** to ensure docs stay up-to-date
5. **Use relative paths** in markers for portability
6. **Test markers** on small files first before processing large documentation sets
7. **Keep source files clean** - avoid mixing documentation and complex code in referenced files
8. **Run without arguments** - Commands default to current directory for convenience (`mk-tools toc`)
9. **Use --add for quick setup** - Quickly add TOC markers to files: `mk-tools toc *.md --add`

## Version Information

```bash
mk-tools version
```

## Get Help

```bash
# General help
mk-tools --help

# Command-specific help
mk-tools codeblocks --help
mk-tools toc --help
mk-tools check --help
```

---

**Full Documentation**: https://github.com/sbkg0002/mk-tools
**Report Issues**: https://github.com/sbkg0002/mk-tools/issues
