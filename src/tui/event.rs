use crossterm::cursor;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static TERMINAL_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initializes the terminal raw mode, enters alternate screen, and installs panic hook.
pub fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    if !TERMINAL_INITIALIZED.swap(true, Ordering::SeqCst) {
        install_panic_hook();
    }

    Ok(terminal)
}

/// Restores normal terminal mode.
pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;
    TERMINAL_INITIALIZED.store(false, Ordering::SeqCst);
    Ok(())
}

/// Installs a panic hook that restores the terminal before letting the default hook run.
fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
        original_hook(panic_info);
    }));
}

/// Polls for next key press event, strictly filtering for `KeyEventKind::Press`
/// to guarantee seamless behavior on Windows and Linux.
pub fn poll_key_event(timeout: Duration) -> io::Result<Option<KeyEvent>> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(Some(key));
            }
        }
    }
    Ok(None)
}
