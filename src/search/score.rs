pub const WEIGHT_ENTRY_TITLE: f32 = 12.0;
pub const WEIGHT_TAG: f32 = 8.0;
pub const WEIGHT_PART_TITLE: f32 = 4.0;
pub const WEIGHT_DESCRIPTION: f32 = 2.0;
pub const WEIGHT_CODE: f32 = 1.0;

/// Computes the similarity score of a single query token against a collection of field tokens.
///
/// Returns a multiplier in `[0.0, 1.0]`:
/// - 1.0 for exact token match
/// - 0.75 for prefix match
/// - 0.50 for single-character typo (Levenshtein distance 1 on length >= 4)
/// - 0.35 for two-character typo (Levenshtein distance 2 on length >= 7)
/// - 0.0 for no match
pub fn score_token_in_field(query_token: &str, field_tokens: &[String], allow_typo: bool) -> f32 {
    let mut best_score = 0.0f32;
    let q_len = query_token.len();
    let (q_mask, q_unique) = if allow_typo && q_len >= 4 {
        char_mask(query_token)
    } else {
        (0, 0)
    };

    for target in field_tokens {
        // 1. Exact match
        if query_token == target {
            return 1.0;
        }

        let t_len = target.len();

        // 2. Prefix match
        if target.starts_with(query_token) && q_len >= 2 {
            let prefix_score = 0.75 + (0.1 * (q_len as f32 / t_len.max(1) as f32));
            if prefix_score > best_score {
                best_score = prefix_score;
            }
            continue;
        }

        // 3. Typo tolerance via Levenshtein edit distance (restricted to titles/tags)
        if allow_typo && q_len >= 4 && best_score < 0.50 {
            let max_allowed_dist = if q_len >= 7 { 2 } else { 1 };
            let len_diff = q_len.abs_diff(t_len);
            if len_diff <= max_allowed_dist {
                let (t_mask, _) = char_mask(target);
                let common = (q_mask & t_mask).count_ones();
                // Necessary condition: any string within edit distance D must share at least (unique - D) characters
                if common + (max_allowed_dist as u32) >= q_unique {
                    let dist = levenshtein_distance(query_token, target);
                    if dist == 1 && 0.50 > best_score {
                        best_score = 0.50;
                    } else if dist == 2 && q_len >= 7 && 0.35 > best_score {
                        best_score = 0.35;
                    }
                }
            }
        }
    }

    best_score
}

#[inline]
fn char_mask(s: &str) -> (u64, u32) {
    let mut mask = 0u64;
    for &b in s.as_bytes() {
        if b.is_ascii_lowercase() {
            mask |= 1u64 << (b - b'a');
        } else if b.is_ascii_alphanumeric() {
            mask |= 1u64 << 27;
        }
    }
    (mask, mask.count_ones())
}

/// Computes Levenshtein edit distance between two strings with zero heap allocation
/// for common token lengths (<= 64 chars).
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    if s1.is_ascii() && s2.is_ascii() {
        return levenshtein_bytes(s1.as_bytes(), s2.as_bytes());
    }

    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    let len1 = v1.len();
    let len2 = v2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    if len1 < len2 {
        return levenshtein_distance(s2, s1);
    }

    let mut prev_row: Vec<usize> = (0..=len2).collect();
    let mut curr_row: Vec<usize> = vec![0; len2 + 1];

    for (i, &c1) in v1.iter().enumerate() {
        curr_row[0] = i + 1;
        for (j, &c2) in v2.iter().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            curr_row[j + 1] = (prev_row[j + 1] + 1)
                .min(curr_row[j] + 1)
                .min(prev_row[j] + cost);
        }
        prev_row.copy_from_slice(&curr_row);
    }

    curr_row[len2]
}

/// Zero-allocation Levenshtein distance on ASCII byte slices.
fn levenshtein_bytes(b1: &[u8], b2: &[u8]) -> usize {
    let len1 = b1.len();
    let len2 = b2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    if len1 < len2 {
        return levenshtein_bytes(b2, b1);
    }

    // Stack allocated buffer for words under 64 characters
    if len2 <= 63 {
        let mut prev_row = [0usize; 64];
        let mut curr_row = [0usize; 64];

        for (j, item) in prev_row.iter_mut().enumerate().take(len2 + 1) {
            *item = j;
        }

        for (i, &byte1) in b1.iter().enumerate() {
            curr_row[0] = i + 1;
            let mut min_in_row = curr_row[0];
            for (j, &byte2) in b2.iter().enumerate() {
                let cost = if byte1 == byte2 { 0 } else { 1 };
                let val = (prev_row[j + 1] + 1)
                    .min(curr_row[j] + 1)
                    .min(prev_row[j] + cost);
                curr_row[j + 1] = val;
                if val < min_in_row {
                    min_in_row = val;
                }
            }
            if min_in_row > 2 {
                return 3;
            }
            prev_row[..=len2].copy_from_slice(&curr_row[..=len2]);
        }

        curr_row[len2]
    } else {
        // Fallback to heap for unusually long tokens
        let mut prev_row: Vec<usize> = (0..=len2).collect();
        let mut curr_row: Vec<usize> = vec![0; len2 + 1];

        for (i, &byte1) in b1.iter().enumerate() {
            curr_row[0] = i + 1;
            for (j, &byte2) in b2.iter().enumerate() {
                let cost = if byte1 == byte2 { 0 } else { 1 };
                curr_row[j + 1] = (prev_row[j + 1] + 1)
                    .min(curr_row[j] + 1)
                    .min(prev_row[j] + cost);
            }
            prev_row.copy_from_slice(&curr_row);
        }

        curr_row[len2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("goroutine", "goroutine"), 0);
        assert_eq!(levenshtein_distance("goroutin", "goroutine"), 1);
        assert_eq!(levenshtein_distance("rosetta", ""), 7);
        assert_eq!(levenshtein_distance("", "rust"), 4);
    }

    #[test]
    fn test_score_token_exact() {
        let field = vec!["goroutine".to_string(), "channel".to_string()];
        assert_eq!(score_token_in_field("goroutine", &field, true), 1.0);
    }

    #[test]
    fn test_score_token_prefix() {
        let field = vec!["variable".to_string()];
        let score = score_token_in_field("var", &field, true);
        assert!(score >= 0.75);
    }

    #[test]
    fn test_score_token_typo() {
        let field = vec!["goroutine".to_string()];
        let score = score_token_in_field("goroutne", &field, true);
        assert_eq!(score, 0.50);
    }
}
