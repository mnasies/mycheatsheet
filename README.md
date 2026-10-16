<p align="center">
  <img src="assets/icon.svg" alt="mycheatsheet logo" width="128" height="128" />
</p>

<h1 align="center">mycheatsheet</h1>

<p align="center">
  <strong>Fast, offline syntax cheatsheet for programming languages in your terminal and browser.</strong>
</p>

<p align="center">
  <a href="https://github.com/mnasies/mycheatsheet/actions/workflows/ci.yml"><img src="https://github.com/mnasies/mycheatsheet/actions/workflows/ci.yml/badge.svg" alt="CI Status" /></a>
  <img src="https://img.shields.io/badge/tests-40%20passing-brightgreen" alt="Tests" />
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20windows-lightgrey?logo=linux&logoColor=white" alt="Platform" />
  <img src="https://img.shields.io/badge/rust-2021%20edition-orange?logo=rust" alt="Rust" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License" /></a>
  <a href="https://github.com/mnasies/mycheatsheet/releases"><img src="https://img.shields.io/badge/release-v0.1.0-blue?logo=github" alt="Latest Release" /></a>
</p>

```text
$ mycheatsheet --notes notes
Loaded 4 language(s) with 168 entries.
mycheatsheet server running at: http://127.0.0.1:7878
Serving Editorial Edition (auto-stops after 15 min idle). Press Ctrl+C to quit.
```

---

## Why I built this

Switching between Go, Rust, C++, and JavaScript every day means constantly tripping over syntax quirks. How do I write a type switch in Go again? What was the syntax for an rvalue reference in C++20? How do I iterate over UTF-8 runes in Rust?

Searching the web or prompting an LLM for quick syntax checks is noisy, slow, and useless when you're on a plane or have spotty Wi-Fi. Most existing cheatsheet apps are either heavy web apps, bloated Electron wrappers, or have the notes hardcoded into the binary.

`mycheatsheet` is a single lightweight Rust binary with two modes:
1. **Local web UI**: A warm, distraction-free editorial interface with instant search, syntax highlighting, and copy buttons.
2. **Terminal UI (TUI)**: A full-screen keyboard-driven terminal dashboard built on `ratatui`.

It comes loaded with a complete default curriculum of **80 standardized notes across Go, Rust, C++, and JavaScript** (168 verified syntax patterns), so you can use it immediately without writing any notes.

The binary is also completely language-agnostic. It reads plain Markdown files from a directory according to a simple protocol. You can add Python, Zig, Lua, or your team's internal DSL in five minutes without touching a line of Rust.

---

## Quick start

### 1. Download pre-compiled binary (Quickest)
Download the standalone executable from the [Releases](https://github.com/mnasies/mycheatsheet/releases/latest) page:

**Linux**:
```bash
# Download and install binary
curl -sSLO https://github.com/mnasies/mycheatsheet/releases/latest/download/mycheatsheet-linux-x86_64.tar.gz
tar -xzf mycheatsheet-linux-x86_64.tar.gz
sudo install -m 755 mycheatsheet /usr/local/bin/

# Launch
mycheatsheet
```

**Windows**:
Download `mycheatsheet-windows-x86_64.zip` from [Releases](https://github.com/mnasies/mycheatsheet/releases/latest), unpack `mycheatsheet.exe`, and place it in your `PATH`.

---

### 2. Install on Linux (Desktop launcher + bundled notes)
The installer builds or places the binary, deploys the desktop icon to your application launcher, and installs the default 80 notes to `~/.local/share/mycheatsheet/notes`:
```bash
./scripts/install.sh

# Now launch from anywhere or from your app launcher:
mycheatsheet          # Web mode
mycheatsheet -t       # Terminal UI mode

# To uninstall cleanly:
./scripts/install.sh --uninstall
```

---

### 3. Install on Windows (PowerShell)
Installs the binary, Start Menu shortcut with embedded icon, and default notes to `%LOCALAPPDATA%\mycheatsheet\notes`:
```powershell
.\scripts\install.ps1

# To uninstall:
.\scripts\install.ps1 -Uninstall
```

---

### 4. Run directly with Cargo (from source)
If running inside the repository clone, it automatically detects the bundled `notes/` directory:
```bash
# Open in your default browser (http://127.0.0.1:7878)
cargo run

# Or launch the interactive terminal UI
cargo run -- -t
```

---

## Usage and flags

```text
Usage: mycheatsheet [OPTIONS] [QUERY]

Arguments:
  [QUERY]  Initial search term (e.g. "goroutine" or "go: defer")

Options:
  -n, --notes <DIR>      Path to notes directory (falls back to config or ./notes)
  -t, --terminal         Launch full-screen Terminal UI instead of web browser
  -l, --lang <LANG>      Filter search results by language ID (e.g. "go", "rust")
  -p, --port <PORT>      Localhost web server port [default: 7878]
      --no-open          Do not automatically launch the system web browser
  -c, --check            Strictly validate notes directory against the Notes Protocol
  -h, --help             Print help information
  -V, --version          Print version information
```

### Helpful tricks
- **Fast prefix search**: Type `go: defer` or `lang:rust trait` in either the web or TUI search bar to scope queries instantly.
- **TUI navigation**: Press `/` to focus the search box, `Up`/`Down` or `Ctrl-N`/`Ctrl-P` to pick a result, `Enter` to read the entry, and `y` to yank code to your clipboard.
- **OSC 52 clipboard**: If you use the TUI over SSH or inside tmux where system clipboards are unavailable, `mycheatsheet` falls back to ANSI OSC 52 copy sequences automatically.
- **Instance reuse**: If `mycheatsheet` is already running on port 7878, running `mycheatsheet` again pings `/health`, opens the existing tab, and exits cleanly.

---

## Adding custom languages

`mycheatsheet` has **zero hardcoded languages**. You can add your own notes by creating a directory and following the protocol:

1. **Folder structure**: Create `notes/<language-id>/XX-slug.md`.
2. **Front matter**: Add `+++` TOML headers with `title`, `lang`, `part`, and `tags`.
3. **Entries**: Write `## Pattern Title` followed by explanatory text and fenced code blocks.
4. **Validation**: Run `mycheatsheet --notes /path/to/notes --check` to verify syntax.

📖 Read the complete [Notes Protocol Specification](NOTES_PROTOCOL.md) for directory formats, front matter schemas, and validation rules.

---

## Architecture & Design

`mycheatsheet` is built for sub-millisecond local latency with zero heavyweight async runtime dependencies.

- **Storage**: Plain Markdown files with TOML front matter.
- **Search Engine**: Hardware-accelerated 64-bit character bitmask pruning (`POPCNT`) and stack-allocated Levenshtein distance scoring.
- **Terminal UI**: Alternate screen buffer with `ratatui` and dual-tier clipboard (OS clipboard + ANSI OSC 52).
- **Web UI**: In-process `tiny_http` server, DNS-rebinding protection, CommonMark rendering, and 15-minute inactivity auto-shutdown.

📖 Read the complete [Architecture & Design Document](ARCHITECTURE.md) with subsystem breakdowns and Mermaid diagrams.

---

## Development & Contributing

Contributions are warmly welcomed! Please read our [Contributing Guide](CONTRIBUTING.md) before submitting pull requests.

```bash
# Clone and checkout dev branch
git clone https://github.com/mnasies/mycheatsheet.git
cd mycheatsheet
git checkout dev

# Run all 40 unit and integration tests
cargo test

# Check code formatting & clippy
cargo fmt --check
cargo clippy -- -D warnings

# Validate all shipped notes
cargo run -- --check
```

---

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>).