use std::fs;
use std::path::{Path, PathBuf};

use crate::notes::parser::parse_front_matter;

/// Summary of an import operation when adding notes to the default notes directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddNotesSummary {
    pub notes_added: Vec<String>,
    pub total_files_copied: usize,
    pub target_dir: PathBuf,
}

/// Discovers all existing note/language names present in the target directory (case-insensitive).
pub fn list_existing_note_names(target_notes_dir: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(entries) = fs::read_dir(target_notes_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    names.push(name);
                }
            }
        }
    }
    names
}

/// Recursively copies all files from `src` to `dst`.
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<usize> {
    fs::create_dir_all(dst)?;
    let mut count = 0;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            count += copy_dir_all(&src_path, &dst_path)?;
        } else if ty.is_file() {
            fs::copy(&src_path, &dst_path)?;
            count += 1;
        }
    }
    Ok(count)
}

/// Determines the language name for a folder directly containing markdown notes.
fn detect_single_note_name(dir: &Path, custom_name: Option<&str>) -> String {
    if let Some(name) = custom_name {
        let trimmed = name.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    // Inspect first .md file for `lang = "..."` front matter
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok((fm, _, _)) = parse_front_matter(&content, &path) {
                        if !fm.lang.trim().is_empty() {
                            return fm.lang.trim().to_string();
                        }
                    }
                }
            }
        }
    }

    // Fall back to directory file name
    dir.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("custom")
        .to_string()
}

/// Adds a notes folder (either a single language folder or a collection of language folders)
/// to the target notes directory (defaults to ~/.mycheatsheet/notes).
///
/// Ensures no two notes share the same name (case-insensitive comparison).
pub fn add_notes_folder(
    src_path: &Path,
    custom_name: Option<&str>,
    target_notes_dir: &Path,
) -> Result<AddNotesSummary, String> {
    if !src_path.exists() {
        return Err(format!(
            "Source path '{}' does not exist.",
            src_path.display()
        ));
    }

    if !src_path.is_dir() {
        return Err(format!(
            "Source path '{}' is not a directory.",
            src_path.display()
        ));
    }

    fs::create_dir_all(target_notes_dir).map_err(|e| {
        format!(
            "Failed to create notes destination directory '{}': {}",
            target_notes_dir.display(),
            e
        )
    })?;

    let existing_names = list_existing_note_names(target_notes_dir);

    // Check if src_path directly contains markdown files
    let mut direct_md_files = Vec::new();
    let mut sub_dirs = Vec::new();

    if let Ok(entries) = fs::read_dir(src_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(ty) = entry.file_type() {
                if ty.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
                    direct_md_files.push(path);
                } else if ty.is_dir() {
                    sub_dirs.push(path);
                }
            }
        }
    }

    if !direct_md_files.is_empty() {
        // Case A: Single language note folder
        let note_name = detect_single_note_name(src_path, custom_name);

        if existing_names
            .iter()
            .any(|n| n.eq_ignore_ascii_case(&note_name))
        {
            return Err(format!(
                "A note named '{}' already exists in {}. No two notes can have the same name.",
                note_name,
                target_notes_dir.display()
            ));
        }

        let dest_folder = target_notes_dir.join(&note_name);
        fs::create_dir_all(&dest_folder).map_err(|e| {
            format!(
                "Failed to create note folder '{}': {}",
                dest_folder.display(),
                e
            )
        })?;

        let mut files_copied = 0;
        for file in direct_md_files {
            if let Some(file_name) = file.file_name() {
                let dest_file = dest_folder.join(file_name);
                fs::copy(&file, &dest_file).map_err(|e| {
                    format!(
                        "Failed to copy note file '{}' to '{}': {}",
                        file.display(),
                        dest_file.display(),
                        e
                    )
                })?;
                files_copied += 1;
            }
        }

        Ok(AddNotesSummary {
            notes_added: vec![note_name],
            total_files_copied: files_copied,
            target_dir: target_notes_dir.to_path_buf(),
        })
    } else if !sub_dirs.is_empty() {
        // Case B: Collection of language folders
        let mut notes_to_add: Vec<(String, PathBuf)> = Vec::new();

        for dir in &sub_dirs {
            let has_md = fs::read_dir(dir).is_ok_and(|entries| {
                entries
                    .flatten()
                    .any(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
            });

            if has_md {
                let folder_name = dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                if !folder_name.is_empty() {
                    // Check duplicate against existing
                    if existing_names
                        .iter()
                        .any(|n| n.eq_ignore_ascii_case(&folder_name))
                    {
                        return Err(format!(
                            "A note named '{}' already exists in {}. No two notes can have the same name.",
                            folder_name,
                            target_notes_dir.display()
                        ));
                    }

                    // Check duplicate within the batch
                    if notes_to_add
                        .iter()
                        .any(|(n, _)| folder_name.eq_ignore_ascii_case(n))
                    {
                        return Err(format!(
                            "Duplicate note named '{}' found in source directory.",
                            folder_name
                        ));
                    }

                    notes_to_add.push((folder_name, dir.clone()));
                }
            }
        }

        if notes_to_add.is_empty() {
            return Err(format!(
                "No note folders containing markdown files found in '{}'.",
                src_path.display()
            ));
        }

        let mut total_copied = 0;
        let mut added_names: Vec<String> = Vec::new();

        for (name, dir_path) in notes_to_add {
            let dest_dir = target_notes_dir.join(&name);
            let count = copy_dir_all(&dir_path, &dest_dir).map_err(|e| {
                format!(
                    "Failed copying notes from '{}' to '{}': {}",
                    dir_path.display(),
                    dest_dir.display(),
                    e
                )
            })?;
            total_copied += count;
            added_names.push(name);
        }

        // Also copy languages.toml if present in collection root
        let lang_toml = src_path.join("languages.toml");
        if lang_toml.is_file() {
            let dest_toml = target_notes_dir.join("languages.toml");
            let _ = fs::copy(&lang_toml, &dest_toml);
        }

        Ok(AddNotesSummary {
            notes_added: added_names,
            total_files_copied: total_copied,
            target_dir: target_notes_dir.to_path_buf(),
        })
    } else {
        Err(format!(
            "No markdown files or note subdirectories found in '{}'.",
            src_path.display()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_add_single_language_note() {
        let temp_src = tempdir().unwrap();
        let python_dir = temp_src.path().join("python");
        fs::create_dir_all(&python_dir).unwrap();
        fs::write(
            python_dir.join("01-main.md"),
            "+++\ntitle = \"Main\"\nlang = \"python\"\npart = 1\ntags = [\"main\"]\n+++\n## Hello\nWorld\n```python\nprint(1)\n```",
        )
        .unwrap();

        let temp_target = tempdir().unwrap();
        let target_dir = temp_target.path().join("notes");

        let summary = add_notes_folder(&python_dir, None, &target_dir).unwrap();
        assert_eq!(summary.notes_added, vec!["python"]);
        assert_eq!(summary.total_files_copied, 1);
        assert!(target_dir.join("python").join("01-main.md").is_file());
    }

    #[test]
    fn test_add_collection_of_notes() {
        let temp_src = tempdir().unwrap();
        let notes_root = temp_src.path().join("notes");
        let go_dir = notes_root.join("go");
        let rust_dir = notes_root.join("rust");
        fs::create_dir_all(&go_dir).unwrap();
        fs::create_dir_all(&rust_dir).unwrap();

        fs::write(
            go_dir.join("01-main.md"),
            "+++\ntitle = \"Go\"\nlang = \"go\"\npart = 1\ntags = [\"go\"]\n+++\n## G\nDesc\n```go\nfunc main() {}\n```",
        )
        .unwrap();
        fs::write(
            rust_dir.join("01-main.md"),
            "+++\ntitle = \"Rust\"\nlang = \"rust\"\npart = 1\ntags = [\"rust\"]\n+++\n## R\nDesc\n```rust\nfn main() {}\n```",
        )
        .unwrap();

        let temp_target = tempdir().unwrap();
        let target_dir = temp_target.path().join("installed_notes");

        let summary = add_notes_folder(&notes_root, None, &target_dir).unwrap();
        assert_eq!(summary.notes_added.len(), 2);
        assert!(target_dir.join("go").join("01-main.md").is_file());
        assert!(target_dir.join("rust").join("01-main.md").is_file());
    }

    #[test]
    fn test_rejects_duplicate_note_names_case_insensitive() {
        let temp_src = tempdir().unwrap();
        let rust_dir = temp_src.path().join("Rust");
        fs::create_dir_all(&rust_dir).unwrap();
        fs::write(rust_dir.join("01-main.md"), "+++\nlang=\"Rust\"\n+++\n").unwrap();

        let temp_target = tempdir().unwrap();
        let target_dir = temp_target.path().join("notes");
        // Pre-create lowercase "rust"
        fs::create_dir_all(target_dir.join("rust")).unwrap();

        let err = add_notes_folder(&rust_dir, None, &target_dir).unwrap_err();
        assert!(err.contains("already exists"));
        assert!(err.contains("No two notes can have the same name"));
    }

    #[test]
    fn test_rejects_non_existent_source() {
        let target = tempdir().unwrap();
        let err = add_notes_folder(Path::new("/non/existent/path/xyz123"), None, target.path())
            .unwrap_err();
        assert!(err.contains("does not exist"));
    }
}
