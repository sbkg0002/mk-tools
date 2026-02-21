# mk-tools Example Document

This document demonstrates the features of mk-tools.

<!-- mk-toc:start from-level=2 to-level=3 -->
- [Introduction](#introduction)
- [Code Examples](#code-examples)
  - [Basic Example](#basic-example)
  - [Partial Code Extraction](#partial-code-extraction)
  - [With Custom Language](#with-custom-language)
  - [Dedented Code](#dedented-code)
- [Features](#features)
  - [Code Synchronization](#code-synchronization)
  - [Table of Contents](#table-of-contents)
- [Advanced Usage](#advanced-usage)
  - [CI Integration](#ci-integration)
  - [Multiple Files](#multiple-files)
- [Conclusion](#conclusion)
<!-- mk-toc:end -->

## Introduction

mk-tools is a CLI tool for managing Markdown files. It can automatically sync code blocks from source files and generate table of contents.

## Code Examples

### Basic Example

Here's a simple Rust program:

<!-- mk-code: ./hello.rs -->
```rust
fn main() {
    println!("Hello, world!");
    greet("mk-tools");
    demonstrate_features();
}

fn greet(name: &str) {
    println!("Welcome to {}!", name);
}

fn demonstrate_features() {
    println!("\nFeatures:");
    println!("- Code block synchronization");
    println!("- Table of contents generation");
    println!("- CI/CD integration");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        greet("test");
    }
}
```

### Partial Code Extraction

You can extract just the main function:

<!-- mk-code: ./hello.rs start=1 end=5 -->
```rust
fn main() {
    println!("Hello, world!");
    greet("mk-tools");
    demonstrate_features();
}
```

### With Custom Language

Or specify a different language identifier:

<!-- mk-code: ./hello.rs lang=rust start=7 end=9 -->
```rust
fn greet(name: &str) {
    println!("Welcome to {}!", name);
}
```

### Dedented Code

Extract with indentation removed:

<!-- mk-code: ./hello.rs start=18 end=25 dedent=4 -->
```rust
#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_greet() {
    greet("test");
}
```

## Features

### Code Synchronization

The code blocks above are automatically synchronized with the source file. When you run:

```bash
mk-tools codeblocks example.md
```

All the code blocks will be updated to match the current content of `hello.rs`.

### Table of Contents

The table of contents at the top of this document is automatically generated from the headings. Run:

```bash
mk-tools toc example.md
```

## Advanced Usage

### CI Integration

You can verify that documentation is up-to-date in CI:

```bash
mk-tools check .
```

### Multiple Files

Process all Markdown files in a directory:

```bash
mk-tools codeblocks docs/
mk-tools toc docs/
```

## Conclusion

mk-tools helps keep your documentation accurate and up-to-date with your codebase.
