use crate::notes::loader::load_notes_dir;
use crate::notes::model::{Language, NoteEntry, NotesDatabase};
use crate::search::{SearchIndex, SearchResult};
use crate::tui::clipboard::{copy_to_clipboard, ClipboardMethod};
use crate::tui::event::{init_terminal, poll_key_event, restore_terminal};
use crate::tui::ui::render;
use crossterm::event::{KeyCode, KeyModifiers};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Focused panel in the TUI navigation model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedPanel {
    Search,
    Results,
    Note,
}

/// The state of the running Terminal User Interface.
pub struct TuiApp {
    pub db: NotesDatabase,
    pub index: SearchIndex,
    pub notes_dir: PathBuf,
    pub query: String,
    pub active_lang: Option<String>,
    pub available_langs: Vec<Language>,
    pub results: Vec<SearchResult>,
    pub selected_index: usize,
    pub focused_panel: FocusedPanel,
    pub note_scroll: u16,
    pub status_message: Option<(String, Instant)>,
    pub show_help: bool,
    pub should_quit: bool,
}

impl TuiApp {
    /// Creates a new TUI application state.
    pub fn new(notes_dir: &Path, initial_query: Option<&str>, initial_lang: Option<&str>) -> Self {
        let (db, _) = load_notes_dir(notes_dir);
        let index = SearchIndex::from_database(&db);

        let available_langs = db.languages.clone();
        let active_lang = initial_lang.map(|s| s.to_ascii_lowercase());
        let query = initial_query.unwrap_or("").to_string();

        let results = if query.trim().is_empty() {
            if let Some(ref lang) = active_lang {
                index.search("", Some(lang))
            } else {
                index.all_results()
            }
        } else {
            index.search(&query, active_lang.as_deref())
        };

        let focused_panel = if results.is_empty() {
            FocusedPanel::Search
        } else {
            FocusedPanel::Results
        };

        TuiApp {
            db,
            index,
            notes_dir: notes_dir.to_path_buf(),
            query,
            active_lang,
            available_langs,
            results,
            selected_index: 0,
            focused_panel,
            note_scroll: 0,
            status_message: None,
            show_help: false,
            should_quit: false,
        }
    }

    /// Finds a specific note entry by language, part, and slug.
    pub fn find_entry(&self, lang: &str, part_num: u32, entry_id: &str) -> Option<&NoteEntry> {
        let part = self.db.get_part(lang, part_num)?;
        part.entries.iter().find(|e| e.id == entry_id)
    }

    /// Refreshes search results based on current query and active language filter.
    pub fn update_search(&mut self) {
        self.results = if self.query.trim().is_empty() {
            if let Some(ref lang) = self.active_lang {
                self.index.search("", Some(lang))
            } else {
                self.index.all_results()
            }
        } else {
            self.index.search(&self.query, self.active_lang.as_deref())
        };

        if self.selected_index >= self.results.len() {
            self.selected_index = self.results.len().saturating_sub(1);
        }
        self.note_scroll = 0;
    }

    /// Cycles the active language filter: All -> Lang1 -> Lang2 -> ... -> All.
    pub fn cycle_language(&mut self) {
        if self.available_langs.is_empty() {
            return;
        }

        self.active_lang = match &self.active_lang {
            None => Some(self.available_langs[0].id.clone()),
            Some(curr) => {
                let idx = self
                    .available_langs
                    .iter()
                    .position(|l| l.id.eq_ignore_ascii_case(curr));
                match idx {
                    Some(i) if i + 1 < self.available_langs.len() => {
                        Some(self.available_langs[i + 1].id.clone())
                    }
                    _ => None, // Wraps back to "All"
                }
            }
        };

        let badge = match &self.active_lang {
            Some(l) => format!("Filter: {}", l.to_uppercase()),
            None => "Filter: ALL".to_string(),
        };
        self.set_status(&badge);
        self.update_search();
    }

    /// Copies the first code block of the currently selected entry to clipboard.
    pub fn copy_active_snippet(&mut self) {
        if let Some(res) = self.results.get(self.selected_index) {
            if let Some(entry) = self.find_entry(&res.lang, res.part, &res.entry_id) {
                if let Some(cb) = entry.code_blocks.first() {
                    match copy_to_clipboard(&cb.code) {
                        Ok(ClipboardMethod::SystemClipboard) => {
                            self.set_status("✔ Copied code block to system clipboard");
                        }
                        Ok(ClipboardMethod::Osc52Terminal) => {
                            self.set_status("✔ Copied code block via OSC 52 terminal copy");
                        }
                        Err(err) => {
                            self.set_status(&format!("✖ Copy failed: {}", err));
                        }
                    }
                    return;
                }
            }
        }
        self.set_status("No code block to copy");
    }

    /// Reloads notes from filesystem and refreshes search index without restarting.
    pub fn reload_notes(&mut self) {
        let (db, warnings) = load_notes_dir(&self.notes_dir);
        self.index = SearchIndex::from_database(&db);
        self.available_langs = db.languages.clone();
        self.db = db;
        self.update_search();

        if warnings.is_empty() {
            self.set_status("✔ Notes reloaded cleanly from disk");
        } else {
            self.set_status(&format!(
                "⚠ Notes reloaded with {} warning(s)",
                warnings.len()
            ));
        }
    }

    /// Sets a temporary status message that auto-clears after 3 seconds.
    pub fn set_status(&mut self, msg: &str) {
        self.status_message = Some((msg.to_string(), Instant::now()));
    }

    /// Checks and clears expired status messages.
    pub fn check_status_expiration(&mut self) {
        if let Some((_, created_at)) = self.status_message {
            if created_at.elapsed() > Duration::from_secs(3) {
                self.status_message = None;
            }
        }
    }

    /// Handles keyboard events according to active panel.
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        // Global quit shortcuts
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        // Help modal toggle
        if self.show_help {
            if key.code == KeyCode::Esc
                || key.code == KeyCode::Char('?')
                || key.code == KeyCode::Char('q')
            {
                self.show_help = false;
            }
            return;
        }

        if key.code == KeyCode::Char('?') && self.focused_panel != FocusedPanel::Search {
            self.show_help = true;
            return;
        }

        // Global shortcuts available outside text typing
        if self.focused_panel != FocusedPanel::Search {
            match key.code {
                KeyCode::Char('/') => {
                    self.focused_panel = FocusedPanel::Search;
                    return;
                }
                KeyCode::Tab => {
                    self.cycle_language();
                    return;
                }
                KeyCode::Char('y') => {
                    self.copy_active_snippet();
                    return;
                }
                KeyCode::Char('r') => {
                    self.reload_notes();
                    return;
                }
                KeyCode::Char('q') => {
                    self.should_quit = true;
                    return;
                }
                _ => {}
            }
        }

        match self.focused_panel {
            FocusedPanel::Search => match key.code {
                KeyCode::Char(c) => {
                    self.query.push(c);
                    self.selected_index = 0;
                    self.update_search();
                }
                KeyCode::Backspace => {
                    self.query.pop();
                    self.selected_index = 0;
                    self.update_search();
                }
                KeyCode::Esc => {
                    if !self.query.is_empty() {
                        self.query.clear();
                        self.update_search();
                    } else {
                        self.focused_panel = FocusedPanel::Results;
                    }
                }
                KeyCode::Enter | KeyCode::Down => {
                    if !self.results.is_empty() {
                        self.focused_panel = FocusedPanel::Results;
                    }
                }
                KeyCode::Tab => {
                    self.cycle_language();
                }
                _ => {}
            },

            FocusedPanel::Results => match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                        self.note_scroll = 0;
                    } else {
                        self.focused_panel = FocusedPanel::Search;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !self.results.is_empty() && self.selected_index + 1 < self.results.len() {
                        self.selected_index += 1;
                        self.note_scroll = 0;
                    }
                }
                KeyCode::Enter => {
                    if !self.results.is_empty() {
                        self.focused_panel = FocusedPanel::Note;
                    }
                }
                KeyCode::Esc => {
                    self.focused_panel = FocusedPanel::Search;
                }
                _ => {}
            },

            FocusedPanel::Note => match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.note_scroll = self.note_scroll.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.note_scroll = self.note_scroll.saturating_add(1);
                }
                KeyCode::PageUp => {
                    self.note_scroll = self.note_scroll.saturating_sub(10);
                }
                KeyCode::PageDown => {
                    self.note_scroll = self.note_scroll.saturating_add(10);
                }
                KeyCode::Esc => {
                    self.focused_panel = FocusedPanel::Results;
                }
                _ => {}
            },
        }
    }
}

/// Runs the interactive terminal UI event loop.
pub fn run_tui(
    notes_dir: &Path,
    initial_query: Option<&str>,
    initial_lang: Option<&str>,
) -> io::Result<()> {
    let mut terminal = init_terminal()?;
    let mut app = TuiApp::new(notes_dir, initial_query, initial_lang);

    loop {
        app.check_status_expiration();
        terminal.draw(|f| render(f, &app))?;

        if let Some(key) = poll_key_event(Duration::from_millis(100))? {
            app.handle_key(key);
        }

        if app.should_quit {
            break;
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}
