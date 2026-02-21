# Contributing to mk-tools

Thank you for your interest in contributing to mk-tools! We welcome contributions from everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Pull Requests](#pull-requests)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Commit Messages](#commit-messages)

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow. Please be respectful and constructive in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please check the existing issues to avoid duplicates.

When filing a bug report, please include:

- **A clear and descriptive title**
- **Steps to reproduce the issue**
- **Expected behavior**
- **Actual behavior**
- **Environment details** (OS, Rust version, mk-tools version)
- **Sample files** if applicable (Markdown files, source files)
- **Error messages** (full output with `--verbose` flag)

Example bug report:

```markdown
## Bug: Code block not updating with line range

**Steps to reproduce:**
1. Create a Markdown file with `<!-- mk-code: test.rs start=5 end=10 -->`
2. Run `mk-tools codeblocks file.md`

**Expected:** Lines 5-10 should be extracted

**Actual:** Entire file is included

**Environment:**
- OS: macOS 14.1
- mk-tools: v0.1.0
- Rust: 1.75.0
```

### Suggesting Enhancements

Enhancement suggestions are welcome! Please include:

- **Clear use case**: Explain the problem you're trying to solve
- **Proposed solution**: Describe how you envision the feature working
- **Alternatives considered**: What other approaches did you think about?
- **Examples**: Show how the feature would be used

### Pull Requests

1. **Fork the repository** and create a new branch from `main`
2. **Make your changes** following our coding standards
3. **Add tests** for new functionality
4. **Update documentation** if needed
5. **Ensure all tests pass** (`cargo test`)
6. **Run formatting** (`cargo fmt`)
7. **Run linter** (`cargo clippy -- -D warnings`)
8. **Submit a pull request** with a clear description

## Development Setup

### Prerequisites

- **Rust** 1.70 or later (latest stable recommended)
- **Git**
- **A text editor** (VS Code, Vim, etc.)

### Initial Setup

1. **Clone the repository:**

```bash
git clone https://github.com/sbkg0002/mk-tools.git
cd mk-tools
```

2. **Build the project:**

```bash
cargo build
```

3. **Run tests:**

```bash
cargo test
```

4. **Run the binary:**

```bash
cargo run -- --help
```

### IDE Setup

#### Visual Studio Code

Recommended extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Better TOML

#### Vim/Neovim

Install rust-analyzer LSP and configure your LSP client.

## Development Workflow

### Creating a Feature Branch

```bash
git checkout -b feature/my-awesome-feature
```

Branch naming conventions:
- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring
- `test/description` - Test improvements

### Making Changes

1. **Write code** following the project structure:
   - Domain logic in `src/domain/`
   - File system operations in `src/fs/`
   - Markdown processing in `src/markdown/`
   - CLI definitions in `src/cli.rs`

2. **Add tests** in the same file as your code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        // Test implementation
    }
}
```

3. **Update documentation** as needed:
   - Inline code comments for complex logic
   - Doc comments (`///`) for public APIs
   - Update README.md for user-facing changes
   - Update CHANGELOG.md

### Running Checks Locally

Before submitting a PR, run these checks:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run all tests
cargo test

# Build in release mode
cargo build --release

# Test the binary
./target/release/mk-tools check examples/
```

## Coding Standards

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` to format code (enforced in CI)
- Address all `clippy` warnings (enforced in CI)
- Use meaningful variable and function names
- Keep functions focused and small
- Prefer explicit error handling over panics

### Code Organization

- Keep modules focused on a single responsibility
- Use clear, descriptive names for types and functions
- Group related functionality together
- Minimize public APIs - only expose what's necessary

### Error Handling

- Use `anyhow::Result` for functions that can fail
- Provide context with `.context()`:

```rust
fs::read_to_string(path)
    .with_context(|| format!("Failed to read file: {}", path.display()))?
```

- Use `thiserror` for custom error types if needed
- Log errors appropriately with the `log` crate

### Comments and Documentation

- Write doc comments for all public items:

```rust
/// Parse markdown content and find all codeblock markers.
///
/// # Arguments
///
/// * `content` - The markdown file content as a string
/// * `markdown_file_path` - Path to the markdown file being processed
///
/// # Returns
///
/// A vector of `CodeblockSpec` containing all found markers.
pub fn find_codeblock_markers(
    content: &str,
    markdown_file_path: &Path,
) -> Result<Vec<CodeblockSpec>> {
    // Implementation
}
```

- Add inline comments for complex logic
- Keep comments up-to-date with code changes

## Testing

### Unit Tests

Write unit tests for all new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_options() {
        let result = parse_options("lang=rust start=5").unwrap();
        assert_eq!(result.lang, Some("rust".to_string()));
        assert_eq!(result.start, Some(5));
    }

    #[test]
    fn test_parse_options_invalid() {
        let result = parse_options("invalid");
        assert!(result.is_err());
    }
}
```

### Integration Tests

For larger features, consider adding integration tests in `tests/`:

```rust
// tests/integration_test.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_codeblocks_command() {
    let dir = tempdir().unwrap();
    // Setup test files
    // Run command
    // Assert results
}
```

### Test Coverage

- Aim for high test coverage of new code
- Test both success and error cases
- Test edge cases and boundary conditions
- Use `tempfile` for filesystem tests

## Documentation

### Code Documentation

- Add doc comments (`///`) for all public functions, types, and modules
- Include examples in doc comments when helpful:

```rust
/// Generate a code fence block.
///
/// # Examples
///
/// ```
/// let block = generate_code_block("fn main() {}", Some("rust"));
/// assert_eq!(block, "```rust\nfn main() {}\n```");
/// ```
pub fn generate_code_block(content: &str, lang: Option<&str>) -> String {
    // Implementation
}
```

### User Documentation

- Update README.md for user-facing changes
- Add examples to the `examples/` directory
- Update the specification in `docs/spec.md` for architectural changes
- Keep CHANGELOG.md up-to-date

## Commit Messages

Write clear, concise commit messages:

### Format

```
<type>: <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

**Good commit messages:**

```
feat: add support for numbered TOC lists

Implement the `style=numbered` option for TOC generation.
This allows users to choose between bullet and numbered lists.

Closes #42
```

```
fix: handle empty code blocks correctly

Previously, empty code blocks would cause a panic. Now they're
handled gracefully with an appropriate error message.

Fixes #38
```

**Bad commit messages:**

```
fix stuff
```

```
WIP
```

```
update files
```

## Pull Request Process

1. **Update CHANGELOG.md** with your changes under the `[Unreleased]` section

2. **Ensure CI passes**:
   - All tests pass
   - Code is formatted (`cargo fmt`)
   - No clippy warnings (`cargo clippy`)

3. **Write a clear PR description**:
   - What does this PR do?
   - Why is this change needed?
   - How was it tested?
   - Are there any breaking changes?

4. **Link related issues**: Use "Fixes #123" or "Closes #123" in the PR description

5. **Respond to review feedback** promptly and professionally

6. **Squash commits** if requested before merging

### PR Template

```markdown
## Description
Brief description of the changes.

## Motivation
Why is this change needed?

## Changes
- Change 1
- Change 2

## Testing
How was this tested?

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Code formatted (`cargo fmt`)
- [ ] Clippy checks pass (`cargo clippy`)
- [ ] All tests pass (`cargo test`)
```

## Getting Help

- **Questions?** Open a discussion or issue on GitHub
- **Stuck?** Don't hesitate to ask for help in your PR
- **Ideas?** We'd love to hear them - open an issue for discussion

## Recognition

All contributors will be recognized in the project. Thank you for helping make mk-tools better!

## License

By contributing to mk-tools, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).
