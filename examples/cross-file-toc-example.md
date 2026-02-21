# Cross-File Table of Contents Example

This example demonstrates how to use mk-tools to generate a table of contents in one file (like README.md) that includes headings from multiple files in a subdirectory.

## Use Case

You have a documentation directory with multiple Markdown files, and you want a master index in your README that links to all the content across those files.

## Directory Structure

```
project/
├── README.md                 # Master index file
└── docs/
    ├── getting-started.md    # Documentation file 1
    ├── api-reference.md      # Documentation file 2
    └── advanced-guide.md     # Documentation file 3
```

## Step 1: Add TOC Marker to Master File

In your `README.md`, add the TOC marker where you want the cross-file table of contents:

```markdown
# My Project

Welcome to my project!

## Documentation Index

<!-- mk-toc:start -->
<!-- mk-toc:end -->

## Quick Start

...rest of your README...
```

## Step 2: Create Your Documentation Files

**docs/getting-started.md:**
```markdown
# Getting Started

## Installation

Instructions for installing the project.

## Quick Start Guide

Get up and running quickly.

### Prerequisites

What you need before starting.
```

**docs/api-reference.md:**
```markdown
# API Reference

## Authentication

How to authenticate API requests.

## Core Methods

### getData()

Retrieve data from the API.

### postData()

Send data to the API.
```

**docs/advanced-guide.md:**
```markdown
# Advanced Guide

## Performance Optimization

Tips for optimizing performance.

## Security Best Practices

Security recommendations.
```

## Step 3: Generate Cross-File TOC

Run mk-tools with the `--from-dir` option:

```bash
mk-tools toc README.md --from-dir docs/
```

Or if you want to customize the heading levels:

```bash
# Only include headings from level 2 to 3
mk-tools toc README.md --from-dir docs/
```

Note: You can still use TOC options in the marker comment itself:
```markdown
<!-- mk-toc:start from-level=2 to-level=3 -->
<!-- mk-toc:end -->
```

## Result

Your `README.md` will now contain:

```markdown
# My Project

Welcome to my project!

## Documentation Index

<!-- mk-toc:start -->
- [Installation](docs/getting-started.md#installation)
- [Quick Start Guide](docs/getting-started.md#quick-start-guide)
  - [Prerequisites](docs/getting-started.md#prerequisites)

- [Authentication](docs/api-reference.md#authentication)
- [Core Methods](docs/api-reference.md#core-methods)
  - [getData()](docs/api-reference.md#getdata)
  - [postData()](docs/api-reference.md#postdata)

- [Performance Optimization](docs/advanced-guide.md#performance-optimization)
- [Security Best Practices](docs/advanced-guide.md#security-best-practices)
<!-- mk-toc:end -->

## Quick Start

...rest of your README...
```

## Features

### Automatic File Grouping

Notice how the TOC automatically groups headings by source file and adds blank lines between files for better readability.

### Relative Path Links

All links are generated relative to the target file (README.md), making them work correctly in GitHub, GitLab, and other platforms:
- `[Heading](docs/file.md#heading)` - relative to README.md location

### Customizable Heading Levels

Use the marker options to control which heading levels are included:

```markdown
<!-- mk-toc:start from-level=1 to-level=4 -->
<!-- mk-toc:end -->
```

### Numbered Lists

You can also use numbered list style:

```markdown
<!-- mk-toc:start style=numbered -->
<!-- mk-toc:end -->
```

This generates:
```markdown
1. [Installation](docs/getting-started.md#installation)
1. [Quick Start Guide](docs/getting-started.md#quick-start-guide)
   1. [Prerequisites](docs/getting-started.md#prerequisites)
```

## Advanced Usage

### Multiple Target Files

You can generate cross-file TOCs in multiple files:

```bash
# Update both README.md and docs/INDEX.md with cross-file TOC from docs/
mk-tools toc README.md docs/INDEX.md --from-dir docs/
```

### Nested Directories

The `--from-dir` option processes all subdirectories recursively:

```bash
# Generate TOC from all files in docs/ and its subdirectories
mk-tools toc README.md --from-dir docs/ --glob "**/*.md"
```

### CI/CD Integration

Check that your cross-file TOC is up-to-date in CI:

```bash
mk-tools toc README.md --from-dir docs/ --check || exit 1
```

### Preview Changes

Use `--dry-run` to see what would change without modifying files:

```bash
mk-tools --dry-run toc README.md --from-dir docs/
```

## Tips

1. **Organize with level-1 headings**: Use `# File Title` as the first heading in each file to create clear sections in your cross-file TOC.

2. **Consistent heading structure**: Keep heading levels consistent across files for a clean-looking TOC.

3. **Exclude files with glob patterns**: Use `--glob` to control which files are included:
   ```bash
   mk-tools toc README.md --from-dir docs/ --glob "docs/*.md"  # Top-level only
   ```

4. **Multiple TOC regions**: You can have multiple TOC markers in the same file - some for cross-file and some for same-file TOCs.

5. **Update regularly**: Run mk-tools whenever you add new documentation files or sections to keep your index current.

## Comparison with Regular TOC

| Feature | Regular TOC | Cross-File TOC |
|---------|-------------|----------------|
| Command | `mk-tools toc FILE` | `mk-tools toc FILE --from-dir DIR` |
| Source | Same file only | All files in directory |
| Links | `#heading` | `path/file.md#heading` |
| Use case | Single-file documentation | Multi-file documentation index |

## Example Projects

This feature is perfect for:

- **API documentation** with separate files for each endpoint
- **Multi-language guides** with a file per language
- **Module documentation** with a file per module
- **Tutorial series** with steps across multiple files
- **Knowledge base** with articles across different categories

## Next Steps

- See the main README for more examples
- Check out `docs/QUICK_REFERENCE.md` for command syntax
- Read `docs/spec.md` for technical details
