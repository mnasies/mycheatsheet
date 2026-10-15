//! Notes loader, parser, model, and validation.
//! Implements the language-agnostic Notes Protocol.

pub mod loader;
pub mod model;
pub mod parser;
pub mod validation;

pub use loader::{get_latest_modified, load_notes_dir};
pub use model::{CodeBlock, FrontMatter, Language, NoteEntry, NotePart, NotesDatabase};
pub use parser::{parse_front_matter, parse_note, slugify, ParseError};
pub use validation::{
    parse_filename_part_prefix, validate_notes_dir, ValidationError, ValidationSummary,
};
