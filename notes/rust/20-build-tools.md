+++
title = "Build tools, package managers, and CLI commands"
lang = "rust"
part = 20
tags = ["cargo", "cli", "clippy", "rustfmt", "release"]
version = "Rust 2021"
+++

## Essential Cargo workflow commands
Cargo is Rust's official package manager, build tool, documentation generator, and test runner.

```bash
# Compile and run during development
cargo run

# Build optimized binary in target/release
cargo build --release

# Run unit and integration tests
cargo test

# Check code for compilation errors without generating binary (fast)
cargo check

# Automatic code formatting
cargo fmt

# Linter with strict error thresholds
cargo clippy -- -D warnings

# Generate and view HTML documentation offline
cargo doc --open
```

## Cargo.toml dependency declarations
`Cargo.toml` tracks crate dependencies with semantic versioning.

```toml
[package]
name = "myproject"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
```
