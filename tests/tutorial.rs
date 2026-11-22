//! Keeps the tutorial honest. In tutorial/*.md, a ```joss block that is
//! followed at once by a ```text block is a whole session: the lines a user
//! types at the console, one after another. It is given to `joss`, which must
//! print exactly what the text block says: the page, with the user's lines
//! and JOSS's answers in the order they were typed. A ```joss block followed
//! by anything else is a fragment and is not run.
//!
//! Run with JOSS_BLESS=1 to rewrite the text blocks from real runs.

// These start the `joss` program as a process, which WASI cannot do. Under
// WASI the same programs are compared with the native build by
// scripts/wasi-check.sh.
#![cfg(not(target_os = "wasi"))]

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

struct Block {
    lang: String,
    text: String,
    /// Byte range of the text within the file.
    start: usize,
    end: usize,
    /// Whether anything but blank lines stands between the block before and
    /// this one.
    adjoins: bool,
}

fn blocks(src: &str) -> Vec<Block> {
    let mut out = Vec::new();
    let mut pos = 0;
    let mut open: Option<(String, usize)> = None;
    let mut prose_since_last = true;
    for line in src.split_inclusive('\n') {
        let fence = line.trim_end().strip_prefix("```");
        match (&open, fence) {
            (None, Some(lang)) => open = Some((lang.to_string(), pos + line.len())),
            (Some((lang, start)), Some("")) => {
                out.push(Block {
                    lang: lang.clone(),
                    text: src[*start..pos].to_string(),
                    start: *start,
                    end: pos,
                    adjoins: !prose_since_last,
                });
                open = None;
                prose_since_last = false;
            }
            (None, None) if !line.trim().is_empty() => prose_since_last = true,
            _ => {}
        }
        pos += line.len();
    }
    assert!(open.is_none(), "a code block is not closed");
    out
}

/// The page `joss` prints for a session read from standard input.
fn page(session: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_joss"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(session.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "joss did not run:\n{session}");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn every_session_in_the_tutorial_types_what_the_tutorial_says() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tutorial");
    let bless = std::env::var("JOSS_BLESS").is_ok();
    let mut files: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .collect();
    files.sort();
    assert!(bless || files.len() >= 14, "tutorial chapters are missing");
    let (mut checked, mut failed) = (0, 0);
    for path in files {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let mut text = fs::read_to_string(&path).unwrap();
        let bs = blocks(&text);
        let mut edits = Vec::new();
        for (i, b) in bs.iter().enumerate() {
            if b.lang != "joss" {
                continue;
            }
            let Some(want) = bs.get(i + 1).filter(|n| n.lang == "text" && n.adjoins) else {
                continue;
            };
            let got = page(&b.text);
            checked += 1;
            if got != want.text {
                if bless {
                    edits.push((want.start, want.end, got));
                } else {
                    failed += 1;
                    eprintln!(
                        "---- {} ----\n{}--- the tutorial says\n{}--- joss types\n{}",
                        name, b.text, want.text, got
                    );
                }
            }
        }
        if !edits.is_empty() {
            for (start, end, new) in edits.into_iter().rev() {
                text.replace_range(start..end, &new);
            }
            fs::write(&path, text).unwrap();
        }
    }
    assert!(bless || checked >= 100, "only {checked} tutorial sessions were found");
    assert_eq!(failed, 0, "tutorial sessions with the wrong page");
}

/// A text block belongs to the joss block before it only if nothing but
/// blank lines stands between them.
#[test]
fn the_blocks_are_read_as_the_tutorial_says_they_are() {
    let src = "a\n\n```joss\nType 2+2.\n```\n```text\nType 2+2.\n     2+2 =       4\n```\n\nprose\n\n```joss\nSet x=1.\n```\n\nNot run:\n\n```text\nnothing\n```\n";
    let bs = blocks(src);
    assert_eq!(bs.len(), 4);
    assert!(bs[1].adjoins && bs[1].lang == "text");
    assert!(!bs[3].adjoins);
    assert_eq!(page(&bs[0].text), bs[1].text);
}
