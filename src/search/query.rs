/// Structured representation of a parsed search query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedQuery {
    /// Optional language filter extracted from query (e.g. `go:` or `lang:go`).
    pub lang_filter: Option<String>,
    /// Lowercase normalized search tokens.
    pub tokens: Vec<String>,
    /// Original raw query string.
    pub raw: String,
}

impl ParsedQuery {
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

/// Parses a user search query, extracting language prefix shortcuts and tokenizing.
///
/// Supported prefix shortcuts:
/// - `lang:go <terms>`
/// - `lang:rust <terms>`
/// - `go: <terms>`
/// - `cpp: <terms>`
/// - `rust: <terms>`
pub fn parse_query(raw_input: &str) -> ParsedQuery {
    let trimmed = raw_input.trim();
    if trimmed.is_empty() {
        return ParsedQuery {
            lang_filter: None,
            tokens: Vec::new(),
            raw: String::new(),
        };
    }

    let mut lang_filter = None;
    let mut remainder = trimmed;

    // Check for `lang:<id>` prefix
    if let Some(rest) = trimmed.strip_prefix("lang:") {
        if let Some((lang_id, after_lang)) =
            rest.split_once(|c: char| c.is_whitespace() || c == ':')
        {
            let clean_id = lang_id.trim();
            if is_valid_lang_ident(clean_id) {
                lang_filter = Some(clean_id.to_ascii_lowercase());
                remainder = after_lang.trim();
            }
        } else if is_valid_lang_ident(rest.trim()) {
            lang_filter = Some(rest.trim().to_ascii_lowercase());
            remainder = "";
        }
    } else if let Some((potential_lang, after_colon)) = trimmed.split_once(':') {
        // Check for `<id>:` prefix, e.g. `go: defer` or `rust: trait`
        let clean_id = potential_lang.trim();
        if is_valid_lang_ident(clean_id) {
            lang_filter = Some(clean_id.to_ascii_lowercase());
            remainder = after_colon.trim();
        }
    }

    let tokens = tokenize(remainder);

    ParsedQuery {
        lang_filter,
        tokens,
        raw: trimmed.to_string(),
    }
}

/// Tokenizes text into lowercase alphanumeric or symbol words.
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for word in input.split(|c: char| c.is_whitespace() || c == ',' || c == ';') {
        let clean = word
            .trim_matches(|c: char| {
                c == '"' || c == '\'' || c == '.' || c == '(' || c == ')' || c == '[' || c == ']'
            })
            .to_lowercase();
        if !clean.is_empty() {
            tokens.push(clean);
        }
    }
    tokens
}

fn is_valid_lang_ident(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 20
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '#' || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_with_lang_prefix() {
        let q = parse_query("lang:go goroutines");
        assert_eq!(q.lang_filter.as_deref(), Some("go"));
        assert_eq!(q.tokens, vec!["goroutines"]);

        let q2 = parse_query("rust: lifetimes and borrowing");
        assert_eq!(q2.lang_filter.as_deref(), Some("rust"));
        assert_eq!(q2.tokens, vec!["lifetimes", "and", "borrowing"]);

        let q3 = parse_query("cpp: std::unique_ptr");
        assert_eq!(q3.lang_filter.as_deref(), Some("cpp"));
        assert_eq!(q3.tokens, vec!["std::unique_ptr"]);
    }

    #[test]
    fn test_parse_plain_query() {
        let q = parse_query("hash map lookup");
        assert_eq!(q.lang_filter, None);
        assert_eq!(q.tokens, vec!["hash", "map", "lookup"]);
    }

    #[test]
    fn test_parse_empty_query() {
        let q = parse_query("   ");
        assert_eq!(q.lang_filter, None);
        assert!(q.tokens.is_empty());
    }

    #[test]
    fn test_tokenize_symbols_and_unicode() {
        let tokens = tokenize("C++20 'lambda' & π-calculus");
        assert_eq!(tokens, vec!["c++20", "lambda", "&", "π-calculus"]);
    }
}
