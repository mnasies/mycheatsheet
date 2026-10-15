use crate::notes::parser::{parse_front_matter, parse_note};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// A protocol validation error suitable for compiler-style diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub file: PathBuf,
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.file.display(), self.line, self.message)
    }
}

/// Summary report after validating all notes in a directory.
#[derive(Debug, Clone, Default)]
pub struct ValidationSummary {
    pub languages_count: usize,
    pub notes_checked: usize,
    pub entries_checked: usize,
    pub errors: Vec<ValidationError>,
}

impl ValidationSummary {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Validates all note files in `notes_dir` against the Notes Protocol.
pub fn validate_notes_dir(notes_dir: &Path) -> ValidationSummary {
    let mut summary = ValidationSummary::default();

    if !notes_dir.is_dir() {
        summary.errors.push(ValidationError {
            file: notes_dir.to_path_buf(),
            line: 1,
            message: format!("notes path '{}' is not a directory", notes_dir.display()),
        });
        return summary;
    }

    // Validate languages.toml if it exists
    let lang_toml = notes_dir.join("languages.toml");
    if lang_toml.is_file() {
        if let Ok(content) = fs::read_to_string(&lang_toml) {
            if let Err(err) = toml::from_str::<crate::notes::model::LanguagesConfig>(&content) {
                summary.errors.push(ValidationError {
                    file: lang_toml.clone(),
                    line: 1,
                    message: format!("languages.toml syntax error: {}", err),
                });
            }
        }
    }

    let read_dir = match fs::read_dir(notes_dir) {
        Ok(rd) => rd,
        Err(err) => {
            summary.errors.push(ValidationError {
                file: notes_dir.to_path_buf(),
                line: 1,
                message: format!("unable to read notes directory: {}", err),
            });
            return summary;
        }
    };

    let mut language_dirs = Vec::new();
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            // Ignore hidden directories or metadata directories
            if !dir_name.starts_with('.') {
                language_dirs.push(path);
            }
        }
    }

    language_dirs.sort();
    summary.languages_count = language_dirs.len();

    for lang_dir in language_dirs {
        let lang_id = lang_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let entries = match fs::read_dir(&lang_dir) {
            Ok(rd) => rd,
            Err(err) => {
                summary.errors.push(ValidationError {
                    file: lang_dir.clone(),
                    line: 1,
                    message: format!("failed to read directory: {}", err),
                });
                continue;
            }
        };

        let mut note_files = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
                note_files.push(path);
            }
        }
        note_files.sort();

        for file_path in note_files {
            summary.notes_checked += 1;
            validate_single_note_file(&file_path, &lang_id, &mut summary);
        }
    }

    summary
}

fn validate_single_note_file(
    file_path: &Path,
    expected_lang: &str,
    summary: &mut ValidationSummary,
) {
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();

    // Protocol rule: Filename must match <number>-<slug>.md
    let (part_from_filename, _) = match parse_filename_part_prefix(file_name) {
        Some(res) => res,
        None => {
            summary.errors.push(ValidationError {
                file: file_path.to_path_buf(),
                line: 1,
                message: format!(
                    "filename '{}' does not match protocol pattern '<number>-<slug>.md' (e.g. '01-program-structure.md')",
                    file_name
                ),
            });
            return;
        }
    };

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(err) => {
            summary.errors.push(ValidationError {
                file: file_path.to_path_buf(),
                line: 1,
                message: format!("failed to read file: {}", err),
            });
            return;
        }
    };

    // Protocol rule: Validate front matter
    let (front_matter, _, _) = match parse_front_matter(&content, file_path) {
        Ok(fm) => fm,
        Err(err) => {
            summary.errors.push(ValidationError {
                file: file_path.to_path_buf(),
                line: err.line,
                message: err.message,
            });
            return;
        }
    };

    // Protocol rule: lang must match directory
    if !front_matter.lang.eq_ignore_ascii_case(expected_lang) {
        summary.errors.push(ValidationError {
            file: file_path.to_path_buf(),
            line: 2,
            message: format!(
                "front matter lang '{}' does not match directory name '{}'",
                front_matter.lang, expected_lang
            ),
        });
    }

    // Protocol rule: part must match filename prefix
    if front_matter.part != part_from_filename {
        summary.errors.push(ValidationError {
            file: file_path.to_path_buf(),
            line: 3,
            message: format!(
                "front matter part '{}' does not match filename number '{}'",
                front_matter.part, part_from_filename
            ),
        });
    }

    // Protocol rule: Full note parsing and entry validation
    match parse_note(&content, file_path, SystemTime::now()) {
        Ok(note_part) => {
            if note_part.entries.is_empty() {
                summary.errors.push(ValidationError {
                    file: file_path.to_path_buf(),
                    line: 1,
                    message: "note contains no '##' entries".to_string(),
                });
            } else {
                for entry in &note_part.entries {
                    summary.entries_checked += 1;
                    if entry.code_blocks.is_empty() {
                        summary.errors.push(ValidationError {
                            file: file_path.to_path_buf(),
                            line: entry.line_number,
                            message: format!(
                                "entry '{}' has no code blocks; at least one fenced code block is required",
                                entry.title
                            ),
                        });
                    }
                }
            }
        }
        Err(err) => {
            summary.errors.push(ValidationError {
                file: file_path.to_path_buf(),
                line: err.line,
                message: err.message,
            });
        }
    }
}

/// Parses the leading integer from filenames such as `01-program-structure.md`.
pub fn parse_filename_part_prefix(filename: &str) -> Option<(u32, &str)> {
    let base = filename.strip_suffix(".md")?;
    let (num_str, slug) = base.split_once('-')?;
    let num: u32 = num_str.parse().ok()?;
    if slug.is_empty() {
        return None;
    }
    Some((num, slug))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filename_part_prefix() {
        assert_eq!(
            parse_filename_part_prefix("01-program-structure.md"),
            Some((1, "program-structure"))
        );
        assert_eq!(
            parse_filename_part_prefix("20-build-tools.md"),
            Some((20, "build-tools"))
        );
        assert_eq!(parse_filename_part_prefix("invalid.md"), None);
        assert_eq!(parse_filename_part_prefix("01-.md"), None);
    }
}
