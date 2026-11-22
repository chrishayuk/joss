//! `joss --check` and `joss --verify`.

use crate::analysis::{analyse, position, Severity};
use crate::transcript::replay;
use std::process::ExitCode;

/// Reads each file without running it and prints what an editor would
/// underline. The exit status is 1 if any file has an error in it.
pub fn command(files: &[String]) -> ExitCode {
    if files.is_empty() {
        eprintln!("joss --check: no file given");
        return ExitCode::from(2);
    }
    let mut errors = 0;
    for path in files {
        let source = match std::fs::read_to_string(path) {
            Ok(source) => source,
            Err(error) => {
                eprintln!("joss: {path}: {error}");
                return ExitCode::from(2);
            }
        };
        for d in analyse(&source).diagnostics {
            let (line, column) = position(&d);
            println!("{path}:{line}:{column}: {}: {}", d.severity.name(), d.message);
            errors += usize::from(d.severity == Severity::Error);
        }
    }
    if errors > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

/// Replays transcripts of original sessions and reports how each compares
/// with the page. The exit status is 1 if any line differs that should not.
pub fn verify(files: &[String]) -> ExitCode {
    if files.is_empty() {
        eprintln!("joss --verify: no transcript given");
        return ExitCode::from(2);
    }
    let mut wrong = 0;
    for path in files {
        // `-` is standard input, which is how a web page hands one over.
        let read = if path == "-" {
            std::io::read_to_string(std::io::stdin())
        } else {
            std::fs::read_to_string(path)
        };
        let text = match read {
            Ok(text) => text,
            Err(error) => {
                eprintln!("joss: {path}: {error}");
                return ExitCode::from(2);
            }
        };
        let report = replay(&text);
        println!("{path}");
        println!("  {:>4} lines typed by the user", report.given);
        println!(
            "  {:>4} lines typed by JOSS match the page column for column",
            report.exact
        );
        if report.respaced > 0 {
            println!("  {:>4} match a page that was retyped, spaces apart", report.respaced);
        }
        if report.unseen > 0 {
            println!("  {:>4} blank lines fall off the edge of a figure", report.unseen);
        }
        for d in &report.known {
            println!(
                "  line {}: a known difference\n      the page: {}\n      here:     {}",
                d.line, d.original, d.typed
            );
        }
        for d in &report.wrong {
            println!(
                "  line {}: DIFFERS\n      the page: {}\n      here:     {}",
                d.line, d.original, d.typed
            );
        }
        wrong += report.wrong.len();
    }
    if wrong > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
