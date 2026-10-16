# Contributing to mycheatsheet

Thank you for your interest in contributing to `mycheatsheet`! Whether you're adding new programming language cheatsheets, improving search performance, or fixing bugs in the TUI/Web UI, we welcome your contributions.

---

## 1. Branching Model

- **`main`**: Protected branch representing stable, released code.
- **`dev`**: Integration branch for ongoing development. **All feature branches and Pull Requests should branch off and target `dev`**.

```bash
# Clone the repository
git clone https://github.com/mnasies/mycheatsheet.git
cd mycheatsheet

# Switch to the dev branch and create a feature branch
git checkout dev
git checkout -b feature/my-new-feature
```

---

## 2. Setting Up Your Environment

You will need the latest stable Rust toolchain:
```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required components
rustup component add rustfmt clippy
```

---

## 3. Contributing New Language Notes

Adding a new language (e.g., Python, Zig, Lua, Bash) requires **zero Rust changes**:

1. Read the [Notes Protocol](NOTES_PROTOCOL.md) for full formatting rules.
2. Create a folder under `notes/<language-id>/` (e.g. `notes/python/`).
3. Add note parts sequentially (`01-program-structure.md`, `02-variables.md`, etc.).
4. Add the language metadata to `notes/languages.toml`.
5. Run the built-in validator:
   ```bash
   cargo run -- --notes notes --check
   ```
6. Ensure all files pass validation with 0 errors.

---

## 4. Quality Gates & Testing

Before submitting a Pull Request, ensure all quality checks pass locally:

```bash
# 1. Run all unit and integration tests
cargo test

# 2. Check code formatting
cargo fmt --check

# 3. Run Clippy linter (zero warnings allowed)
cargo clippy -- -D warnings

# 4. Verify all note files pass protocol validation
cargo run -- --check
```

---

## 5. Submitting a Pull Request

1. Push your branch to your fork or branch on GitHub.
2. Open a Pull Request targeting the **`dev`** branch (not `main`).
3. Provide a clear description of the problem solved or feature added.
4. If addressing an open issue, link it in the PR body (e.g. `Closes #1`).

---

## 6. Good First Issues

If you're looking for where to start, check our GitHub Issues tab filtered by:
- [`good first issue`](https://github.com/mnasies/mycheatsheet/labels/good%20first%20issue): Well-scoped tasks ideal for newcomers (e.g. adding new language curricula, CLI completions).
- [`enhancement`](https://github.com/mnasies/mycheatsheet/labels/enhancement): New features or improvements to existing capabilities.
