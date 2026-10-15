use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// Metadata for a programming language, either loaded from `languages.toml`
/// or dynamically inferred from a note directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language {
    /// Unique lowercase identifier matching directory name (e.g., "go", "rust").
    pub id: String,
    /// Human-friendly display name (e.g., "Go", "Rust", "C++", "JavaScript").
    pub name: String,
    /// Ordering index for UI presentation.
    #[serde(default = "default_order")]
    pub order: u32,
    /// Target language version label (e.g., "Go 1.22+", "Rust 2021").
    #[serde(default)]
    pub version: Option<String>,
}

fn default_order() -> u32 {
    999
}

/// Root configuration schema for `languages.toml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagesConfig {
    #[serde(default)]
    pub languages: Vec<Language>,
}

/// Front matter metadata extracted from `+++ ... +++` in note files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub lang: String,
    pub part: u32,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub version: Option<String>,
}

/// A fenced code block extracted from an entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeBlock {
    pub language: String,
    pub code: String,
}

/// An individual cheatsheet entry under an `## ` heading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteEntry {
    /// Unique identifier for deep linking, derived from slugifying the title.
    pub id: String,
    /// The entry title (from the `## ` heading).
    pub title: String,
    /// Short 1-2 sentence description following the heading.
    pub description: String,
    /// Extracted code blocks within this entry.
    pub code_blocks: Vec<CodeBlock>,
    /// Raw Markdown content of this entry for rendering.
    pub raw_markdown: String,
    /// 1-indexed line number where this entry begins in the source file.
    pub line_number: usize,
}

/// A single note file representing a numbered Part for a language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotePart {
    pub lang: String,
    pub part: u32,
    pub title: String,
    pub version: Option<String>,
    pub tags: Vec<String>,
    pub entries: Vec<NoteEntry>,
    pub file_path: PathBuf,
    #[serde(skip, default = "SystemTime::now")]
    pub modified: SystemTime,
}

/// The in-memory database of all loaded languages and note parts.
#[derive(Debug, Clone, Default)]
pub struct NotesDatabase {
    pub languages: Vec<Language>,
    pub parts: Vec<NotePart>,
}

impl NotesDatabase {
    /// Creates an empty database.
    pub fn new() -> Self {
        Self::default()
    }

    /// Finds language metadata by identifier.
    pub fn get_language(&self, id: &str) -> Option<&Language> {
        self.languages
            .iter()
            .find(|l| l.id.eq_ignore_ascii_case(id))
    }

    /// Returns all parts belonging to the specified language, sorted by part number.
    pub fn get_parts_for_lang(&self, lang: &str) -> Vec<&NotePart> {
        let mut parts: Vec<&NotePart> = self
            .parts
            .iter()
            .filter(|p| p.lang.eq_ignore_ascii_case(lang))
            .collect();
        parts.sort_by_key(|p| p.part);
        parts
    }

    /// Returns a specific part by language and part number.
    pub fn get_part(&self, lang: &str, part_num: u32) -> Option<&NotePart> {
        self.parts
            .iter()
            .find(|p| p.lang.eq_ignore_ascii_case(lang) && p.part == part_num)
    }

    /// Total count of all parsed entries across all notes.
    pub fn total_entries_count(&self) -> usize {
        self.parts.iter().map(|p| p.entries.len()).sum()
    }
}
