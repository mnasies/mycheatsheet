use clap::Parser;
use directories::ProjectDirs;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Default TCP port for the local web server.
pub const DEFAULT_PORT: u16 = 7878;

/// Fast, offline syntax reference for programming languages.
#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "mycheatsheet",
    author,
    version,
    about = "Fast, offline syntax reference for programming languages in terminal and browser",
    after_help = "EXAMPLES:\n    mycheatsheet                 Launch web mode and open in browser\n    mycheatsheet -t              Launch full-screen terminal UI\n    mycheatsheet -l go           Filter initial view to Go\n    mycheatsheet --check         Validate all notes files and exit\n    mycheatsheet \"goroutine\"     Launch web mode with initial search query"
)]
pub struct Cli {
    /// Optional initial search query.
    #[arg(value_name = "QUERY")]
    pub query: Option<String>,

    /// Open full-screen terminal UI instead of web UI.
    #[arg(short = 't', long = "terminal")]
    pub terminal: bool,

    /// Initial language filter (e.g. go, rust, cpp, js).
    #[arg(short = 'l', long = "lang", value_name = "ID")]
    pub lang: Option<String>,

    /// Override notes folder path.
    #[arg(long = "notes", value_name = "DIR")]
    pub notes: Option<PathBuf>,

    /// Web server port (falls back to an open port if busy).
    #[arg(long = "port", default_value_t = DEFAULT_PORT, value_name = "PORT")]
    pub port: u16,

    /// Start the web server without opening the default browser.
    #[arg(long = "no-open")]
    pub no_open: bool,

    /// Validate all notes files and exit.
    #[arg(long = "check")]
    pub check: bool,
}

/// Resolves the notes directory path following the resolution precedence:
/// 1. CLI flag `--notes <dir>`
/// 2. Environment variable `MYCHEATSHEET_NOTES`
/// 3. `notes_dir` field in `config.toml` inside the OS config directory
/// 4. Default data directory via `directories` crate
///
/// Returns `Some(PathBuf)` if a directory is resolved, or `None` if no configured directory exists.
pub fn resolve_notes_dir(cli_override: Option<&Path>) -> Option<PathBuf> {
    // 1. CLI flag override
    if let Some(path) = cli_override {
        return Some(path.to_path_buf());
    }

    // 2. Environment variable
    if let Ok(env_path) = env::var("MYCHEATSHEET_NOTES") {
        let trimmed = env_path.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }

    // 3. OS config directory: config.toml
    if let Some(proj_dirs) = ProjectDirs::from("com", "mnasie", "mycheatsheet") {
        let config_file = proj_dirs.config_dir().join("config.toml");
        if config_file.is_file() {
            if let Ok(content) = fs::read_to_string(&config_file) {
                if let Some(dir) = parse_notes_dir_from_config(&content) {
                    return Some(dir);
                }
            }
        }
    }

    // 4. Default OS data directory
    if let Some(proj_dirs) = ProjectDirs::from("com", "mnasie", "mycheatsheet") {
        let data_notes = proj_dirs.data_dir().join("notes");
        if data_notes.is_dir() {
            return Some(data_notes);
        }
    }

    None
}

/// Minimal line-by-line parser for `notes_dir = "..."` to avoid unnecessary dependencies
/// while config schema remains simple.
fn parse_notes_dir_from_config(toml_content: &str) -> Option<PathBuf> {
    for line in toml_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if let Some((key, val)) = trimmed.split_once('=') {
            if key.trim() == "notes_dir" {
                let clean_val = val.trim().trim_matches('"').trim_matches('\'').trim();
                if !clean_val.is_empty() {
                    return Some(PathBuf::from(clean_val));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_defaults() {
        let cli = Cli::parse_from(["mycheatsheet"]);
        assert!(!cli.terminal);
        assert!(!cli.no_open);
        assert!(!cli.check);
        assert_eq!(cli.port, DEFAULT_PORT);
        assert_eq!(cli.query, None);
        assert_eq!(cli.lang, None);
        assert_eq!(cli.notes, None);
    }

    #[test]
    fn test_cli_flags() {
        let cli = Cli::parse_from([
            "mycheatsheet",
            "--terminal",
            "-l",
            "go",
            "--notes",
            "/tmp/custom_notes",
            "--port",
            "9090",
            "--no-open",
            "slice",
        ]);
        assert!(cli.terminal);
        assert!(cli.no_open);
        assert_eq!(cli.lang.as_deref(), Some("go"));
        assert_eq!(cli.port, 9090);
        assert_eq!(cli.notes, Some(PathBuf::from("/tmp/custom_notes")));
        assert_eq!(cli.query.as_deref(), Some("slice"));
    }

    #[test]
    fn test_parse_notes_dir_from_config() {
        let sample = r#"
            # Configuration file for mycheatsheet
            notes_dir = "/home/user/notes"
        "#;
        assert_eq!(
            parse_notes_dir_from_config(sample),
            Some(PathBuf::from("/home/user/notes"))
        );

        let empty = r#"
            # No notes dir specified
            port = 7878
        "#;
        assert_eq!(parse_notes_dir_from_config(empty), None);
    }

    #[test]
    fn test_resolve_notes_dir_cli_precedence() {
        let cli_path = PathBuf::from("/override/notes");
        let resolved = resolve_notes_dir(Some(&cli_path));
        assert_eq!(resolved, Some(cli_path));
    }
}
