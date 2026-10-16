# mycheatsheet

Fast, offline syntax cheatsheet for programming languages in your terminal and browser.

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

### 1. Run directly with Cargo (zero configuration)
If running from the cloned repository, it automatically detects the bundled `notes/` directory:
```bash
# Open in your default browser (http://127.0.0.1:7878)
cargo run

# Or launch the interactive terminal UI
cargo run -- -t
```

### 2. Install on Linux (installs binary, desktop launcher, and default notes)
The installer builds the binary, deploys the desktop icon, and installs the default 80 notes to `~/.local/share/mycheatsheet/notes`:
```bash
./scripts/install.sh

# Now launch from anywhere or from your app launcher:
mycheatsheet          # Web mode
mycheatsheet -t       # Terminal UI mode

# To uninstall cleanly:
./scripts/install.sh --uninstall
```

### 3. Install on Windows (PowerShell)
Installs the binary, Start Menu shortcut with icon, and default notes to `%LOCALAPPDATA%\mycheatsheet\notes`:
```powershell
.\scripts\install.ps1

# To uninstall:
.\scripts\install.ps1 -Uninstall
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

## Adding custom languages (The Notes Protocol)

The core application has **zero hardcoded languages**. You can add your own notes by creating a directory and following this simple protocol.

### Directory structure
```text
my-notes/
├── languages.toml              # Optional: display names and UI ordering
└── python/                     # Folder name becomes the language ID
    ├── 01-program-structure.md
    ├── 02-variables.md
    └── ...
```

### `languages.toml` format
```toml
[[languages]]
id = "python"
name = "Python"
order = 1
version = "3.12+"
```

### Note file specification
Every note file is named `<number>-<slug>.md` (e.g., `01-program-structure.md`) and contains TOML front matter followed by Markdown entries:

```markdown
+++
title = "Program structure & entry point"
lang = "python"
part = 1
tags = ["entry", "main", "script"]
version = "Python 3.12+"
+++

## Script entry point
Python scripts run top-to-bottom. Use the `__name__` check to allow running standalone or as an imported module.

```python
def main():
    print("Hello, Python!")

if __name__ == "__main__":
    main()
```

## Command line arguments
The `sys.argv` list contains CLI parameters starting with the script name.

```python
import sys

print(f"Arguments: {sys.argv[1:]}")
```
```

### Protocol rules
1. **Filename**: Must start with a 2-digit number matching the `part` number in the front matter (`01-*.md`).
2. **Front matter**: Enclosed in `+++`. Requires `title`, `lang`, `part`, and at least one tag in `tags`.
3. **Entries**: Every section begins with a `## <Title>` heading.
4. **Description & Code**: Each section must have a brief description and at least one fenced code block (```` ```<lang> ````).
5. **No raw HTML**: Keep content in clean Markdown; raw HTML is stripped for security.

### Check your notes
Run the built-in validator to catch typos or syntax mistakes before launching:
```bash
mycheatsheet --notes /path/to/my-notes --check
```
If anything is broken, it tells you the exact file and line number:
```text
notes/python/02-variables.md:14: unclosed code block in entry 'Variable assignment'
Validation failed: 1 error(s) found across 20 note(s).
```

---

## How it works

- **Parser & Loader**: [`src/notes/`](src/notes/) scans note folders and parses front matter and code blocks into an in-memory database. Corrupted files trigger non-fatal warnings so one bad note never crashes your app.
- **Search Engine**: [`src/search/`](src/search/) tokenizes titles, tags, and code blocks. Queries run in under 2ms using a zero-allocation Levenshtein distance implementation pruned with 64-bit character bitmasks (`POPCNT`).
- **Web Server**: [`src/web/`](src/web/) uses `tiny_http` with zero async runtime overhead. It strictly binds to `127.0.0.1`, validates `Host` headers to prevent DNS rebinding, escapes HTML, and automatically shuts down after 15 minutes of inactivity. Assets are embedded directly in the binary.
- **Terminal UI**: [`src/tui/`](src/tui/) uses `ratatui` and `crossterm` with a 4-zone layout (search top, results left, preview right, help bottom).
- **Clipboard**: [`src/tui/clipboard.rs`](src/tui/clipboard.rs) tries the OS clipboard via `arboard`, then falls back to ANSI OSC 52 escape sequences if run in an environment without X11/Wayland utilities.

---

## What it can't do yet

- **Live note editing in-app**: Notes are read-only when viewing. You edit them in your favorite editor; the web UI picks up edits on refresh, and the TUI reloads them when you press `r`.
- **Fuzzy regex searching**: The search engine supports token prefix matching and typo tolerance, but not full regex queries.

---

## Development

```bash
# Build dev binary
cargo build

# Run all 40 unit and integration tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Validate all shipped notes
cargo run -- --notes notes --check
```

### Repo layout
```text
mycheatsheet/
├── assets/          # Vector SVG icon, multi-resolution PNGs, ICO
├── notes/           # 80 standardized notes (Go, Rust, C++, JavaScript)
├── scripts/         # Linux (install.sh) and Windows (install.ps1) installers
├── src/
│   ├── config.rs    # CLI arguments & path resolution
│   ├── launcher/    # Browser launcher & single-instance detection
│   ├── notes/       # Parser, loader, validation rules
│   ├── search/      # In-memory index, tokenizer, typo scoring
│   ├── tui/         # Ratatui dashboard, key events, clipboard
│   └── web/         # Localhost HTTP server, markdown renderer, embedded assets
└── tests/           # Integration tests for notes, search, TUI, and web
```

---

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))

at your option.
