//! Every transcript of an original session is replayed, and every line JOSS
//! types is compared with the page.

use joss::transcript::replay;
use std::fs;

#[test]
fn the_original_sessions_replay() {
    replay_all("transcripts");
}

/// Sessions written for the tests, for what no surviving page shows.
#[test]
fn the_rules_the_sources_give_hold() {
    replay_all("tests/sessions");
}

fn replay_all(folder: &str) {
    let mut names: Vec<_> = fs::read_dir(folder)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "txt"))
        .collect();
    names.sort();
    assert!(!names.is_empty());
    let mut failed = false;
    for path in names {
        let report = replay(&fs::read_to_string(&path).unwrap());
        println!(
            "{}: {} lines given; JOSS typed {} that match column for column, {} that match a retyping, {} off the edge of a figure, {} known differences, {} wrong",
            path.display(), report.given, report.exact, report.respaced, report.unseen, report.known.len(), report.wrong.len()
        );
        for d in &report.wrong {
            failed = true;
            println!(
                "  line {}:\n    page:  {:?}\n    typed: {:?}",
                d.line, d.original, d.typed
            );
        }
    }
    assert!(!failed, "a transcript does not replay");
}
