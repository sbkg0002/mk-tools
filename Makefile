.PHONY: help build test check fmt lint clean install release run-example all

# Default target
all: check build test

# Show help
help:
	@echo "mk-tools Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  make build          - Build the project in debug mode"
	@echo "  make release        - Build the project in release mode"
	@echo "  make test           - Run all tests"
	@echo "  make check          - Run all checks (fmt, lint, test)"
	@echo "  make fmt            - Format code with rustfmt"
	@echo "  make lint           - Run clippy lints"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make install        - Install binary to /usr/local/bin"
	@echo "  make run-example    - Run mk-tools on example files"
	@echo "  make watch          - Watch for changes and run tests"
	@echo "  make doc            - Generate and open documentation"
	@echo "  make bench          - Run benchmarks (if any)"
	@echo "  make ci             - Run full CI checks locally"
	@echo "  make all            - Run check, build, and test (default)"

# Build in debug mode
build:
	@echo "Building mk-tools (debug)..."
	cargo build

# Build in release mode
release:
	@echo "Building mk-tools (release)..."
	cargo build --release
	@echo "Binary available at: target/release/mk-tools"

# Run all tests
test:
	@echo "Running tests..."
	cargo test

# Run tests with output
test-verbose:
	@echo "Running tests (verbose)..."
	cargo test -- --nocapture --test-threads=1

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all

# Check formatting without modifying
fmt-check:
	@echo "Checking code formatting..."
	cargo fmt --all -- --check

# Run clippy lints
lint:
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features

# Run all checks (for CI)
check: fmt-check lint test
	@echo "All checks passed!"

# Full CI check
ci: check
	@echo "Running full CI checks..."
	cargo build --release
	@echo "CI checks complete!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	find . -name "*.bak" -type f -delete
	@echo "Clean complete!"

# Install binary to system
install: release
	@echo "Installing mk-tools to /usr/local/bin..."
	sudo cp target/release/mk-tools /usr/local/bin/
	@echo "Installed! Run 'mk-tools --help' to verify."

# Run mk-tools on example files
run-example:
	@echo "Running mk-tools on examples..."
	cargo run -- codeblocks examples/example.md
	cargo run -- toc examples/example.md
	@echo "Example processing complete!"

# Run mk-tools to update project docs
update-docs:
	@echo "Updating project documentation..."
	cargo run -- codeblocks README.md CONTRIBUTING.md
	cargo run -- toc README.md CONTRIBUTING.md docs/
	@echo "Documentation updated!"

# Check project docs are up-to-date
check-docs:
	@echo "Checking documentation is up-to-date..."
	cargo run -- check README.md CONTRIBUTING.md docs/

# Watch for changes and run tests
watch:
	@echo "Watching for changes..."
	@command -v cargo-watch >/dev/null 2>&1 || { echo "cargo-watch not installed. Run: cargo install cargo-watch"; exit 1; }
	cargo watch -x test

# Generate and open documentation
doc:
	@echo "Generating documentation..."
	cargo doc --no-deps --open

# Open documentation without rebuilding
doc-open:
	@echo "Opening documentation..."
	cargo doc --no-deps --open

# Run benchmarks
bench:
	@echo "Running benchmarks..."
	cargo bench

# Check for outdated dependencies
outdated:
	@command -v cargo-outdated >/dev/null 2>&1 || { echo "cargo-outdated not installed. Run: cargo install cargo-outdated"; exit 1; }
	cargo outdated

# Audit dependencies for security vulnerabilities
audit:
	@command -v cargo-audit >/dev/null 2>&1 || { echo "cargo-audit not installed. Run: cargo install cargo-audit"; exit 1; }
	cargo audit

# Run with verbose logging
run-verbose:
	@echo "Running with verbose logging..."
	RUST_LOG=debug cargo run -- --help

# Create a new release (update version and tag)
prepare-release:
	@echo "Preparing release..."
	@echo "Current version: $$(cargo pkgid | cut -d'#' -f2)"
	@echo "Update version in Cargo.toml and CHANGELOG.md"
	@echo "Then run: git tag vX.Y.Z && git push origin vX.Y.Z"

# Show project statistics
stats:
	@echo "Project Statistics:"
	@echo "-------------------"
	@echo "Lines of Rust code:"
	@find src -name "*.rs" | xargs wc -l | tail -1
	@echo ""
	@echo "Number of tests:"
	@grep -r "#\[test\]" src | wc -l | xargs echo
	@echo ""
	@echo "Dependencies:"
	@cargo tree --depth 1 | wc -l | xargs echo

# Development setup
setup:
	@echo "Setting up development environment..."
	@command -v rustc >/dev/null 2>&1 || { echo "Rust not installed. Visit https://rustup.rs"; exit 1; }
	rustup component add rustfmt clippy
	@echo "Setup complete!"
	@echo "Optional tools:"
	@echo "  cargo install cargo-watch    # For 'make watch'"
	@echo "  cargo install cargo-audit    # For 'make audit'"
	@echo "  cargo install cargo-outdated # For 'make outdated'"
