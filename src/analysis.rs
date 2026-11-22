//! Reads a file of JOSS without running it.
//!
//! A `.joss` file is what would be typed at the console, a line at a time:
//! steps, forms, direct instructions, and notes (lines that begin or end
//! with `*`). This module works out what each line is, colours its words, and
//! finds what can be found without running anything: lines that are not
//! JOSS, a `Do` of a part with no steps, a form with too few blanks, a
//! letter that is used and never given a value.
//!
//! JOSS itself said only "Eh?" about a line it could not read, and said it
//! when the line was typed or the step was reached. An editor can say where.
//! Everything here is advice from the editor; none of it is JOSS speaking.
//!
//! `joss --check` prints what this finds, and the language server (`lsp/`)
//! is built on it.

use crate::chars::canonical_with_map;
use crate::format::{self, FormFault};
use crate::lex::{self, Tok, CONNECTIVES, FUNCTIONS, NOUNS, VERBS};
use crate::num::Num;
use crate::parse::{self, Eh, Line, Stmt, Verb};
use std::collections::BTreeMap;

/// What a word of the file is, for colouring.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A whole line that JOSS ignores.
    Comment,
    /// The number in front of a step.
    StepNumber,
    Verb,
    Noun,
    Connective,
    Function,
    /// A letter.
    Variable,
    Number,
    /// Words between double quotes, and the fixed words of a form.
    Text,
    /// A blank in a form, to be filled with a number.
    Field,
    Operator,
    Punctuation,
    /// Something that is not JOSS.
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub line: usize,
    /// Columns (characters) in the line as it is in the file.
    pub start: usize,
    pub end: usize,
    /// The token as JOSS holds it (`·` for `@`, and so on).
    pub text: String,
    /// A letter that is being given a value here (`Set x`, `for x`, `Demand x`).
    pub defines: bool,
    /// For a numeral after `step`, `part` or `form`: what it names.
    pub names: Option<Named>,
}

/// A step, a part or a form, named by number.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Named {
    Step(Num),
    Part(i64),
    Form(i64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineKind {
    Empty,
    Comment,
    Step {
        number: Num,
        part: i64,
    },
    Direct,
    FormHead(i64),
    /// The line of form `n`.
    FormBody(i64),
    /// A value typed in answer to a `Demand`.
    Answer,
    /// Not JOSS.
    Bad,
}

#[derive(Clone, Debug)]
pub struct LineInfo {
    pub kind: LineKind,
    /// The instruction, if the line is a direct one or a step that reads.
    pub stmt: Option<Stmt>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error,
    Warning,
    Hint,
}

impl Severity {
    pub fn name(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Hint => "hint",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
    /// The text is dead: a step that a later line replaces.
    pub unnecessary: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Analysis {
    pub lines: Vec<LineInfo>,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
    /// Every line that gives each step, in order. The last one stands.
    pub steps: BTreeMap<Num, Vec<usize>>,
    /// The line of the picture of each form (the line after `Form n:`).
    pub forms: BTreeMap<i64, Vec<usize>>,
}

fn word_kind(word: &str) -> Kind {
    if VERBS.contains(&word) {
        Kind::Verb
    } else if NOUNS.contains(&word) {
        Kind::Noun
    } else if CONNECTIVES.contains(&word) {
        Kind::Connective
    } else if FUNCTIONS.contains(&word) {
        Kind::Function
    } else {
        Kind::Unknown
    }
}

/// Splits a JOSS line into coloured tokens. `map` turns columns of the
/// canonical text back into columns of the file.
fn colour(line: usize, text: &str, map: &[usize], out: &mut Vec<Token>) {
    let chars: Vec<char> = text.chars().collect();
    let lexed = lex::lex(text);
    let mut i = 0;
    while i < lexed.len() {
        let token = &lexed[i];
        let kind = match &token.tok {
            Tok::Number(_) => Kind::Number,
            Tok::Letter(_) => Kind::Variable,
            Tok::Word(word) => word_kind(word),
            Tok::Text(_) => Kind::Text,
            Tok::Blank => Kind::Field,
            Tok::Period | Tok::Comma | Tok::Colon | Tok::Semicolon | Tok::Open(_) | Tok::Close(_) | Tok::Bar => {
                Kind::Punctuation
            }
            Tok::Plus | Tok::Minus | Tok::Times | Tok::Over | Tok::Power => Kind::Operator,
            Tok::Eq | Tok::Ne | Tok::Le | Tok::Ge | Tok::Lt | Tok::Gt => Kind::Operator,
            Tok::Dollar => Kind::Variable,
            Tok::Other(_) => Kind::Unknown,
        };
        let mut names = None;
        if let (Tok::Number(digits), Some(Tok::Word(noun))) = (&token.tok, i.checked_sub(1).map(|p| &lexed[p].tok)) {
            if let Some(n) = Num::numeral(digits) {
                names = match noun.as_str() {
                    "step" => Some(Named::Step(n)),
                    "part" => n.to_i64().map(Named::Part),
                    "form" | "Form" => n.to_i64().map(Named::Form),
                    _ => None,
                };
            }
        }
        out.push(Token {
            kind,
            line,
            start: map[token.start],
            end: map[token.end],
            text: chars[token.start..token.end].iter().collect(),
            defines: false,
            names,
        });
        i += 1;
    }
}

/// Marks the letters that are given a value: after `Set`, `Demand` and
/// `for`, up to the `=` (or the end, for `Demand`), the first letter.
fn mark_definitions(tokens: &mut [Token]) {
    let mut i = 0;
    while i < tokens.len() {
        let opens = tokens[i].kind == Kind::Verb && (tokens[i].text == "Set" || tokens[i].text == "Demand")
            || tokens[i].kind == Kind::Connective && tokens[i].text == "for";
        if opens && i + 1 < tokens.len() && tokens[i + 1].kind == Kind::Variable && tokens[i + 1].text != "$" {
            tokens[i + 1].defines = true;
        }
        i += 1;
    }
}

impl Analysis {
    fn report(
        &mut self,
        line: usize,
        start: usize,
        end: usize,
        severity: Severity,
        code: &'static str,
        message: String,
    ) {
        self.diagnostics.push(Diagnostic {
            line,
            start,
            end: end.max(start + 1),
            severity,
            code,
            message,
            unnecessary: false,
        });
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    /// The token at a column of a line, if there is one.
    pub fn token_at(&self, line: usize, column: usize) -> Option<&Token> {
        let touching = |t: &&Token| t.line == line && t.start <= column && column <= t.end;
        // Prefer the token the column is inside to one it only touches.
        self.tokens
            .iter()
            .filter(touching)
            .find(|t| column < t.end)
            .or_else(|| self.tokens.iter().rfind(touching))
    }

    /// The line that defines a step, a part's first step, or a form, as the
    /// file ends up.
    pub fn definition(&self, named: Named) -> Option<usize> {
        match named {
            Named::Step(n) => self.steps.get(&n).and_then(|lines| lines.last().copied()),
            Named::Part(p) => self.part_lines(p).first().copied(),
            Named::Form(n) => self.forms.get(&n).and_then(|lines| lines.last().copied()),
        }
    }

    /// The lines of the steps of a part, in step order.
    pub fn part_lines(&self, part: i64) -> Vec<usize> {
        self.steps
            .iter()
            .filter(|(n, _)| n.ip().to_i64() == Some(part))
            .filter_map(|(_, lines)| lines.last().copied())
            .collect()
    }

    /// Every token that names the same step, part or form.
    pub fn mentions(&self, named: Named) -> Vec<&Token> {
        self.tokens.iter().filter(|t| t.names == Some(named)).collect()
    }

    /// Every use of a letter.
    pub fn letter_uses(&self, letter: &str) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|t| t.kind == Kind::Variable && t.text == letter)
            .collect()
    }

    /// The parts that have steps, in order.
    pub fn parts(&self) -> Vec<i64> {
        let mut parts: Vec<i64> = self.steps.keys().filter_map(|n| n.ip().to_i64()).collect();
        parts.dedup();
        parts
    }
}

/// Reads a whole file.
pub fn analyse(source: &str) -> Analysis {
    let mut a = Analysis::default();
    let raw: Vec<&str> = source.lines().collect();
    // First pass: what each line is.
    let mut pending_form: Option<i64> = None;
    let mut canon: Vec<(String, Vec<usize>)> = Vec::new();
    for (index, text) in raw.iter().enumerate() {
        let (line, map) = canonical_with_map(text, pending_form.is_some());
        let width = text.chars().count();
        if let Some(n) = pending_form.take() {
            a.forms.entry(n).or_default().push(index);
            a.lines.push(LineInfo {
                kind: LineKind::FormBody(n),
                stmt: None,
            });
            form_tokens(index, &line, &mut a.tokens);
            canon.push((line, map));
            continue;
        }
        // `! interrupt after 16 lines`, `! joss 1964`: not typed, but a key
        // pressed or a setting of the console. Coloured as a note.
        if is_key(text) {
            a.tokens.push(Token {
                kind: Kind::Comment,
                line: index,
                start: 0,
                end: width,
                text: line.clone(),
                defines: false,
                names: None,
            });
            a.lines.push(LineInfo {
                kind: LineKind::Comment,
                stmt: None,
            });
            canon.push((line, map));
            continue;
        }
        let info = match parse::line(&line) {
            Ok(Line::Empty) => LineInfo {
                kind: LineKind::Empty,
                stmt: None,
            },
            Ok(Line::Killed) => {
                a.tokens.push(Token {
                    kind: Kind::Comment,
                    line: index,
                    start: 0,
                    end: width,
                    text: line.clone(),
                    defines: false,
                    names: None,
                });
                LineInfo {
                    kind: LineKind::Comment,
                    stmt: None,
                }
            }
            Ok(Line::Form(number)) => {
                colour(index, &line, &map, &mut a.tokens);
                match number.to_i64().filter(|n| *n > 0) {
                    Some(n) => {
                        pending_form = Some(n);
                        LineInfo {
                            kind: LineKind::FormHead(n),
                            stmt: None,
                        }
                    }
                    None => {
                        a.report(
                            index,
                            0,
                            width,
                            Severity::Error,
                            "form-number",
                            "A form number is a whole number above zero.".into(),
                        );
                        LineInfo {
                            kind: LineKind::Bad,
                            stmt: None,
                        }
                    }
                }
            }
            Ok(Line::Step {
                number,
                text: step_text,
                ..
            }) => {
                colour(index, &line, &map, &mut a.tokens);
                if let Some(first) = a.tokens.iter_mut().rev().take_while(|t| t.line == index).last() {
                    first.kind = Kind::StepNumber;
                }
                let part = number.ip().to_i64().unwrap_or(0);
                let offset = line.chars().count() - line.trim_start_matches([' ', '#']).chars().count()
                    + line
                        .trim_start_matches([' ', '#'])
                        .chars()
                        .take_while(|c| c.is_ascii_digit() || *c == '.')
                        .count();
                let after = line.chars().skip(offset).take_while(|c| *c == ' ' || *c == '#').count();
                let stmt = match parse::step(&step_text) {
                    Ok(stmt) => Some(stmt),
                    Err(Eh { at }) => {
                        let column = map[(offset + after + at).min(map.len() - 1)];
                        a.report(
                            index,
                            column,
                            width,
                            Severity::Error,
                            "eh",
                            "Eh? This step is not JOSS, from here on. JOSS will say so when it reaches the step."
                                .into(),
                        );
                        None
                    }
                };
                if number.sign() <= 0 || part < 1 {
                    a.report(
                        index,
                        0,
                        map[offset.min(map.len() - 1)],
                        Severity::Error,
                        "step-number",
                        "A step number is above zero, and its whole part (the part it belongs to) is at least 1."
                            .into(),
                    );
                    LineInfo {
                        kind: LineKind::Bad,
                        stmt,
                    }
                } else {
                    a.steps.entry(number).or_default().push(index);
                    LineInfo {
                        kind: LineKind::Step { number, part },
                        stmt,
                    }
                }
            }
            Ok(Line::ShortSet(place, value)) => {
                colour(index, &line, &map, &mut a.tokens);
                if let Some(first) = a
                    .tokens
                    .iter_mut()
                    .find(|t| t.line == index && t.kind == Kind::Variable)
                {
                    first.defines = true;
                }
                LineInfo {
                    kind: LineKind::Direct,
                    stmt: Some(Stmt {
                        verb: Verb::Set(place, value),
                        cond: None,
                    }),
                }
            }
            Ok(Line::Direct(stmt)) => {
                colour(index, &line, &map, &mut a.tokens);
                LineInfo {
                    kind: LineKind::Direct,
                    stmt: Some(stmt),
                }
            }
            Err(Eh { at }) => {
                colour(index, &line, &map, &mut a.tokens);
                let _ = at;
                LineInfo {
                    kind: LineKind::Bad,
                    stmt: None,
                }
            }
        };
        a.lines.push(info);
        canon.push((line, map));
    }
    mark_definitions(&mut a.tokens);

    // A line that is not an instruction but reads as a value is an answer to
    // a Demand, if the program has a Demand in it.
    let demands = a.lines.iter().any(|l| {
        matches!(
            &l.stmt,
            Some(Stmt {
                verb: Verb::Demand(..),
                ..
            })
        )
    });
    for index in 0..a.lines.len() {
        if a.lines[index].kind != LineKind::Bad || a.lines[index].stmt.is_some() {
            continue;
        }
        if a.diagnostics.iter().any(|d| d.line == index) {
            continue;
        }
        let (line, map) = &canon[index];
        let as_value = parse::answer(line);
        if demands && as_value.is_ok() {
            a.lines[index].kind = LineKind::Answer;
            continue;
        }
        let at = match parse::line(line) {
            Err(Eh { at }) => at,
            Ok(_) => 0,
        };
        let width = raw[index].chars().count();
        let column = map[at.min(map.len() - 1)].min(width.saturating_sub(1));
        a.report(
            index,
            column,
            width,
            Severity::Error,
            "eh",
            "Eh? This line is not JOSS, from here on.".into(),
        );
    }

    cross_check(&mut a, &raw, &canon);
    a.diagnostics.sort_by_key(|d| (d.line, d.start, d.severity));
    a
}

/// Whether a line of a file is a key or a setting, not something typed: the
/// lines the `joss` command and the transcripts understand.
pub fn is_key(line: &str) -> bool {
    let Some(rest) = line.trim().strip_prefix('!') else {
        return false;
    };
    let words: Vec<&str> = rest.split_whitespace().collect();
    match words.as_slice() {
        ["interrupt", "after", n, "lines" | "line"] | ["users" | "time", n] => n.parse::<u32>().is_ok(),
        ["joss", "1964" | "1965"] => true,
        _ => false,
    }
}

/// Colours the picture of a form: its blanks are fields, the rest is text.
fn form_tokens(line: usize, text: &str, out: &mut Vec<Token>) {
    let chars: Vec<char> = text.chars().collect();
    let found = format::fields(text).unwrap_or_default();
    let mut at = 0;
    let mut push = |kind: Kind, start: usize, end: usize| {
        if end > start && chars[start..end].iter().any(|c| *c != ' ') {
            out.push(Token {
                kind,
                line,
                start,
                end,
                text: chars[start..end].iter().collect(),
                defines: false,
                names: None,
            });
        }
    };
    for field in &found {
        push(Kind::Text, at, field.start);
        push(Kind::Field, field.start, field.start + field.len);
        at = field.start + field.len;
    }
    push(Kind::Text, at, chars.len());
}

/// The checks that need the whole file.
fn cross_check(a: &mut Analysis, raw: &[&str], canon: &[(String, Vec<usize>)]) {
    let width = |line: usize| raw[line].chars().count();

    // A step typed again replaces the one before.
    let replaced: Vec<(usize, usize, Num)> = a
        .steps
        .iter()
        .flat_map(|(n, lines)| lines.windows(2).map(|pair| (pair[0], pair[1], *n)).collect::<Vec<_>>())
        .collect();
    for (old, new, n) in replaced {
        let message = format!(
            "Step {} is typed again on line {}, which replaces this one.",
            format::plain(n),
            new + 1
        );
        a.diagnostics.push(Diagnostic {
            line: old,
            start: 0,
            end: width(old),
            severity: Severity::Hint,
            code: "replaced",
            message,
            unnecessary: true,
        });
    }

    // Forms whose blanks cannot be told apart.
    let form_lines: Vec<(i64, usize)> = a
        .forms
        .iter()
        .flat_map(|(n, lines)| lines.iter().map(|l| (*n, *l)))
        .collect();
    for (_, line) in &form_lines {
        if format::fields(&canon[*line].0) == Err(FormFault::FieldsRunTogether) {
            a.report(*line, 0, width(*line), Severity::Warning, "fields-run-together", "Two blanks touch, so JOSS cannot tell where one number ends and the next begins: \"Your fields run together in the form.\" Put a space between them.".into());
        }
    }

    // Which steps, parts and forms exist: for a step, anywhere in the file;
    // for a direct instruction, above it.
    let mut steps_above: Vec<Num> = Vec::new();
    let mut forms_above: Vec<i64> = Vec::new();
    let all_steps: Vec<Num> = a.steps.keys().copied().collect();
    let all_forms: Vec<i64> = a.forms.keys().copied().collect();
    let mut notes: Vec<(usize, usize, usize, Severity, &'static str, String)> = Vec::new();
    for index in 0..a.lines.len() {
        let kind = a.lines[index].kind.clone();
        let is_step = matches!(kind, LineKind::Step { .. });
        if let LineKind::Step { number, .. } = kind {
            steps_above.push(number);
        }
        if let LineKind::FormBody(n) = kind {
            forms_above.push(n);
        }
        let (steps, forms) = if is_step {
            (&all_steps, &all_forms)
        } else {
            (&steps_above, &forms_above)
        };
        let when = if is_step { "" } else { " yet" };
        for token in a.tokens.iter().filter(|t| t.line == index) {
            let missing = match token.names {
                Some(Named::Step(n)) if !steps.contains(&n) => {
                    Some(format!("There is no step {}{when}.", format::plain(n)))
                }
                Some(Named::Part(p)) if !steps.iter().any(|n| n.ip().to_i64() == Some(p)) => {
                    Some(format!("Part {p} has no steps{when}."))
                }
                Some(Named::Form(n)) if !forms.contains(&n) && !matches!(kind, LineKind::FormHead(_)) => {
                    Some(format!("There is no form {n}{when}."))
                }
                _ => None,
            };
            if let Some(message) = missing {
                notes.push((index, token.start, token.end, Severity::Warning, "missing", message));
            }
        }
        // Do part 1.1: the part number must be whole.
        if let Some(Stmt { verb, .. }) = &a.lines[index].stmt {
            if is_step && verb.direct_only() {
                notes.push((
                    index,
                    0,
                    width(index),
                    Severity::Error,
                    "direct-only",
                    format!(
                        "`{}` like this can only be given directly, not kept as a step.",
                        verb.word()
                    ),
                ));
            }
            if !is_step && verb.indirect_only() {
                notes.push((
                    index,
                    0,
                    width(index),
                    Severity::Error,
                    "indirect-only",
                    format!(
                        "`{}` only means something as a step of a stored program. Give it a step number.",
                        verb.word()
                    ),
                ));
            }
            if let Verb::TypeInForm(values, parse::Expr::Num(form)) = verb {
                let picture = form.to_i64().and_then(|n| a.forms.get(&n)).and_then(|lines| {
                    if is_step {
                        lines.last()
                    } else {
                        lines.iter().rev().find(|l| **l < index)
                    }
                });
                if let Some(line) = picture {
                    if let Ok(found) = format::fields(&canon[*line].0) {
                        if values.len() < found.len() {
                            notes.push((index, 0, width(index), Severity::Warning, "too-few-values", format!("Form {} has {} blanks and this gives {} values: \"I have too few values for the form.\"", format::plain(*form), found.len(), values.len())));
                        }
                    }
                }
            }
        }
        for pair in a
            .tokens
            .iter()
            .filter(|t| t.line == index && kind != LineKind::Bad)
            .collect::<Vec<_>>()
            .windows(2)
        {
            if pair[0].kind == Kind::Noun
                && pair[0].text == "part"
                && pair[1].kind == Kind::Number
                && pair[1].text.contains('.')
            {
                notes.push((
                    index,
                    pair[1].start,
                    pair[1].end,
                    Severity::Error,
                    "part-number",
                    "A part number is a whole number: \"Part number must be integer and 0<part<10*9.\"".into(),
                ));
            }
        }
    }
    for (line, start, end, severity, code, message) in notes {
        a.report(line, start, end, severity, code, message);
    }

    // Letters used and never given a value anywhere.
    let defined: Vec<String> = a.tokens.iter().filter(|t| t.defines).map(|t| t.text.clone()).collect();
    let mut told: Vec<String> = Vec::new();
    let mut unset = Vec::new();
    for token in a.tokens.iter().filter(|t| t.kind == Kind::Variable && t.text != "$") {
        if matches!(a.lines[token.line].kind, LineKind::FormBody(_)) {
            continue;
        }
        // `Delete x.` and a bare `Type b.` of nothing are not uses worth a note.
        if !defined.contains(&token.text) && !told.contains(&token.text) {
            told.push(token.text.clone());
            unset.push((token.line, token.start, token.end, token.text.clone()));
        }
    }
    for (line, start, end, letter) in unset {
        a.report(line, start, end, Severity::Warning, "never-set", format!("Nothing in this file gives {letter} a value. If it has none when this is done, JOSS says \"{letter} is undefined.\""));
    }
}

/// The line and column of a diagnostic as a person counts them, from 1.
pub fn position(d: &Diagnostic) -> (usize, usize) {
    (d.line + 1, d.start + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIMES: &str = "* PRIME NUMBERS\n1.1 Do part 2 for d = 2(1)ip[sqrt(x)] if x > 3.\n1.2 Type x if x != 0.\n2.1 Set x = 0 if fp(x/d) = 0.\nDo part 1 for x = 2(1)50.\n";

    fn codes(source: &str) -> Vec<(&'static str, usize)> {
        analyse(source)
            .diagnostics
            .iter()
            .map(|d| (d.code, d.line + 1))
            .collect()
    }

    #[test]
    fn a_program_from_1965_has_nothing_wrong_with_it() {
        let a = analyse(PRIMES);
        assert_eq!(a.diagnostics, []);
        assert_eq!(a.lines[0].kind, LineKind::Comment);
        assert!(matches!(a.lines[1].kind, LineKind::Step { part: 1, .. }));
        assert_eq!(a.lines[4].kind, LineKind::Direct);
        assert_eq!(a.parts(), [1, 2]);
        assert_eq!(a.definition(Named::Part(2)), Some(3));
        // d is given its values by the for clause, x by the direct Do.
        assert!(a.letter_uses("d").iter().any(|t| t.defines));
    }

    #[test]
    fn stand_ins_keep_their_columns() {
        let a = analyse("1.2 Type x if x != 0.\nSet x = 1.\n");
        let zero = a.tokens.iter().find(|t| t.text == "0").unwrap();
        assert_eq!((zero.start, zero.end), (19, 20));
        let ne = a.tokens.iter().find(|t| t.text == "≠").unwrap();
        assert_eq!((ne.start, ne.end), (16, 18));
    }

    #[test]
    fn what_can_be_found_without_running() {
        assert_eq!(codes("Do part1.1.\n"), [("eh", 1)]);
        assert_eq!(codes("1.1 Type x\nSet x=1.\n"), [("eh", 1)]);
        assert_eq!(codes("Set x=1.\n1.1 Type x.\nDo part 2.\n"), [("missing", 3)]);
        assert_eq!(codes("Set x=1.\nDo part 1.\n1.1 Type x.\n"), [("missing", 2)]);
        assert_eq!(codes("Set x=1.\n1.1 Type x.\n1.1 Type x+1.\n"), [("replaced", 2)]);
        assert_eq!(codes("1.1 Type y.\n"), [("never-set", 1)]);
        assert_eq!(codes("1.1 Go.\n"), [("direct-only", 1)]);
        assert_eq!(codes("Done.\n"), [("indirect-only", 1)]);
        assert_eq!(
            codes("Set a=1.\nForm 1:\na = __ b = __\nType a in form 1.\n"),
            [("too-few-values", 4)]
        );
        assert_eq!(codes("Set a=1.\nType a in form 1.\n"), [("missing", 2)]);
        assert_eq!(
            codes("Form 1:\nroots: ___._________.____\n"),
            [("fields-run-together", 2)]
        );
        assert_eq!(codes("Set x=1.\n1.1 Type x.\nDo part 1.1.\n"), [("part-number", 3)]);
    }

    #[test]
    fn answers_to_a_demand_are_not_mistakes() {
        let a = analyse("1.1 Demand x.\n1.2 Type x*2.\nDo part 1.\n12\n");
        assert_eq!(a.diagnostics, []);
        assert_eq!(a.lines[3].kind, LineKind::Answer);
        assert_eq!(codes("Set x=1.\n12\n"), [("eh", 2)]);
    }

    #[test]
    fn a_key_is_not_a_line_typed() {
        let a = analyse("! joss 1964\n1.1 Type i.\n! interrupt after 3 lines\nDo step 1.1 for i=1(1)9.\n");
        assert_eq!(a.diagnostics, []);
        assert_eq!(a.lines[2].kind, LineKind::Comment);
        assert_eq!(codes("! what\n"), [("eh", 1)]);
    }

    #[test]
    fn a_form_is_a_picture() {
        let a = analyse("Form 1:\na =___   roots: ___.____\n");
        let kinds: Vec<Kind> = a.tokens.iter().filter(|t| t.line == 1).map(|t| t.kind).collect();
        assert_eq!(kinds, [Kind::Text, Kind::Field, Kind::Text, Kind::Field]);
    }
}
