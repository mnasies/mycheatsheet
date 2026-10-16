//! Terminal User Interface (TUI) powered by ratatui and crossterm.

pub mod app;
pub mod clipboard;
pub mod event;
pub mod ui;

pub use app::{run_tui, FocusedPanel, TuiApp};
pub use clipboard::{copy_to_clipboard, ClipboardMethod};
