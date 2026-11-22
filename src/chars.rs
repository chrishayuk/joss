//! The JOSS typewriter's characters, and how to type them today.
//!
//! The IBM 868 typewriters on the JOHNNIAC had a keyboard chosen for JOSS:
//! "brackets and the symbols for arithmetic operations and numerical relations
//! replaced the less frequently used special characters" (P-3149, p. 3). Four
//! of them are not on a modern keyboard, so each has a stand-in that is
//! accepted on input. Output always uses the real character.
//!
//! | JOSS | meaning          | you can also type |
//! |------|------------------|-------------------|
//! | `·`  | times            | `@` or `×`        |
//! | `≠`  | is not equal to  | `!=`              |
//! | `≤`  | at most          | `<=`              |
//! | `≥`  | at least         | `>=`              |
//!
//! `*` is not times. It is "to the power": "a five-pointed, upward-pointing,
//! slightly elevated asterisk" (P-2922, p. 11).

pub const DOT: char = '·';
pub const NE: char = '≠';
pub const LE: char = '≤';
pub const GE: char = '≥';

/// A line as the JOSS typewriter would hold it: stand-ins replaced by the
/// real characters, and every character counted as one column. Text inside
/// double quotes is left alone, and so is everything when `literal` is set
/// (the line of a form, which is a picture and not an instruction).
pub fn canonical(line: &str, literal: bool) -> String {
    canonical_with_map(line, literal).0
}

/// The same, with the column in `line` that each column of the result came
/// from (and one more entry, for the end of the line). Editors use it to
/// point back at what was typed.
pub fn canonical_with_map(line: &str, literal: bool) -> (String, Vec<usize>) {
    let chars: Vec<char> = line.trim_end_matches(['\r', '\n']).chars().collect();
    let mut out = String::with_capacity(line.len());
    let mut map = Vec::with_capacity(chars.len() + 1);
    let mut quoted = false;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        map.push(i);
        if literal {
            out.push(c);
            i += 1;
            continue;
        }
        if c == '"' {
            quoted = !quoted;
        }
        if quoted {
            out.push(c);
            i += 1;
            continue;
        }
        let (put, took) = match (c, next) {
            ('!', Some('=')) => (NE, 2),
            ('<', Some('=')) => (LE, 2),
            ('>', Some('=')) => (GE, 2),
            ('@' | '×' | '∙' | '•' | '⋅', _) => (DOT, 1),
            ('“' | '”', _) => {
                quoted = !quoted;
                ('"', 1)
            }
            ('−' | '–', _) => ('-', 1),
            ('\t', _) => (' ', 1),
            _ => (c, 1),
        };
        out.push(put);
        i += took;
    }
    map.push(chars.len());
    (out, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stand_ins_become_the_real_characters() {
        assert_eq!(
            canonical("Set x = 2@a if a != 0 and b <= c.", false),
            "Set x = 2·a if a ≠ 0 and b ≤ c."
        );
        assert_eq!(canonical("Type \"a <= b @ c\".", false), "Type \"a <= b @ c\".");
        assert_eq!(canonical("a <= __.__ @", true), "a <= __.__ @");
        let (text, map) = canonical_with_map("if a<=b.", false);
        assert_eq!(text, "if a≤b.");
        assert_eq!(map, [0, 1, 2, 3, 4, 6, 7, 8]);
    }
}
