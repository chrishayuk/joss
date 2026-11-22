//! Splits a typed line into tokens.
//!
//! A JOSS line is read a character at a time off a typewriter, and the rules
//! are those of the keyboard. A single letter is a name (there are 52 of
//! them, capitals and small letters being different names). Two or more
//! letters together are a word of the language: `Set`, `step`, `sqrt`. The
//! strike-out `#` is a space. Positions are counted in characters, which on a
//! typewriter are columns.

use crate::chars::{DOT, GE, LE, NE};

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    /// A numeral: digits with at most one point. No sign, no exponent.
    Number(String),
    /// One letter: a name the user can give a value to.
    Letter(char),
    /// Two or more letters.
    Word(String),
    /// What stood between two double quotes.
    Text(String),
    /// A run of underscores: a blank line asked for in a `Type` list.
    Blank,
    Period,
    Comma,
    Colon,
    Semicolon,
    /// `(` or `[`. They pair with their own kind but mean the same.
    Open(char),
    Close(char),
    Bar,
    Plus,
    Minus,
    Times,
    Over,
    Power,
    Eq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
    Dollar,
    /// Anything the typewriter could send that the language has no use for.
    Other(char),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub tok: Tok,
    /// Columns, counted from 0: `start` is the first, `end` is one past the last.
    pub start: usize,
    pub end: usize,
    /// Whether a space (or a struck-out character) came before it.
    pub spaced: bool,
}

pub fn lex(line: &str) -> Vec<Token> {
    let chars: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let mut spaced = false;
    while i < chars.len() {
        let c = chars[i];
        let start = i;
        if c == ' ' || c == '#' || c == '\t' {
            spaced = true;
            i += 1;
            continue;
        }
        let digit_next = chars.get(i + 1).is_some_and(|d| d.is_ascii_digit());
        let tok = if c.is_ascii_digit() || (c == '.' && digit_next) {
            let mut text = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                text.push(chars[i]);
                i += 1;
            }
            // A point belongs to the numeral only if a digit follows it:
            // the point after `Set x = 1.` ends the sentence.
            if i + 1 < chars.len() && chars[i] == '.' && chars[i + 1].is_ascii_digit() {
                text.push('.');
                i += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    text.push(chars[i]);
                    i += 1;
                }
            }
            Tok::Number(text)
        } else if c.is_ascii_alphabetic() {
            let mut text = String::new();
            while i < chars.len() && chars[i].is_ascii_alphabetic() {
                text.push(chars[i]);
                i += 1;
            }
            if text.len() == 1 {
                Tok::Letter(c)
            } else {
                Tok::Word(text)
            }
        } else if c == '"' {
            i += 1;
            let mut text = String::new();
            let mut closed = false;
            while i < chars.len() {
                if chars[i] == '"' {
                    closed = true;
                    i += 1;
                    break;
                }
                text.push(chars[i]);
                i += 1;
            }
            if closed {
                Tok::Text(text)
            } else {
                Tok::Other('"')
            }
        } else if c == '_' {
            while i < chars.len() && chars[i] == '_' {
                i += 1;
            }
            Tok::Blank
        } else {
            i += 1;
            match c {
                '.' => Tok::Period,
                ',' => Tok::Comma,
                ':' => Tok::Colon,
                ';' => Tok::Semicolon,
                '(' | '[' => Tok::Open(c),
                ')' | ']' => Tok::Close(c),
                '|' => Tok::Bar,
                '+' => Tok::Plus,
                '-' => Tok::Minus,
                '/' => Tok::Over,
                '*' => Tok::Power,
                '=' => Tok::Eq,
                '<' => Tok::Lt,
                '>' => Tok::Gt,
                '$' => Tok::Dollar,
                _ if c == DOT => Tok::Times,
                _ if c == NE => Tok::Ne,
                _ if c == LE => Tok::Le,
                _ if c == GE => Tok::Ge,
                _ => Tok::Other(c),
            }
        };
        out.push(Token {
            tok,
            start,
            end: i,
            spaced,
        });
        spaced = false;
    }
    out
}

/// The words of the language. Anything else of two letters or more is not JOSS.
pub const VERBS: &[&str] = &[
    "Set", "Do", "Type", "Delete", "Line", "Page", "To", "Done", "Stop", "Demand", "Cancel", "Go", "Form",
];
pub const NOUNS: &[&str] = &[
    "step", "part", "form", "all", "steps", "parts", "forms", "values", "size", "time", "users",
];
pub const CONNECTIVES: &[&str] = &["if", "for", "in", "and", "or"];
pub const FUNCTIONS: &[&str] = &[
    "sqrt", "log", "exp", "sin", "cos", "arg", "ip", "fp", "dp", "xp", "sgn", "max", "min",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(line: &str) -> Vec<Tok> {
        lex(line).into_iter().map(|t| t.tok).collect()
    }

    #[test]
    fn the_point_after_a_numeral_ends_the_sentence() {
        assert_eq!(
            toks("Do step 1.1."),
            [
                Tok::Word("Do".into()),
                Tok::Word("step".into()),
                Tok::Number("1.1".into()),
                Tok::Period
            ]
        );
        assert_eq!(
            toks("Set a=1."),
            [
                Tok::Word("Set".into()),
                Tok::Letter('a'),
                Tok::Eq,
                Tok::Number("1".into()),
                Tok::Period
            ]
        );
        assert_eq!(toks(".5"), [Tok::Number(".5".into())]);
    }

    #[test]
    fn letters_together_are_a_word() {
        assert_eq!(toks("part1.1"), [Tok::Word("part".into()), Tok::Number("1.1".into())]);
        assert!(!lex("part1.1")[1].spaced);
        assert!(lex("part 1.1")[1].spaced);
        assert_eq!(toks("x#y"), [Tok::Letter('x'), Tok::Letter('y')]);
    }
}
