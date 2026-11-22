//! Every session in examples/ is replayed and the page compared with
//! examples/<name>.out, which is what `joss examples/<name>.joss` prints: the
//! lines the user types and the lines JOSS types back. Then `joss --check`
//! reads every one of them without running it.
//!
//! Run with JOSS_BLESS=1 to rewrite the .out files from real runs.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn examples() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let mut sessions: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "joss"))
        .collect();
    sessions.sort();
    sessions
}

fn name(path: &Path) -> String {
    path.file_stem().unwrap().to_string_lossy().into_owned()
}

#[test]
fn every_example_types_the_page_in_its_out_file() {
    let bless = std::env::var("JOSS_BLESS").is_ok();
    let sessions = examples();
    assert!(sessions.len() >= 50, "only {} examples were found", sessions.len());
    let mut failures = Vec::new();
    for path in &sessions {
        let name = name(path);
        let output = Command::new(env!("CARGO_BIN_EXE_joss")).arg(path).output().unwrap();
        assert!(output.status.success(), "{name}: joss did not run");
        let page = String::from_utf8_lossy(&output.stdout).into_owned();
        let out = path.with_extension("out");
        if bless {
            fs::write(&out, &page).unwrap();
            continue;
        }
        let expected = fs::read_to_string(&out).unwrap_or_else(|_| panic!("{name}.out is missing"));
        if page != expected {
            eprintln!("---- {name} ----\n--- {name}.out says\n{expected}--- joss types\n{page}");
            failures.push(name);
        }
    }
    assert!(failures.is_empty(), "examples whose page differs: {failures:?}");
}

/// The sessions from the papers of 1964 and 1965 are all here, under names
/// that say where they come from.
#[test]
fn the_programs_of_the_papers_are_among_them() {
    let names: Vec<String> = examples().iter().map(|path| name(path)).collect();
    for wanted in [
        "1964-film-brightness",
        "1964-film-rlc-circuit",
        "1964-film-table",
        "1965-atom",
        "1965-gauss-integration",
        "1965-hypotenuse",
        "1965-matrix-inversion",
        "1965-primes",
        "1965-quadratic-with-mistakes",
        "1965-root-finding",
        "1965-table",
    ] {
        assert!(names.iter().any(|name| name == wanted), "{wanted}.joss is missing");
    }
    for path in examples().iter().filter(|path| name(path).starts_with("19")) {
        let text = fs::read_to_string(path).unwrap();
        let first = text.lines().next().unwrap_or("");
        assert!(
            first.starts_with("* From ") && first.contains("RAND"),
            "{}: the first line should say which paper it is from",
            name(path)
        );
    }
}

/// Sessions in which the mistakes are the point: two of Shaw's own pages. In
/// figure 2 of P-3149 he makes one mistake after another on purpose, and on
/// page 2 of P-3131 he types `Type 2+2+.` to show "Eh?". Only these may have
/// errors or warnings in them.
const MISTAKES_ON_PURPOSE: &[&str] = &["1965-elements", "1965-quadratic-with-mistakes"];

/// `joss --check` reads a session without running it. No example may have an
/// error or a warning in it unless the mistake is what the example is for.
///
/// A line beginning `!` is not typed at the console: it is a key pressed or a
/// setting (`! interrupt after 4 lines`, `! joss 1964`). `joss` obeys these
/// lines but `joss --check` calls them "Eh?", so an error on such a line is
/// passed over here.
#[test]
fn examples_pass_the_static_check() {
    let sessions = examples();
    let output = Command::new(env!("CARGO_BIN_EXE_joss"))
        .arg("--check")
        .args(&sessions)
        .output()
        .unwrap();
    let report = String::from_utf8_lossy(&output.stdout);
    let mut complaints = Vec::new();
    for line in report.lines() {
        let serious = line.contains(": error: ") || line.contains(": warning: ");
        if !serious {
            continue;
        }
        // path:line:column: severity: message
        let Some((path, rest)) = line.split_once(".joss:") else {
            complaints.push(line.to_string());
            continue;
        };
        let path = PathBuf::from(format!("{path}.joss"));
        if MISTAKES_ON_PURPOSE.contains(&name(&path).as_str()) {
            continue;
        }
        let number: usize = rest.split(':').next().and_then(|n| n.parse().ok()).unwrap_or(0);
        let source = fs::read_to_string(&path).unwrap_or_default();
        let key_pressed = source
            .lines()
            .nth(number.saturating_sub(1))
            .is_some_and(|text| text.starts_with('!'));
        if !key_pressed {
            complaints.push(line.to_string());
        }
    }
    assert!(complaints.is_empty(), "{}", complaints.join("\n"));
}
