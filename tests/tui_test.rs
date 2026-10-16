use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mycheatsheet::tui::{FocusedPanel, TuiApp};
use std::path::Path;

#[test]
fn test_tui_app_initialization() {
    let notes_dir = Path::new("notes");
    let app = TuiApp::new(notes_dir, None, None);

    assert!(!app.db.languages.is_empty());
    assert!(!app.results.is_empty());
    assert_eq!(app.selected_index, 0);
    assert_eq!(app.active_lang, None);
    assert!(!app.show_help);
    assert!(!app.should_quit);
}

#[test]
fn test_tui_app_query_typing_and_backspace() {
    let notes_dir = Path::new("notes");
    let mut app = TuiApp::new(notes_dir, None, None);
    app.focused_panel = FocusedPanel::Search;

    // Type "init"
    for c in "init".chars() {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    assert_eq!(app.query, "init");
    assert!(!app.results.is_empty());
    assert_eq!(app.results[0].entry_id, "package-initialization-with-init");

    // Backspace twice -> "in"
    app.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(app.query, "in");
}

#[test]
fn test_tui_app_navigation_and_focus() {
    let notes_dir = Path::new("notes");
    let mut app = TuiApp::new(notes_dir, None, None);
    app.focused_panel = FocusedPanel::Results;

    let initial_idx = app.selected_index;
    assert_eq!(initial_idx, 0);

    // Press Down
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.selected_index, 1);

    // Press Up
    app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(app.selected_index, 0);

    // Press Enter to focus Note
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(app.focused_panel, FocusedPanel::Note);

    // Press Esc to return to Results
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(app.focused_panel, FocusedPanel::Results);
}

#[test]
fn test_tui_app_cycle_language() {
    let notes_dir = Path::new("notes");
    let mut app = TuiApp::new(notes_dir, None, None);
    app.focused_panel = FocusedPanel::Results;

    assert_eq!(app.active_lang, None);

    // Tab -> First language (Go)
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.active_lang.as_deref(), Some("go"));
    for r in &app.results {
        assert_eq!(r.lang, "go");
    }

    // Tab -> Second language (Rust)
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.active_lang.as_deref(), Some("rust"));
    for r in &app.results {
        assert_eq!(r.lang, "rust");
    }
}

#[test]
fn test_tui_app_help_modal_and_quit() {
    let notes_dir = Path::new("notes");
    let mut app = TuiApp::new(notes_dir, None, None);
    app.focused_panel = FocusedPanel::Results;

    // Press '?' to open help modal
    app.handle_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
    assert!(app.show_help);

    // Press Esc to close help modal
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(!app.show_help);

    // Press Ctrl-C to quit
    app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
    assert!(app.should_quit);
}

#[test]
fn test_tui_app_reload_and_copy() {
    let notes_dir = Path::new("notes");
    let mut app = TuiApp::new(notes_dir, None, None);
    app.focused_panel = FocusedPanel::Results;

    // Press 'r' to reload notes
    app.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    assert!(app.status_message.is_some());
    let (msg, _) = app.status_message.as_ref().unwrap();
    assert!(msg.to_lowercase().contains("reloaded"));

    // Press 'y' to copy active snippet
    app.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    let (copy_msg, _) = app.status_message.as_ref().unwrap();
    assert!(copy_msg.contains("Copied") || copy_msg.contains("Copy"));
}
