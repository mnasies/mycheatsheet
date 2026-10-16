use crate::notes::model::NotesDatabase;
use crate::search::query::{parse_query, tokenize};
use crate::search::score::{
    score_token_in_field, WEIGHT_DESCRIPTION, WEIGHT_ENTRY_TITLE, WEIGHT_PART_TITLE, WEIGHT_TAG,
};

/// A single search result item returned to Web or TUI.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub lang: String,
    pub lang_name: String,
    pub part: u32,
    pub part_title: String,
    pub entry_id: String,
    pub entry_title: String,
    pub description: String,
    pub snippet: String,
    pub score: f32,
}

/// An entry indexed with pre-tokenized fields for sub-millisecond query evaluation.
#[derive(Debug, Clone)]
struct IndexedEntry {
    lang: String,
    lang_name: String,
    part: u32,
    part_title: String,
    part_title_tokens: Vec<String>,
    tags_tokens: Vec<String>,
    entry_id: String,
    entry_title: String,
    entry_title_tokens: Vec<String>,
    description: String,
    snippet: String,
    body_haystack: String,
}

/// In-memory search index built across notes data.
#[derive(Debug, Clone, Default)]
pub struct SearchIndex {
    entries: Vec<IndexedEntry>,
}

impl SearchIndex {
    /// Builds an in-memory search index from a loaded `NotesDatabase`.
    pub fn from_database(db: &NotesDatabase) -> Self {
        let mut entries = Vec::new();

        for part in &db.parts {
            let lang_name = db
                .get_language(&part.lang)
                .map(|l| l.name.clone())
                .unwrap_or_else(|| part.lang.clone());

            let part_title_tokens = tokenize(&part.title);
            let tags_tokens: Vec<String> = part.tags.iter().flat_map(|tag| tokenize(tag)).collect();

            for entry in &part.entries {
                let entry_title_tokens = tokenize(&entry.title);

                let snippet = entry
                    .code_blocks
                    .first()
                    .map(|cb| cb.code.clone())
                    .unwrap_or_default();

                let mut body_haystack = entry.description.to_lowercase();
                for cb in &entry.code_blocks {
                    body_haystack.push(' ');
                    body_haystack.push_str(&cb.code.to_lowercase());
                }

                entries.push(IndexedEntry {
                    lang: part.lang.clone(),
                    lang_name: lang_name.clone(),
                    part: part.part,
                    part_title: part.title.clone(),
                    part_title_tokens: part_title_tokens.clone(),
                    tags_tokens: tags_tokens.clone(),
                    entry_id: entry.id.clone(),
                    entry_title: entry.title.clone(),
                    entry_title_tokens,
                    description: entry.description.clone(),
                    snippet,
                    body_haystack,
                });
            }
        }

        SearchIndex { entries }
    }

    /// Total number of indexed entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether index contains zero entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns all indexed entries as search results (useful for default browsing views).
    pub fn all_results(&self) -> Vec<SearchResult> {
        self.entries
            .iter()
            .map(|e| SearchResult {
                lang: e.lang.clone(),
                lang_name: e.lang_name.clone(),
                part: e.part,
                part_title: e.part_title.clone(),
                entry_id: e.entry_id.clone(),
                entry_title: e.entry_title.clone(),
                description: e.description.clone(),
                snippet: e.snippet.clone(),
                score: 1.0,
            })
            .collect()
    }

    /// Searches the index with optional language override.
    ///
    /// Query shortcuts like `go: defer` or `lang:go defer` are parsed automatically.
    /// If `lang_override` is specified (e.g. from CLI `-l go` or UI tab), it takes priority.
    pub fn search(&self, query_str: &str, lang_override: Option<&str>) -> Vec<SearchResult> {
        let parsed = parse_query(query_str);

        // Effective language filter: lang_override wins if specified, otherwise parsed from query
        let active_lang = lang_override
            .map(|s| s.to_ascii_lowercase())
            .or(parsed.lang_filter);

        // If query tokens are empty:
        if parsed.tokens.is_empty() {
            if let Some(ref target_lang) = active_lang {
                // Return all entries for the filtered language sorted by part
                return self
                    .entries
                    .iter()
                    .filter(|e| e.lang.eq_ignore_ascii_case(target_lang))
                    .map(|e| SearchResult {
                        lang: e.lang.clone(),
                        lang_name: e.lang_name.clone(),
                        part: e.part,
                        part_title: e.part_title.clone(),
                        entry_id: e.entry_id.clone(),
                        entry_title: e.entry_title.clone(),
                        description: e.description.clone(),
                        snippet: e.snippet.clone(),
                        score: 1.0,
                    })
                    .collect();
            }
            return Vec::new();
        }

        let mut results = Vec::new();

        for entry in &self.entries {
            // Apply language filter
            if let Some(ref target_lang) = active_lang {
                if !entry.lang.eq_ignore_ascii_case(target_lang) {
                    continue;
                }
            }

            let mut total_score = 0.0f32;
            let mut matched_token_count = 0;

            for token in &parsed.tokens {
                let title_score = score_token_in_field(token, &entry.entry_title_tokens, true);
                let tag_score = score_token_in_field(token, &entry.tags_tokens, true);
                let part_score = score_token_in_field(token, &entry.part_title_tokens, false);

                let body_score = if entry.body_haystack.contains(token.as_str()) {
                    1.0f32
                } else {
                    0.0f32
                };

                let token_score = (title_score * WEIGHT_ENTRY_TITLE)
                    + (tag_score * WEIGHT_TAG)
                    + (part_score * WEIGHT_PART_TITLE)
                    + (body_score * WEIGHT_DESCRIPTION);

                if token_score > 0.0 {
                    total_score += token_score;
                    matched_token_count += 1;
                }
            }

            if matched_token_count > 0 {
                // Multi-token match bonus: boosts items matching all search terms
                let coverage_ratio = matched_token_count as f32 / parsed.tokens.len() as f32;
                let final_score = total_score * (1.0 + coverage_ratio);

                results.push(SearchResult {
                    lang: entry.lang.clone(),
                    lang_name: entry.lang_name.clone(),
                    part: entry.part,
                    part_title: entry.part_title.clone(),
                    entry_id: entry.entry_id.clone(),
                    entry_title: entry.entry_title.clone(),
                    description: entry.description.clone(),
                    snippet: entry.snippet.clone(),
                    score: final_score,
                });
            }
        }

        // Sort descending by score, then ascending by part number, then title
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.part.cmp(&b.part))
                .then_with(|| a.entry_title.cmp(&b.entry_title))
        });

        results
    }
}
