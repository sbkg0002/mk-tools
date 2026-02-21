# Automatic TOC Marker Insertion Example

This example demonstrates the `--add` option for the `toc` command, which automatically inserts TOC markers into Markdown files.

## What is --add?

The `--add` option tells mk-tools to:

1. Check if a file has TOC markers (`<!-- mk-toc:start -->` / `<!-- mk-toc:end -->`)
2. If not, automatically insert them
3. Place them below the first H1 heading (or at the beginning if no H1 exists)
4. Generate the TOC immediately

## Use Cases

### Quick Setup for New Files

You've just created a new documentation file and want to add a TOC:

```bash
mk-tools toc new-guide.md --add
```

### Batch Processing Existing Files

Add TOCs to multiple files at once:

```bash
mk-tools toc docs/*.md --add
```

### Safe to Run Multiple Times

The `--add` option is idempotent - if markers already exist, nothing happens:

```bash
# First run: adds markers and generates TOC
mk-tools toc guide.md --add

# Second run: no changes, markers already exist
mk-tools toc guide.md --add
```

## Example Walkthrough

### Step 1: Starting File

You have a file without any TOC markers:

**my-guide.md:**
```markdown
# User Guide

Welcome to our comprehensive user guide. This document covers everything you need to know.

## Installation

Instructions for installing the software.

### Windows Installation

Steps for Windows users.

### macOS Installation

Steps for macOS users.

## Configuration

How to configure the software.

## Troubleshooting

Common issues and solutions.
```

### Step 2: Run mk-tools with --add

```bash
mk-tools toc my-guide.md --add
```

Output:
```
[INFO] Running toc command
[INFO] Processing 1 Markdown file(s)
[INFO] Adding TOC markers to my-guide.md
[INFO] Updated: my-guide.md
[INFO] Summary: 1 file(s) changed, 0 file(s) with errors
```

### Step 3: Result

The file now has TOC markers and a generated TOC:

**my-guide.md:**
```markdown
# User Guide

<!-- mk-toc:start -->
- [Installation](#installation)
  - [Windows Installation](#windows-installation)
  - [macOS Installation](#macos-installation)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)
<!-- mk-toc:end -->

Welcome to our comprehensive user guide. This document covers everything you need to know.

## Installation

Instructions for installing the software.

### Windows Installation

Steps for Windows users.

### macOS Installation

Steps for macOS users.

## Configuration

How to configure the software.

## Troubleshooting

Common issues and solutions.
```

## Advanced Usage

### Combining with Other Options

You can combine `--add` with other TOC options:

```bash
# Add markers and use custom heading levels
mk-tools toc guide.md --add
# Then customize in the file:
# <!-- mk-toc:start from-level=1 to-level=3 -->
```

### Files Without H1 Heading

If a file doesn't start with an H1 heading, markers are added at the beginning:

**Before (api-notes.md):**
```markdown
Some preliminary notes about the API.

## Authentication

How to authenticate.
```

After running `mk-tools toc api-notes.md --add`:

**After (api-notes.md):**
```markdown
<!-- mk-toc:start -->
- [Authentication](#authentication)
<!-- mk-toc:end -->

Some preliminary notes about the API.

## Authentication

How to authenticate.
```

### Batch Processing with Glob Patterns

Add TOCs to all markdown files in a directory:

```bash
# Current directory
mk-tools toc --add

# Specific directory
mk-tools toc docs/ --add

# Only top-level files (not subdirectories)
mk-tools toc docs/*.md --add

# All files recursively (default)
mk-tools toc docs/ --add --glob "**/*.md"
```

### Dry Run Mode

Preview what would happen without making changes:

```bash
mk-tools --dry-run toc guide.md --add
```

Output shows which files would be modified without actually modifying them.

## Best Practices

### 1. Always Start with H1

Structure your documents with a clear H1 heading at the top:

```markdown
# Document Title

<!-- TOC will be inserted here -->

Content...
```

### 2. Run on New Files

Make it part of your workflow when creating new documentation:

```bash
# Create new file
cat > new-doc.md << 'EOF'
# New Documentation

Introduction text.

## Section 1
...
EOF

# Add TOC
mk-tools toc new-doc.md --add
```

### 3. Safe Batch Updates

It's safe to run on entire directories - existing TOCs won't be duplicated:

```bash
# Add TOCs to any files that don't have them
mk-tools toc docs/ --add
```

### 4. Customize After Adding

After markers are added, you can customize the options:

```markdown
<!-- mk-toc:start from-level=2 to-level=4 style=numbered -->
<!-- mk-toc:end -->
```

Then run again to regenerate with new options:

```bash
mk-tools toc guide.md
```

## Comparison: Manual vs --add

### Manual Method

1. Open file in editor
2. Find the right location after H1
3. Type `<!-- mk-toc:start -->`
4. Type `<!-- mk-toc:end -->`
5. Save file
6. Run `mk-tools toc file.md`

### With --add

1. Run `mk-tools toc file.md --add`

That's it! One command does everything.

## Integration with CI/CD

You can use `--add` in CI to ensure all documentation files have TOCs:

```yaml
# .github/workflows/docs.yml
name: Ensure TOCs

on:
  pull_request:
    paths:
      - 'docs/**/*.md'

jobs:
  check-tocs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install mk-tools
        run: |
          curl -L https://github.com/sbkg0002/mk-tools/releases/latest/download/mk-tools-x86_64-unknown-linux-gnu.tar.gz | tar xz
          sudo mv mk-tools /usr/local/bin/
      
      - name: Add TOCs if missing
        run: mk-tools toc docs/ --add
      
      - name: Check if any files changed
        run: |
          if [[ -n $(git status -s) ]]; then
            echo "Some files are missing TOC markers:"
            git status -s
            exit 1
          fi
```

## Troubleshooting

### Markers Added in Wrong Place

If markers aren't where you want them, you can:

1. Remove them manually
2. Move your H1 heading to the desired location
3. Run `mk-tools toc file.md --add` again

### Multiple H1 Headings

Only the *first* H1 heading is used for marker placement. If you have multiple H1s:

- Consider using H2 for subsequent sections
- Or manually place markers where desired

### No Changes Happening

If running with `--add` doesn't add markers:

- Check if markers already exist (they won't be duplicated)
- Check file permissions
- Use `-v` flag for verbose output: `mk-tools toc file.md --add -v`

## Summary

The `--add` option is a time-saver for:

- ✅ Quickly setting up TOCs in new files
- ✅ Batch processing multiple files
- ✅ Ensuring all documentation has TOCs
- ✅ Automating TOC setup in CI/CD

It's safe, idempotent, and smart about placement. Try it out:

```bash
mk-tools toc YOUR_FILE.md --add
```
