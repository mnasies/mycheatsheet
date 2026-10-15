//! Search ranking and filtering shared between Web and TUI.

pub mod index;
pub mod query;
pub mod score;

pub use index::{SearchIndex, SearchResult};
pub use query::{parse_query, tokenize, ParsedQuery};
pub use score::{
    levenshtein_distance, score_token_in_field, WEIGHT_CODE, WEIGHT_DESCRIPTION,
    WEIGHT_ENTRY_TITLE, WEIGHT_PART_TITLE, WEIGHT_TAG,
};
