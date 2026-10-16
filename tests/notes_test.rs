use mycheatsheet::notes::{load_notes_dir, validate_notes_dir};
use std::fs;
use std::path::Path;

#[test]
fn test_real_notes_directory_validates_cleanly() {
    let notes_dir = Path::new("notes");
    let summary = validate_notes_dir(notes_dir);
    if !summary.is_ok() {
        for err in &summary.errors {
            eprintln!("Error: {}", err);
        }
    }
    assert!(
        summary.is_ok(),
        "notes directory should pass all protocol validation rules"
    );
    assert_eq!(summary.notes_checked, 80);
    assert_eq!(summary.languages_count, 4);
    assert!(summary.entries_checked >= 150);
}

#[test]
fn test_load_real_notes_database() {
    let notes_dir = Path::new("notes");
    let (db, warnings) = load_notes_dir(notes_dir);

    assert!(warnings.is_empty(), "expected zero warnings on real notes");
    assert_eq!(db.languages.len(), 4);

    let go = db.get_language("go").expect("go should be present");
    assert_eq!(go.name, "Go");
    assert_eq!(go.version.as_deref(), Some("Go 1.22+"));

    let rust = db.get_language("rust").expect("rust should be present");
    assert_eq!(rust.name, "Rust");

    let cpp = db.get_language("cpp").expect("cpp should be present");
    assert_eq!(cpp.name, "C++");

    let js = db.get_language("js").expect("js should be present");
    assert_eq!(js.name, "JavaScript");

    let go_parts = db.get_parts_for_lang("go");
    assert_eq!(go_parts.len(), 20);
    assert_eq!(go_parts[0].part, 1);
    assert_eq!(go_parts[0].title, "Program structure & entry point");
    assert_eq!(go_parts[1].part, 2);
    assert_eq!(go_parts[1].title, "Variables, constants, mutability");
}

#[test]
fn test_validation_detects_malformed_filename() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let lang_dir = temp_dir.path().join("go");
    fs::create_dir_all(&lang_dir).expect("mkdir");

    let bad_file = lang_dir.join("bad_filename.md");
    fs::write(
        &bad_file,
        r#"+++
title = "Bad"
lang = "go"
part = 1
+++

## Entry
Description.

```go
x := 1
```
"#,
    )
    .expect("write");

    let summary = validate_notes_dir(temp_dir.path());
    assert!(!summary.is_ok());
    assert!(summary.errors.iter().any(|e| e
        .message
        .contains("does not match protocol pattern '<number>-<slug>.md'")));
}

#[test]
fn test_validation_detects_mismatched_lang() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let lang_dir = temp_dir.path().join("go");
    fs::create_dir_all(&lang_dir).expect("mkdir");

    let bad_file = lang_dir.join("01-entry.md");
    fs::write(
        &bad_file,
        r#"+++
title = "Mismatch"
lang = "rust"
part = 1
+++

## Entry
Description.

```go
x := 1
```
"#,
    )
    .expect("write");

    let summary = validate_notes_dir(temp_dir.path());
    assert!(!summary.is_ok());
    assert!(summary
        .errors
        .iter()
        .any(|e| e.message.contains("does not match directory name 'go'")));
}

#[test]
fn test_validation_detects_missing_code_blocks() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let lang_dir = temp_dir.path().join("go");
    fs::create_dir_all(&lang_dir).expect("mkdir");

    let bad_file = lang_dir.join("01-entry.md");
    fs::write(
        &bad_file,
        r#"+++
title = "No code"
lang = "go"
part = 1
+++

## Heading without code
Just some text and no code block.
"#,
    )
    .expect("write");

    let summary = validate_notes_dir(temp_dir.path());
    assert!(!summary.is_ok());
    assert!(summary
        .errors
        .iter()
        .any(|e| e.message.contains("has no code blocks")));
}

#[test]
fn test_loader_fault_tolerance_skips_malformed_file() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let lang_dir = temp_dir.path().join("python");
    fs::create_dir_all(&lang_dir).expect("mkdir");

    // Good file
    let good_file = lang_dir.join("01-basics.md");
    fs::write(
        &good_file,
        r#"+++
title = "Python Basics"
lang = "python"
part = 1
+++

## Hello World
Print hello.

```python
print("Hello")
```
"#,
    )
    .expect("write");

    // Corrupt file
    let bad_file = lang_dir.join("02-corrupt.md");
    fs::write(&bad_file, "this is not valid front matter at all").expect("write");

    let (db, warnings) = load_notes_dir(temp_dir.path());

    // Fault tolerance: the corrupt file generates a warning, but doesn't crash the loader!
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("Failed to parse"));

    // The good note and the dynamically inferred language are loaded
    assert_eq!(db.languages.len(), 1);
    assert_eq!(db.languages[0].id, "python");
    assert_eq!(db.languages[0].name, "Python");
    assert_eq!(db.parts.len(), 1);
    assert_eq!(db.parts[0].title, "Python Basics");
}

#[test]
fn test_integration_add_notes_folder_and_load() {
    let temp_source = tempfile::tempdir().expect("tempdir");
    let zig_src = temp_source.path().join("zig");
    fs::create_dir_all(&zig_src).expect("mkdir");

    fs::write(
        zig_src.join("01-syntax.md"),
        r#"+++
title = "Zig Syntax"
lang = "zig"
part = 1
tags = ["zig", "syntax"]
+++

## Hello Zig
Basic program entry point.

```zig
const std = @import("std");

pub fn main() void {
    std.debug.print("Hello, Zig!\n", .{});
}
```
"#,
    )
    .expect("write");

    let temp_store = tempfile::tempdir().expect("tempdir");
    let notes_store = temp_store.path().join("notes");

    // Add zig notes to notes store
    let summary = mycheatsheet::notes::add_notes_folder(&zig_src, None, &notes_store).expect("add");
    assert_eq!(summary.notes_added, vec!["zig"]);
    assert_eq!(summary.total_files_copied, 1);

    // Verify loading from store
    let (db, warnings) = load_notes_dir(&notes_store);
    assert!(warnings.is_empty());
    assert_eq!(db.languages.len(), 1);
    assert_eq!(db.languages[0].id, "zig");

    // Verify duplicate rejection (case-insensitive "Zig")
    let duplicate_err =
        mycheatsheet::notes::add_notes_folder(&zig_src, Some("Zig"), &notes_store).unwrap_err();
    assert!(duplicate_err.contains("already exists"));
    assert!(duplicate_err.contains("No two notes can have the same name"));
}
