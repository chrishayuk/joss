//! Replays a transcript of an original JOSS session and compares.
//!
//! The files in `transcripts/` are sessions printed in RAND's papers, typed
//! in from the scans. Each line says who typed it:
//!
//! ```text
//! U: Type 2+2.          typed by the user
//! J:      2+2 =      4  typed by JOSS, column for column
//! J~ 2+2 = 4            typed by JOSS, known only from a retyping that closed
//!                       up the columns: compared with the spaces left out
//! D: x = 3              a line JOSS began (`x = `) and the user finished
//! D~ x = 3              the same, retyped
//! J?                    a blank line JOSS must have typed where the figure is
//!                       cut off and cannot show it
//! P:                    the paper moved to a new page (not compared)
//! ! interrupt after 16 lines     the INTERRUPT key, pressed after JOSS has
//!                                typed that many lines of its next answer
//! ! joss 1964           which JOSS: the wording of March 1964, or of 1965
//! ! users 6             what `Type users.` is to answer (and `! time 1230`)
//! O: text               typed by JOSS for the operators, not in answer to
//!                       the user: not replayed
//! ~: text               what this reconstruction types instead of the J:
//!                       line above. A known difference, kept in view.
//! # ...                 a note
//! ```
//!
//! `replay` gives the user's lines to a new session and checks every line
//! JOSS types against the page.

use crate::session::{Out, Session, Waiting, Year};

#[derive(Clone, Debug, PartialEq)]
pub struct Difference {
    /// Line of the transcript file, from 1.
    pub line: usize,
    pub original: String,
    pub typed: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    /// Lines typed by the user.
    pub given: usize,
    /// Lines JOSS typed that match the page column for column.
    pub exact: usize,
    /// Lines that match a retyped page, spaces apart.
    pub respaced: usize,
    /// Blank lines at the cut edge of a figure, which the page cannot show.
    pub unseen: usize,
    /// Lines that differ from the page in a way the transcript records.
    pub known: Vec<Difference>,
    /// Lines that differ and should not.
    pub wrong: Vec<Difference>,
}

impl Report {
    pub fn agrees(&self) -> bool {
        self.wrong.is_empty()
    }
}

fn squeezed(text: &str) -> String {
    text.chars().filter(|c| *c != ' ').collect()
}

struct Expected {
    line: usize,
    text: String,
    /// Compare with the spaces left out.
    loose: bool,
    /// What the reconstruction is known to type instead.
    instead: Option<String>,
    /// Off the edge of the figure.
    unseen: bool,
}

pub fn replay(transcript: &str) -> Report {
    let mut report = Report::default();
    let mut session = Session::new();
    let mut typed: Vec<String> = Vec::new();
    let mut asked: Option<String> = None;
    let mut expected: Vec<Expected> = Vec::new();

    fn settle(report: &mut Report, expected: &mut Vec<Expected>, typed: &mut Vec<String>, at: usize) {
        // A retyping cannot be trusted for blank lines any more than for
        // blank columns: where the whole answer is retyped, leave them out.
        let loose = !expected.is_empty() && expected.iter().all(|e| e.loose);
        if loose {
            expected.retain(|e| !e.text.trim().is_empty());
            typed.retain(|t| !t.trim().is_empty());
        }
        let mut got = std::mem::take(typed).into_iter();
        for want in expected.drain(..) {
            let line = got.next();
            let Some(line) = line else {
                report.wrong.push(Difference {
                    line: want.line,
                    original: want.text,
                    typed: "(nothing)".into(),
                });
                continue;
            };
            let same = |a: &str, b: &str| {
                if want.loose {
                    squeezed(a) == squeezed(b)
                } else {
                    a.trim_end() == b.trim_end()
                }
            };
            match &want.instead {
                Some(instead) if same(instead, &line) => report.known.push(Difference {
                    line: want.line,
                    original: want.text,
                    typed: line,
                }),
                None if same(&want.text, &line) => {
                    if want.unseen {
                        report.unseen += 1
                    } else if want.loose {
                        report.respaced += 1
                    } else {
                        report.exact += 1
                    }
                }
                _ => report.wrong.push(Difference {
                    line: want.line,
                    original: want.text,
                    typed: line,
                }),
            }
        }
        for extra in got {
            report.wrong.push(Difference {
                line: at,
                original: "(nothing)".into(),
                typed: extra,
            });
        }
    }

    let feed = |session: &mut Session, typed: &mut Vec<String>, asked: &mut Option<String>, line: &str| {
        for out in session.feed(line) {
            match out {
                Out::Line(text) => typed.push(text),
                Out::Ask(text) => *asked = Some(text),
                Out::Page => {}
            }
        }
    };

    for (index, raw) in transcript.lines().enumerate() {
        let number = index + 1;
        let (mark, text) = match raw.char_indices().nth(2) {
            Some((at, _)) => (&raw[..at], raw[at..].strip_prefix(' ').unwrap_or(&raw[at..])),
            None => (raw, ""),
        };
        if raw.starts_with('#') || raw.trim().is_empty() || mark == "P:" || mark == "O:" {
            continue;
        }
        if let Some(directive) = raw.strip_prefix('!') {
            let words: Vec<&str> = directive.split_whitespace().collect();
            match words.as_slice() {
                ["interrupt", "after", n, ..] => session.interrupt_after = n.parse().ok(),
                ["users", n] => session.users = n.parse().unwrap_or(1),
                ["time", n] => session.time = n.parse().unwrap_or(0),
                ["joss", "1964"] => session.year = Year::Y1964,
                ["joss", "1965"] => session.year = Year::Y1965,
                _ => report.wrong.push(Difference {
                    line: number,
                    original: raw.into(),
                    typed: "(not understood)".into(),
                }),
            }
            continue;
        }
        match mark {
            "U:" => {
                settle(&mut report, &mut expected, &mut typed, number);
                report.given += 1;
                feed(&mut session, &mut typed, &mut asked, text);
            }
            "D:" | "D~" => {
                settle(&mut report, &mut expected, &mut typed, number);
                let loose = mark == "D~";
                let prompt = asked.take().unwrap_or_default();
                let (began, answer) = match text.split_once('=') {
                    Some((label, answer)) => (format!("{label}="), answer.trim()),
                    None => (text.to_string(), ""),
                };
                let matches = if loose {
                    squeezed(&began) == squeezed(&prompt)
                } else {
                    began.trim_end() == prompt.trim_end()
                };
                if session.waiting() != Waiting::Value || !matches {
                    report.wrong.push(Difference {
                        line: number,
                        original: began,
                        typed: prompt,
                    });
                } else if loose {
                    report.respaced += 1;
                } else {
                    report.exact += 1;
                }
                if !answer.is_empty() && session.waiting() == Waiting::Value {
                    report.given += 1;
                    feed(&mut session, &mut typed, &mut asked, answer);
                }
            }
            "J:" | "J~" | "J?" => expected.push(Expected {
                line: number,
                text: text.to_string(),
                loose: mark == "J~",
                instead: None,
                unseen: mark == "J?",
            }),
            "~:" => {
                if let Some(last) = expected.last_mut() {
                    last.instead = Some(text.to_string());
                }
            }
            _ => report.wrong.push(Difference {
                line: number,
                original: raw.into(),
                typed: "(not understood)".into(),
            }),
        }
    }
    let end = transcript.lines().count();
    settle(&mut report, &mut expected, &mut typed, end);
    report
}
