use clap::Parser;
use mycheatsheet::config::{self, Cli};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Notes directory resolution
    let notes_dir = match config::resolve_notes_dir(cli.notes.as_deref()) {
        Some(dir) => dir,
        None => {
            eprintln!("Error: No notes directory could be found.");
            eprintln!();
            eprintln!("To set up your notes directory, you can:");
            eprintln!("  1. Pass the path directly:  mycheatsheet --notes /path/to/notes");
            eprintln!("  2. Set the environment var: export MYCHEATSHEET_NOTES=/path/to/notes");
            eprintln!("  3. Add `notes_dir = \"/path/to/notes\"` in your mycheatsheet config.toml");
            eprintln!(
                "  4. Run the installer script to automatically link this repository's notes/"
            );
            return ExitCode::from(1);
        }
    };

    if cli.check {
        println!("Checking notes directory: {}", notes_dir.display());
        println!("Notes validation logic will run in Stage 2.");
        return ExitCode::SUCCESS;
    }

    if cli.terminal {
        println!(
            "Starting terminal UI with notes from: {}",
            notes_dir.display()
        );
        println!("TUI interface will be launched in Stage 4.");
        return ExitCode::SUCCESS;
    }

    println!(
        "Starting web server on 127.0.0.1:{} with notes from: {}",
        cli.port,
        notes_dir.display()
    );
    if cli.no_open {
        println!("Browser opening disabled via --no-open.");
    } else {
        println!("Opening default browser...");
    }
    println!("Web server and UI will be launched in Stage 5.");

    ExitCode::SUCCESS
}
