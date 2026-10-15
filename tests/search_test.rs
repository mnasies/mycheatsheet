use mycheatsheet::notes::model::{CodeBlock, Language, NoteEntry, NotePart, NotesDatabase};
use mycheatsheet::search::SearchIndex;
use std::time::Instant;

fn create_sample_database() -> NotesDatabase {
    let mut db = NotesDatabase::new();
    db.languages = vec![
        Language {
            id: "go".to_string(),
            name: "Go".to_string(),
            order: 1,
            version: Some("Go 1.22+".to_string()),
        },
        Language {
            id: "rust".to_string(),
            name: "Rust".to_string(),
            order: 2,
            version: Some("Rust 2021".to_string()),
        },
    ];

    // Part 1: Go Program Structure
    db.parts.push(NotePart {
        lang: "go".to_string(),
        part: 1,
        title: "Program structure & entry point".to_string(),
        version: Some("Go 1.22+".to_string()),
        tags: vec![
            "main".to_string(),
            "package".to_string(),
            "init".to_string(),
        ],
        entries: vec![
            NoteEntry {
                id: "minimal-executable".to_string(),
                title: "Minimal executable program".to_string(),
                description: "Every standalone Go executable requires package main.".to_string(),
                code_blocks: vec![CodeBlock {
                    language: "go".to_string(),
                    code: "package main\nfunc main() {}".to_string(),
                }],
                raw_markdown: String::new(),
                line_number: 10,
            },
            NoteEntry {
                id: "package-init".to_string(),
                title: "Package initialization with init()".to_string(),
                description: "An init function runs automatically before main.".to_string(),
                code_blocks: vec![CodeBlock {
                    language: "go".to_string(),
                    code: "func init() {}".to_string(),
                }],
                raw_markdown: String::new(),
                line_number: 25,
            },
        ],
        file_path: std::path::PathBuf::from("notes/go/01-program-structure.md"),
        modified: std::time::SystemTime::now(),
    });

    // Part 2: Go Variables & Concurrency
    db.parts.push(NotePart {
        lang: "go".to_string(),
        part: 2,
        title: "Variables and Goroutines".to_string(),
        version: Some("Go 1.22+".to_string()),
        tags: vec![
            "concurrency".to_string(),
            "channel".to_string(),
            "goroutine".to_string(),
        ],
        entries: vec![
            NoteEntry {
                id: "short-declaration".to_string(),
                title: "Short variable declaration".to_string(),
                description: "Use := to declare and assign.".to_string(),
                code_blocks: vec![CodeBlock {
                    language: "go".to_string(),
                    code: "x := 42".to_string(),
                }],
                raw_markdown: String::new(),
                line_number: 10,
            },
            NoteEntry {
                id: "launch-goroutine".to_string(),
                title: "Launching a goroutine".to_string(),
                description: "Prefix function call with go keyword.".to_string(),
                code_blocks: vec![CodeBlock {
                    language: "go".to_string(),
                    code: "go doWork()".to_string(),
                }],
                raw_markdown: String::new(),
                line_number: 20,
            },
        ],
        file_path: std::path::PathBuf::from("notes/go/02-variables.md"),
        modified: std::time::SystemTime::now(),
    });

    // Part 1: Rust Program Structure
    db.parts.push(NotePart {
        lang: "rust".to_string(),
        part: 1,
        title: "Program structure & entry point".to_string(),
        version: Some("Rust 2021".to_string()),
        tags: vec!["entry point".to_string(), "main".to_string()],
        entries: vec![
            NoteEntry {
                id: "fn-main".to_string(),
                title: "Minimal fn main".to_string(),
                description: "Entry point for binary crates.".to_string(),
                code_blocks: vec![CodeBlock {
                    language: "rust".to_string(),
                    code: "fn main() {}".to_string(),
                }],
                raw_markdown: String::new(),
                line_number: 10,
            },
            NoteEntry {
                id: "main-result".to_string(),
                title: "Main returning Result".to_string(),
                description: "Return Result for error propagation.".to_string(),
                code_blocks: vec![CodeBlock {
                    language: "rust".to_string(),
                    code: "fn main() -> Result<(), Box<dyn Error>> {}".to_string(),
                }],
                raw_markdown: String::new(),
                line_number: 20,
            },
        ],
        file_path: std::path::PathBuf::from("notes/rust/01-program-structure.md"),
        modified: std::time::SystemTime::now(),
    });

    db
}

#[test]
fn test_search_ranking_hierarchy() {
    let db = create_sample_database();
    let index = SearchIndex::from_database(&db);

    // Searching "goroutine"
    // Match in title: "Launching a goroutine" (should rank highest)
    // Match in tag: "Variables and Goroutines" tag on part 2 (lower than direct title match)
    let results = index.search("goroutine", None);
    assert!(!results.is_empty());
    assert_eq!(results[0].entry_id, "launch-goroutine");
    assert_eq!(results[0].entry_title, "Launching a goroutine");
}

#[test]
fn test_search_with_language_filter_prefix() {
    let db = create_sample_database();
    let index = SearchIndex::from_database(&db);

    // Query with "go:" prefix
    let results = index.search("go: entry point", None);
    assert!(!results.is_empty());
    for r in &results {
        assert_eq!(r.lang, "go", "all results must be Go");
    }

    // Query with "lang:rust" prefix
    let rust_results = index.search("lang:rust entry point", None);
    assert!(!rust_results.is_empty());
    for r in &rust_results {
        assert_eq!(r.lang, "rust", "all results must be Rust");
    }
}

#[test]
fn test_search_with_lang_override() {
    let db = create_sample_database();
    let index = SearchIndex::from_database(&db);

    // Query mentions "main" which exists in both Go and Rust, but lang_override is "rust"
    let results = index.search("main", Some("rust"));
    assert!(!results.is_empty());
    for r in &results {
        assert_eq!(r.lang, "rust");
    }
}

#[test]
fn test_search_empty_queries() {
    let db = create_sample_database();
    let index = SearchIndex::from_database(&db);

    // Completely empty
    let empty = index.search("", None);
    assert!(empty.is_empty());

    // Empty query with lang override returns all entries for that language
    let go_all = index.search("", Some("go"));
    assert_eq!(go_all.len(), 4);
    for r in &go_all {
        assert_eq!(r.lang, "go");
    }
}

#[test]
fn test_search_typo_tolerance() {
    let db = create_sample_database();
    let index = SearchIndex::from_database(&db);

    // Query "goroutin" (missing final e) should find "Launching a goroutine"
    let results = index.search("goroutin", None);
    assert!(!results.is_empty());
    assert_eq!(results[0].entry_id, "launch-goroutine");
}

#[test]
fn test_search_unicode_support() {
    let mut db = create_sample_database();
    db.parts[0].entries.push(NoteEntry {
        id: "unicode-entry".to_string(),
        title: "Über-fast string scanning & π calculation".to_string(),
        description: "Handles UTF-8 symbols seamlessly.".to_string(),
        code_blocks: vec![CodeBlock {
            language: "go".to_string(),
            code: "pi := 3.14159".to_string(),
        }],
        raw_markdown: String::new(),
        line_number: 50,
    });

    let index = SearchIndex::from_database(&db);
    let results = index.search("über-fast", None);
    assert!(!results.is_empty());
    assert_eq!(results[0].entry_id, "unicode-entry");

    let results2 = index.search("π", None);
    assert!(!results2.is_empty());
    assert_eq!(results2[0].entry_id, "unicode-entry");
}

#[test]
fn test_search_benchmark_2000_entries_under_50ms() {
    let mut db = NotesDatabase::new();
    db.languages = vec![
        Language {
            id: "go".to_string(),
            name: "Go".to_string(),
            order: 1,
            version: None,
        },
        Language {
            id: "rust".to_string(),
            name: "Rust".to_string(),
            order: 2,
            version: None,
        },
        Language {
            id: "cpp".to_string(),
            name: "C++".to_string(),
            order: 3,
            version: None,
        },
        Language {
            id: "js".to_string(),
            name: "JavaScript".to_string(),
            order: 4,
            version: None,
        },
    ];

    let languages = ["go", "rust", "cpp", "js"];
    let sample_titles = [
        "Variables, constants, and mutability",
        "Buffered and unbuffered channels",
        "Pattern matching on enums with guards",
        "Move semantics and rvalue references",
        "Event loop, promises, and async await",
        "Dynamic dispatch with trait objects",
        "Memory allocation with smart pointers",
        "Generic algorithms with concepts",
        "Goroutines and synchronization primitives",
        "Slice capacity, growth, and copy semantics",
    ];

    // Generate 2,000 synthetic entries (200 parts with 10 entries each)
    let mut entry_counter = 0;
    for part_idx in 1..=200 {
        let lang = languages[part_idx % languages.len()];
        let mut entries = Vec::new();

        for e_idx in 0..10 {
            entry_counter += 1;
            let title = format!(
                "{} #{}",
                sample_titles[e_idx % sample_titles.len()],
                entry_counter
            );
            entries.push(NoteEntry {
                id: format!("entry-{}", entry_counter),
                title,
                description: "Short sentence explaining the idiomatic syntax and behavior."
                    .to_string(),
                code_blocks: vec![CodeBlock {
                    language: lang.to_string(),
                    code: "let x = calculate_value(42);\nprintln!(\"{}\", x);".to_string(),
                }],
                raw_markdown: String::new(),
                line_number: 1,
            });
        }

        db.parts.push(NotePart {
            lang: lang.to_string(),
            part: part_idx as u32,
            title: format!("Standard Part {}", part_idx),
            version: None,
            tags: vec![
                "syntax".to_string(),
                "idioms".to_string(),
                "memory".to_string(),
            ],
            entries,
            file_path: std::path::PathBuf::from(format!("notes/{}/part-{}.md", lang, part_idx)),
            modified: std::time::SystemTime::now(),
        });
    }

    assert_eq!(db.total_entries_count(), 2000);

    // Build the index
    let start_build = Instant::now();
    let index = SearchIndex::from_database(&db);
    let build_duration = start_build.elapsed();
    assert_eq!(index.len(), 2000);
    println!(
        "Built search index for 2,000 entries in {:?}",
        build_duration
    );

    // Run 5 realistic benchmark queries
    let test_queries = [
        "goroutines",
        "move semantics",
        "pattern matching",
        "go: channels",
        "async await event loop",
    ];

    for q in test_queries {
        let start = Instant::now();
        let results = index.search(q, None);
        let elapsed = start.elapsed();

        println!(
            "Query '{}' returned {} results in {:?}",
            q,
            results.len(),
            elapsed
        );

        assert!(
            elapsed.as_millis() < 50,
            "Query '{}' took {:?}, exceeding 50ms latency threshold!",
            q,
            elapsed
        );
        assert!(!results.is_empty(), "Query '{}' should return results", q);
    }
}
