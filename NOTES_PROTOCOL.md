# Notes Protocol Specification

`mycheatsheet` is completely language-agnostic. The core engine has zero hardcoded languages, syntax rules, or note hierarchies. It discovers, validates, and renders any programming language or technology topic adhering to the **Notes Protocol**.

---

## 1. Directory Structure

A note repository contains a root directory passed to `--notes <PATH>` (or installed in `~/.local/share/mycheatsheet/notes` on Linux / `%LOCALAPPDATA%\mycheatsheet\notes` on Windows).

```text
notes/
├── languages.toml                  # Optional: metadata and UI tab order
├── go/                             # Language directory (folder name = language ID)
│   ├── 01-program-structure.md
│   ├── 02-variables-and-types.md
│   └── ...
├── rust/
│   ├── 01-program-structure.md
│   └── ...
└── python/                         # Custom user language
    ├── 01-program-structure.md
    └── ...
```

---

## 2. Language Metadata (`languages.toml`)

You can customize display names, descriptions, and UI presentation order by placing a `languages.toml` file at the root of the notes directory:

```toml
[[languages]]
id = "go"
name = "Go"
order = 1
version = "Go 1.22+"

[[languages]]
id = "rust"
name = "Rust"
order = 2
version = "Rust 2021"

[[languages]]
id = "python"
name = "Python"
order = 5
version = "Python 3.12+"
```

### Fields:
- `id` (string, required): Matches the subfolder name in `notes/`.
- `name` (string, required): Human-readable name displayed in tab headers and dropdowns.
- `order` (integer, optional): Sort priority in the UI (lowest numbers appear first).
- `version` (string, optional): Displayed badge showing target language version.

If `languages.toml` is omitted, the engine automatically derives language names from the directory names and sorts them alphabetically.

---

## 3. Note File Specification

Every note file must follow strict naming and structuring rules.

### Filename Format
- Must match `XX-slug.md`, where `XX` is a two-digit integer starting from `01`.
- The number prefix must match the `part` integer declared inside the front matter.
- Examples: `01-program-structure.md`, `02-variables-and-types.md`.

### Front Matter Format
Front matter is enclosed between `+++` fences at the very top of the file:

```markdown
+++
title = "Variables & Data Types"
lang = "python"
part = 2
tags = ["variables", "types", "casting", "strings"]
version = "Python 3.12+"
+++
```

#### Required Front Matter Fields:
| Field | Type | Description |
| :--- | :--- | :--- |
| `title` | string | Human-readable title of the topic part. |
| `lang` | string | Language identifier (must match the directory name). |
| `part` | integer | Sequential index (must match the filename prefix). |
| `tags` | list of strings | Searchable keywords (at least one tag is required). |

#### Optional Front Matter Fields:
| Field | Type | Description |
| :--- | :--- | :--- |
| `version` | string | Language version badge for this specific part. |

---

## 4. Note Entries Specification

Below the closing `+++`, the file contains one or more syntax entries. Each entry represents an actionable syntax pattern.

```markdown
## Variable Declaration & Type Hints
Python variables are dynamically typed, but type annotations document intent and enable static verification with Mypy or Pyright.

```python
name: str = "Alice"
age: int = 30
is_active: bool = True
items: list[str] = ["apple", "banana"]
```

## Type Casting & Conversion
Convert primitives using standard built-in constructors. Invalid casts raise `ValueError`.

```python
count_str = "42"
count_num = int(count_str)
price = float("19.99")
as_string = str(100)
```
```

### Entry Requirements:
1. **Level-2 Heading (`## <Title>`)**:
   - Each entry must start with a `## ` header representing the pattern's name.
2. **Explanatory Text**:
   - At least 1–2 sentences explaining the context, trade-offs, or gotchas.
3. **Fenced Code Block**:
   - At least one fenced code block (```` ```<lang> ... ``` ````) demonstrating practical, idiomatic syntax.
   - Code blocks must have explicit closing fences.
4. **No Raw HTML**:
   - Raw HTML tags (`<script>`, `<div>`, etc.) are stripped during rendering to preserve security.

---

## 5. Built-in Protocol Validator (`--check`)

`mycheatsheet` ships with a strict validator to verify note files before deploying:

```bash
# Validate bundled notes
cargo run -- --check

# Validate a custom directory
mycheatsheet --notes /path/to/my-notes --check
```

### What `--check` Validates:
- Correct filename prefix formatting (`XX-*.md`).
- Front matter presence and valid TOML syntax.
- `part` number consistency between filename and front matter.
- Non-empty `tags` array.
- Language directory matching the `lang` front matter attribute.
- At least one `## ` entry per file.
- Unclosed code block fences or missing syntax highlighting tags.

### Error Output Example:
```text
notes/python/02-variables.md:14: unclosed code block in entry 'Type Casting'
Validation failed: 1 error(s) found across 20 note(s).
```
