//! `joss`: a JOSS console.
//!
//!     joss                     sit at the console: type a line, JOSS answers
//!     joss session.joss        replay the lines in a file and print the page
//!     joss --marked file       the same, with `U: ` and `J: ` in front of
//!                              each line to show who typed it
//!     joss --quiet file        print only what JOSS typed
//!     joss -i file             type the lines in a file, then hand the
//!                              typewriter over: carry on at the console
//!     joss --check file        read a file without running it; report lines
//!                              that are not JOSS
//!     joss --verify file...    replay transcripts of original sessions (see
//!                              transcripts/) and compare with the page
//!     joss --year 1964         the JOSS of the 1964 film: its wording and
//!                              its columns. The default is 1965.
//!     joss --patience N        press the interrupt key after N steps
//!     joss --status            end with a line `S: instruction|form|value
//!                              suspended|none N`: what JOSS is waiting for,
//!                              whether a task is waiting for Go, and the
//!                              line of the page (used by the web page)
//!
//! In a file, a line `! interrupt after N lines` is not typed: it is the
//! INTERRUPT key, pressed once JOSS has typed N lines of its next answer.
//!
//! With no file name and input that is not a terminal, the lines are read
//! from standard input, so `echo 'Type 2+2.' | joss` works.

use joss::session::{Out, Session, Waiting, Year};
use std::io::{self, BufRead, IsTerminal, Write};
use std::process::ExitCode;

#[derive(Clone, Copy, PartialEq)]
enum Show {
    /// The page as it would look: what was typed, and what JOSS typed.
    Page,
    /// The page with a mark in front of each line: `U: ` or `J: `.
    Marked,
    /// Only what JOSS typed.
    Quiet,
}

const USAGE: &str = "usage: joss [--marked | --quiet] [--colour] [--year 1964|1965] [--patience N] [-i] [file]\n       joss --check file...\n       joss --verify transcript...";

fn main() -> ExitCode {
    let mut show = Show::Page;
    let mut colour = false;
    let mut check = false;
    let mut verify = false;
    let mut status = false;
    let mut then_console = false;
    let mut year = Year::Y1965;
    let mut patience = None;
    let mut files = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--marked" => show = Show::Marked,
            "--quiet" => show = Show::Quiet,
            "--colour" | "--color" => colour = true,
            "--check" => check = true,
            "--verify" => verify = true,
            "--status" => status = true,
            "-i" | "--interactive" => then_console = true,
            "--year" => match args.next().as_deref() {
                Some("1964") => year = Year::Y1964,
                Some("1965") => year = Year::Y1965,
                _ => {
                    eprintln!("{USAGE}");
                    return ExitCode::from(2);
                }
            },
            "--patience" => match args.next().and_then(|n| n.parse().ok()) {
                Some(n) => patience = Some(n),
                None => {
                    eprintln!("{USAGE}");
                    return ExitCode::from(2);
                }
            },
            "--help" | "-h" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            "--version" => {
                println!("joss {}", env!("CARGO_PKG_VERSION"));
                return ExitCode::SUCCESS;
            }
            _ if arg.starts_with("--") => {
                eprintln!("joss: unknown option {arg}\n{USAGE}");
                return ExitCode::from(2);
            }
            _ => files.push(arg),
        }
    }

    if check {
        return joss::check::command(&files);
    }
    if verify {
        return joss::check::verify(&files);
    }

    let stdin = io::stdin();
    // Lines from a file are echoed onto the page; lines typed at a terminal
    // are already there.
    let at_terminal = stdin.is_terminal();
    let lines: Box<dyn Iterator<Item = (io::Result<String>, bool)>> = match files.first() {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => {
                let from_file = text
                    .lines()
                    .map(|l| (Ok(l.to_string()), false))
                    .collect::<Vec<_>>()
                    .into_iter();
                if then_console {
                    Box::new(from_file.chain(stdin.lock().lines().map(move |l| (l, at_terminal))))
                } else {
                    Box::new(from_file)
                }
            }
            Err(error) => {
                eprintln!("joss: {path}: {error}");
                return ExitCode::from(2);
            }
        },
        None => Box::new(stdin.lock().lines().map(move |l| (l, at_terminal))),
    };
    let mut interactive = false;

    let mut session = Session::new();
    session.patience = patience;
    session.year = year;
    let mut out = io::stdout().lock();
    // JOSS typed in black and the user in green.
    let (green, plain) = if colour { ("\x1b[32m", "\x1b[0m") } else { ("", "") };
    for (line, typed_here) in lines {
        interactive = typed_here;
        let Ok(line) = line else { break };
        // `! interrupt after 13 lines`: not something typed, but a key
        // pressed. The INTERRUPT button is pressed once JOSS has typed that
        // many lines in answer to the next line.
        if let Some(directive) = line.strip_prefix('!') {
            let mut words = directive.split_whitespace();
            let known = match (words.next(), words.next(), words.next()) {
                (Some("interrupt"), Some("after"), Some(n)) => {
                    n.parse().map(|n| session.interrupt_after = Some(n)).is_ok()
                }
                (Some("joss"), Some("1964"), None) => {
                    session.year = Year::Y1964;
                    true
                }
                (Some("joss"), Some("1965"), None) => {
                    session.year = Year::Y1965;
                    true
                }
                (Some("users"), Some(n), None) => n.parse().map(|n| session.users = n).is_ok(),
                (Some("time"), Some(n), None) => n.parse().map(|n| session.time = n).is_ok(),
                _ => false,
            };
            if known {
                if show == Show::Marked {
                    let _ = writeln!(out, "{line}");
                }
                continue;
            }
        }
        let answering = session.waiting() == Waiting::Value;
        // The page shows the typewriter's own characters: `·` where `@` was
        // typed. The line of a form is a picture and is left as it is.
        let line = joss::chars::canonical(&line, session.waiting() == Waiting::Form);
        if !interactive {
            let _ = match show {
                Show::Page => writeln!(out, "{green}{line}{plain}"),
                Show::Marked if answering => writeln!(out, "{green}{line}{plain}"),
                Show::Marked => writeln!(out, "U: {green}{line}{plain}"),
                Show::Quiet if answering => writeln!(out),
                Show::Quiet => Ok(()),
            };
        }
        for typed in session.feed(&line) {
            let _ = match (typed, show) {
                (Out::Line(text), Show::Marked) => {
                    writeln!(out, "{}", format!("J: {text}").trim_end())
                }
                (Out::Line(text), _) => writeln!(out, "{text}"),
                (Out::Ask(text), Show::Marked) => write!(out, "D: {text}"),
                (Out::Ask(text), _) => write!(out, "{text}"),
                (Out::Page, Show::Marked) => writeln!(out, "P:"),
                // A new sheet. On a terminal there is only the one.
                (Out::Page, _) => Ok(()),
            };
        }
        let _ = out.flush();
    }
    if session.waiting() == Waiting::Value && !interactive {
        // The lines ran out while JOSS was waiting for a value.
        let _ = writeln!(out);
    }
    if status {
        // For a page that wants to know whose turn it is and for what.
        let waiting = match session.waiting() {
            Waiting::Instruction => "instruction",
            Waiting::Form => "form",
            Waiting::Value => "value",
        };
        let task = if session.suspended() { "suspended" } else { "none" };
        let _ = writeln!(out, "S: {waiting} {task} {}", session.line_number());
    }
    ExitCode::SUCCESS
}
