use crate::notes::model::{CodeBlock, FrontMatter, NoteEntry, NotePart};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Error encountered while parsing a note file according to the Notes Protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub file: PathBuf,
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.file.display(), self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Parses a note file from its string content into a `NotePart`.
pub fn parse_note(
    content: &str,
    file_path: &Path,
    modified: SystemTime,
) -> Result<NotePart, ParseError> {
    // Strip UTF-8 BOM if present
    let clean_content = content.strip_prefix('\u{feff}').unwrap_or(content);

    let (front_matter, body, body_start_line) = parse_front_matter(clean_content, file_path)?;

    let entries = parse_entries(body, body_start_line, file_path)?;

    Ok(NotePart {
        lang: front_matter.lang,
        part: front_matter.part,
        title: front_matter.title,
        version: front_matter.version,
        tags: front_matter.tags,
        entries,
        file_path: file_path.to_path_buf(),
        modified,
    })
}

/// Extracts TOML front matter delimited by `+++` and returns `(FrontMatter, body_str, body_start_line)`.
pub fn parse_front_matter<'a>(
    content: &'a str,
    file_path: &Path,
) -> Result<(FrontMatter, &'a str, usize), ParseError> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err(ParseError {
            file: file_path.to_path_buf(),
            line: 1,
            message: "file is empty".to_string(),
        });
    }

    // Find opening +++
    let mut start_idx = None;
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "+++" {
            start_idx = Some(idx);
            break;
        }
        return Err(ParseError {
            file: file_path.to_path_buf(),
            line: idx + 1,
            message: format!(
                "expected front matter opening '+++', found '{}'",
                line.trim()
            ),
        });
    }

    let start_idx = match start_idx {
        Some(idx) => idx,
        None => {
            return Err(ParseError {
                file: file_path.to_path_buf(),
                line: 1,
                message: "missing front matter opening '+++'".to_string(),
            });
        }
    };

    // Find closing +++
    let mut end_idx = None;
    for (idx, line) in lines.iter().enumerate().skip(start_idx + 1) {
        if line.trim() == "+++" {
            end_idx = Some(idx);
            break;
        }
    }

    let end_idx = match end_idx {
        Some(idx) => idx,
        None => {
            return Err(ParseError {
                file: file_path.to_path_buf(),
                line: lines.len(),
                message: "unclosed front matter: missing closing '+++'".to_string(),
            });
        }
    };

    let toml_str = lines[(start_idx + 1)..end_idx].join("\n");
    let front_matter: FrontMatter = toml::from_str(&toml_str).map_err(|err| {
        let line_offset = err
            .span()
            .map(|s| toml_str[..s.start].lines().count())
            .unwrap_or(1);
        ParseError {
            file: file_path.to_path_buf(),
            line: start_idx + 1 + line_offset,
            message: format!("invalid TOML in front matter: {}", err.message()),
        }
    })?;

    if front_matter.title.trim().is_empty() {
        return Err(ParseError {
            file: file_path.to_path_buf(),
            line: start_idx + 2,
            message: "front matter field 'title' cannot be empty".to_string(),
        });
    }

    if front_matter.lang.trim().is_empty() {
        return Err(ParseError {
            file: file_path.to_path_buf(),
            line: start_idx + 2,
            message: "front matter field 'lang' cannot be empty".to_string(),
        });
    }

    // Body starts right after the closing +++ line
    let body_start_line = end_idx + 2;

    Ok((
        front_matter,
        content_slice_from_line(content, end_idx + 1),
        body_start_line,
    ))
}

fn content_slice_from_line(content: &str, line_idx: usize) -> &str {
    let mut current_line = 0;
    for (byte_offset, c) in content.char_indices() {
        if current_line == line_idx {
            return &content[byte_offset..];
        }
        if c == '\n' {
            current_line += 1;
        }
    }
    ""
}

/// Parses Markdown content into individual `NoteEntry` objects split by `## ` headings.
pub fn parse_entries(
    body: &str,
    start_line_offset: usize,
    file_path: &Path,
) -> Result<Vec<NoteEntry>, ParseError> {
    let lines: Vec<&str> = body.lines().collect();
    let mut entries = Vec::new();

    let mut current_title: Option<String> = None;
    let mut current_title_line = 0;
    let mut current_lines: Vec<&str> = Vec::new();

    let mut in_code_block = false;

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let absolute_line = start_line_offset + idx;

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
        }

        // Detect ## heading outside code blocks
        if !in_code_block && trimmed.starts_with("## ") {
            if let Some(title) = current_title.take() {
                let entry = build_entry(&title, &current_lines, current_title_line, file_path)?;
                entries.push(entry);
                current_lines.clear();
            }

            let title_text = trimmed[3..].trim().to_string();
            if title_text.is_empty() {
                return Err(ParseError {
                    file: file_path.to_path_buf(),
                    line: absolute_line,
                    message: "entry heading '##' has an empty title".to_string(),
                });
            }
            current_title = Some(title_text);
            current_title_line = absolute_line;
            continue;
        }

        if current_title.is_some() {
            current_lines.push(line);
        }
    }

    if in_code_block {
        return Err(ParseError {
            file: file_path.to_path_buf(),
            line: start_line_offset + lines.len(),
            message: "unclosed code block: missing terminating '```'".to_string(),
        });
    }

    if let Some(title) = current_title.take() {
        let entry = build_entry(&title, &current_lines, current_title_line, file_path)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn build_entry(
    title: &str,
    lines: &[&str],
    line_number: usize,
    file_path: &Path,
) -> Result<NoteEntry, ParseError> {
    let id = slugify(title);

    let mut description_lines = Vec::new();
    let mut code_blocks = Vec::new();
    let mut in_code = false;
    let mut current_code_lang = String::new();
    let mut current_code_lines = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if !in_code {
                in_code = true;
                current_code_lang = trimmed.trim_start_matches('`').trim().to_string();
                current_code_lines.clear();
            } else {
                in_code = false;
                code_blocks.push(CodeBlock {
                    language: current_code_lang.clone(),
                    code: current_code_lines.join("\n"),
                });
                current_code_lines.clear();
                current_code_lang.clear();
            }
            continue;
        }

        if in_code {
            current_code_lines.push(*line);
        } else if code_blocks.is_empty() && !trimmed.is_empty() {
            description_lines.push(trimmed);
        }
    }

    if in_code {
        return Err(ParseError {
            file: file_path.to_path_buf(),
            line: line_number + lines.len(),
            message: format!("unclosed code block in entry '{}'", title),
        });
    }

    let description = description_lines.join(" ");
    let raw_markdown = format!("## {}\n{}", title, lines.join("\n"));

    Ok(NoteEntry {
        id,
        title: title.to_string(),
        description,
        code_blocks,
        raw_markdown,
        line_number,
    })
}

/// Generates a clean URL and DOM slug from a heading title.
pub fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = true;

    for c in title.chars() {
        if c.is_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        "entry".to_string()
    } else {
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Short declaration"), "short-declaration");
        assert_eq!(
            slugify("Variables & Mutability (Go 1.22+)"),
            "variables-mutability-go-1-22"
        );
        assert_eq!(slugify("---"), "entry");
    }

    #[test]
    fn test_parse_valid_note() {
        let content = r#"+++
title = "Variables, constants, mutability"
lang = "go"
part = 2
tags = ["var", "const", "short declaration"]
version = "Go 1.22+"
+++

## Short declaration
One line of explanation at most.

```go
x := 5
```

## Multiple declarations
Declare multiple variables in a block.

```go
var (
    a = 1
    b = 2
)
```
"#;
        let note = parse_note(content, Path::new("go/02-variables.md"), SystemTime::now())
            .expect("should parse valid note");

        assert_eq!(note.title, "Variables, constants, mutability");
        assert_eq!(note.lang, "go");
        assert_eq!(note.part, 2);
        assert_eq!(note.version.as_deref(), Some("Go 1.22+"));
        assert_eq!(note.tags, vec!["var", "const", "short declaration"]);
        assert_eq!(note.entries.len(), 2);

        assert_eq!(note.entries[0].title, "Short declaration");
        assert_eq!(note.entries[0].id, "short-declaration");
        assert_eq!(
            note.entries[0].description,
            "One line of explanation at most."
        );
        assert_eq!(note.entries[0].code_blocks.len(), 1);
        assert_eq!(note.entries[0].code_blocks[0].language, "go");
        assert_eq!(note.entries[0].code_blocks[0].code, "x := 5");

        assert_eq!(note.entries[1].title, "Multiple declarations");
        assert_eq!(note.entries[1].id, "multiple-declarations");
        assert_eq!(note.entries[1].code_blocks.len(), 1);
    }

    #[test]
    fn test_missing_front_matter() {
        let content = "## Missing front matter\nNo +++ tags.";
        let err = parse_note(content, Path::new("test.md"), SystemTime::now()).unwrap_err();
        assert!(err.message.contains("expected front matter opening '+++'"));
    }

    #[test]
    fn test_unclosed_code_block() {
        let content = r#"+++
title = "Test"
lang = "rust"
part = 1
+++

## Entry
Unclosed snippet.

```rust
let x = 1;
"#;
        let err = parse_note(content, Path::new("rust/01-test.md"), SystemTime::now()).unwrap_err();
        assert!(err.message.contains("unclosed code block"));
    }
}
