use crate::notes::model::{Language, LanguagesConfig, NotesDatabase};
use crate::notes::parser::parse_note;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Loads all notes from `notes_dir` into an in-memory `NotesDatabase`.
///
/// Returns the parsed database and a list of non-fatal warning messages for any notes
/// that failed to parse.
pub fn load_notes_dir(notes_dir: &Path) -> (NotesDatabase, Vec<String>) {
    let mut db = NotesDatabase::new();
    let mut warnings = Vec::new();

    if !notes_dir.is_dir() {
        warnings.push(format!(
            "Notes directory '{}' does not exist or is not a directory.",
            notes_dir.display()
        ));
        return (db, warnings);
    }

    // 1. Load languages.toml if available
    let languages_file = notes_dir.join("languages.toml");
    let mut known_languages = Vec::new();

    if languages_file.is_file() {
        match fs::read_to_string(&languages_file) {
            Ok(content) => match toml::from_str::<LanguagesConfig>(&content) {
                Ok(cfg) => known_languages = cfg.languages,
                Err(err) => {
                    warnings.push(format!("Warning: Failed to parse languages.toml: {}", err));
                }
            },
            Err(err) => {
                warnings.push(format!("Warning: Could not read languages.toml: {}", err));
            }
        }
    }

    // 2. Discover all language subdirectories dynamically (fully agnostic)
    let read_dir = match fs::read_dir(notes_dir) {
        Ok(rd) => rd,
        Err(err) => {
            warnings.push(format!("Warning: Failed to read notes directory: {}", err));
            return (db, warnings);
        }
    };

    let mut discovered_dirs = Vec::new();
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if !dir_name.starts_with('.') {
                discovered_dirs.push(path);
            }
        }
    }
    discovered_dirs.sort();

    // 3. Register discovered languages that were not in languages.toml
    for lang_dir in &discovered_dirs {
        let lang_id = lang_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        if !known_languages
            .iter()
            .any(|l| l.id.eq_ignore_ascii_case(&lang_id))
        {
            known_languages.push(Language {
                id: lang_id.clone(),
                name: title_case(&lang_id),
                order: 999,
                version: None,
            });
        }
    }

    // Sort languages by configured order, then id
    known_languages.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.id.cmp(&b.id)));
    db.languages = known_languages;

    // 4. Load notes from each discovered directory
    for lang_dir in discovered_dirs {
        let entries = match fs::read_dir(&lang_dir) {
            Ok(rd) => rd,
            Err(err) => {
                warnings.push(format!(
                    "Warning: Could not read directory '{}': {}",
                    lang_dir.display(),
                    err
                ));
                continue;
            }
        };

        let mut md_files = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
                md_files.push(path);
            }
        }
        md_files.sort();

        for file_path in md_files {
            let modified = fs::metadata(&file_path)
                .and_then(|m| m.modified())
                .unwrap_or_else(|_| SystemTime::now());

            match fs::read_to_string(&file_path) {
                Ok(content) => match parse_note(&content, &file_path, modified) {
                    Ok(note_part) => {
                        db.parts.push(note_part);
                    }
                    Err(err) => {
                        warnings.push(format!(
                            "Warning: Failed to parse {}: {}",
                            err.file.display(),
                            err.message
                        ));
                    }
                },
                Err(err) => {
                    warnings.push(format!(
                        "Warning: Could not read file '{}': {}",
                        file_path.display(),
                        err
                    ));
                }
            }
        }
    }

    // Sort parts by language order, then part number
    let lang_orders: HashMap<String, u32> = db
        .languages
        .iter()
        .map(|l| (l.id.to_ascii_lowercase(), l.order))
        .collect();

    db.parts.sort_by(|a, b| {
        let order_a = lang_orders
            .get(&a.lang.to_ascii_lowercase())
            .copied()
            .unwrap_or(999);
        let order_b = lang_orders
            .get(&b.lang.to_ascii_lowercase())
            .copied()
            .unwrap_or(999);
        order_a
            .cmp(&order_b)
            .then_with(|| a.lang.cmp(&b.lang))
            .then_with(|| a.part.cmp(&b.part))
    });

    (db, warnings)
}

/// Helper to capitalize an identifier for fallback display name (e.g. "python" -> "Python").
fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Checks the latest modification time across all `.md` files in `notes_dir`.
/// Used by hot reload mechanisms to detect file changes.
pub fn get_latest_modified(notes_dir: &Path) -> Option<SystemTime> {
    let mut latest = None;

    if let Ok(rd) = fs::read_dir(notes_dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(sub_rd) = fs::read_dir(&path) {
                    for sub_entry in sub_rd.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.extension().and_then(|e| e.to_str()) == Some("md") {
                            if let Ok(meta) = fs::metadata(&sub_path) {
                                if let Ok(mtime) = meta.modified() {
                                    latest = match latest {
                                        None => Some(mtime),
                                        Some(prev) if mtime > prev => Some(mtime),
                                        Some(prev) => Some(prev),
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    latest
}
