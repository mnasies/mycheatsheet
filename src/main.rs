use clap::Parser;
use mycheatsheet::config::{self, Cli};
use mycheatsheet::launcher;
use mycheatsheet::notes::{load_notes_dir, validate_notes_dir};
use mycheatsheet::search::SearchIndex;
use mycheatsheet::web::server::WebServer;
use std::process::ExitCode;
use std::sync::Arc;

fn main() -> ExitCode {
    let mut cli = Cli::parse();

    // Handle "add" subcommand or --add flag
    if let Some(config::Commands::Add { path, name }) = cli.command {
        let dest = config::default_notes_dir();
        match mycheatsheet::notes::add_notes_folder(&path, name.as_deref(), &dest) {
            Ok(summary) => {
                println!(
                    "Successfully added note(s) to {}:",
                    summary.target_dir.display()
                );
                for n in &summary.notes_added {
                    println!("  - {}", n);
                }
                println!(
                    "\nTotal files copied: {}. You can now run 'mycheatsheet <NOTE>' to open a note.",
                    summary.total_files_copied
                );
                return ExitCode::SUCCESS;
            }
            Err(err) => {
                eprintln!("Error adding notes: {}", err);
                return ExitCode::from(1);
            }
        }
    }

    if let Some(path) = cli.add {
        let dest = config::default_notes_dir();
        match mycheatsheet::notes::add_notes_folder(&path, None, &dest) {
            Ok(summary) => {
                println!(
                    "Successfully added note(s) to {}:",
                    summary.target_dir.display()
                );
                for n in &summary.notes_added {
                    println!("  - {}", n);
                }
                println!(
                    "\nTotal files copied: {}. You can now run 'mycheatsheet <NOTE>' to open a note.",
                    summary.total_files_copied
                );
                return ExitCode::SUCCESS;
            }
            Err(err) => {
                eprintln!("Error adding notes: {}", err);
                return ExitCode::from(1);
            }
        }
    }

    // Notes directory resolution
    let notes_dir = match config::resolve_notes_dir(cli.notes.as_deref()) {
        Some(dir) => dir,
        None => {
            eprintln!("Error: No notes directory could be found.");
            eprintln!();
            eprintln!("To set up your notes directory, you can:");
            eprintln!("  1. Add a notes folder:       mycheatsheet add /path/to/notes");
            eprintln!("  2. Pass the path directly:   mycheatsheet --notes /path/to/notes");
            eprintln!("  3. Set the environment var:  export MYCHEATSHEET_NOTES=/path/to/notes");
            eprintln!("  4. Add `notes_dir = \"/path/to/notes\"` in your mycheatsheet config.toml");
            return ExitCode::from(1);
        }
    };

    // Protocol validation mode: mycheatsheet --check
    if cli.check {
        println!("Validating notes in: {}", notes_dir.display());
        let summary = validate_notes_dir(&notes_dir);

        if !summary.is_ok() {
            eprintln!();
            for err in &summary.errors {
                eprintln!("{}", err);
            }
            eprintln!(
                "\nValidation failed: {} error(s) found across {} note(s).",
                summary.errors.len(),
                summary.notes_checked
            );
            return ExitCode::from(1);
        }

        println!(
            "Success: All {} note(s) across {} language(s) passed validation ({} entries).",
            summary.notes_checked, summary.languages_count, summary.entries_checked
        );
        return ExitCode::SUCCESS;
    }

    // Load notes database for interactive modes (with fault tolerance)
    let (db, warnings) = load_notes_dir(&notes_dir);
    if !warnings.is_empty() {
        eprintln!(
            "Notice: {} note(s) encountered warnings or failed to parse:",
            warnings.len()
        );
        for w in &warnings {
            eprintln!("  {}", w);
        }
        eprintln!();
    }

    if db.languages.is_empty() {
        eprintln!("No notes found in: {}", notes_dir.display());
        eprintln!();
        eprintln!("To add notes to your default collection, run:");
        eprintln!("  mycheatsheet add /path/to/notes");
        return ExitCode::from(1);
    }

    // Check if initial query matches an installed language/note name (e.g. "mycheatsheet Rust")
    let mut direct_note_name: Option<String> = None;
    if cli.lang.is_none() {
        if let Some(ref q) = cli.query {
            let q_trimmed = q.trim();
            if let Some(matched) = db.languages.iter().find(|l| {
                l.id.eq_ignore_ascii_case(q_trimmed) || l.name.eq_ignore_ascii_case(q_trimmed)
            }) {
                direct_note_name = Some(matched.name.clone());
                cli.lang = Some(matched.id.clone());
                cli.query = None;
            }
        }
    }

    // Initialize search index shared across frontends
    let index = SearchIndex::from_database(&db);

    // Initial search query preview if supplied via CLI
    if let Some(ref q) = cli.query {
        let results = index.search(q, cli.lang.as_deref());
        println!("Initial query: \"{}\" ({} match(es))", q, results.len());
        for (i, r) in results.iter().take(3).enumerate() {
            println!(
                "  [{}] {} (Part {}) > {}",
                i + 1,
                r.lang_name,
                r.part,
                r.entry_title
            );
        }
        if results.len() > 3 {
            println!("  ... and {} more results.", results.len() - 3);
        }
        println!();
    }

    // Interactive Terminal UI mode: mycheatsheet -t / --terminal
    if cli.terminal {
        if let Err(err) =
            mycheatsheet::tui::run_tui(&notes_dir, cli.query.as_deref(), cli.lang.as_deref())
        {
            eprintln!("Error running terminal UI: {}", err);
            return ExitCode::from(1);
        }
        return ExitCode::SUCCESS;
    }

    // Default Web Mode: Single instance reuse check
    if let Some(existing_url) = launcher::check_existing_instance(cli.port) {
        let target_url = if let Some(ref lang) = cli.lang {
            format!("{}/lang/{}", existing_url, lang)
        } else {
            existing_url
        };
        println!("Existing mycheatsheet instance detected at {}.", target_url);
        if !cli.no_open {
            if let Some(ref note_name) = direct_note_name {
                println!(
                    "Opening note '{}' in existing browser session...",
                    note_name
                );
            } else {
                println!("Opening existing session in browser...");
            }
            let _ = launcher::open_browser(&target_url);
        }
        return ExitCode::SUCCESS;
    }

    // Start local web server
    let db_arc = Arc::new(db);
    let index_arc = Arc::new(index);

    let server = match WebServer::bind(db_arc.clone(), index_arc, cli.port, None) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error starting web server: {}", err);
            return ExitCode::from(1);
        }
    };

    let mut server_url = server.url();
    if let Some(ref lang) = cli.lang {
        server_url = format!("{}/lang/{}", server_url, lang);
    }

    if let Some(ref note_name) = direct_note_name {
        println!("Opening note '{}': {}", note_name, server_url);
    } else {
        println!(
            "Loaded {} language(s) with {} entries.",
            db_arc.languages.len(),
            db_arc.total_entries_count()
        );
        println!("mycheatsheet server running at: {}", server_url);
    }
    println!("Serving Editorial Edition (auto-stops after 15 min idle). Press Ctrl+C to quit.");

    if !cli.no_open {
        let _ = launcher::open_browser(&server_url);
    }

    if let Err(err) = server.run() {
        eprintln!("Web server error: {}", err);
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}
