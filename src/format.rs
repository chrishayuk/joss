//! How JOSS types numbers: on a line of their own, and into a form.
//!
//! "If the user fails to specify a form for the output, JOSS chooses a
//! reasonable one: JOSS types in fixed-point form if the magnitude of the
//! result is between .001 and 1,000,000; otherwise, JOSS types in scientific
//! notation." (P-3149, p. 8)
//!
//! "Each number is identified by the very expression used in the step calling
//! for the output. JOSS tries to line up equals signs and decimal points."
//! (P-2922, p. 28)

use crate::chars::DOT;
use crate::num::Num;
use crate::session::Year;

/// The column (counting from 0) of the `=` in `Type x.`: the label is set
/// right in eight columns, then a space, then `=`.
pub const EQUALS_COLUMN: usize = 9;

/// The column of the units digit. In the figures of 1964 it is column 16;
/// in the pages of April and May 1965 everything after the `=` has moved one
/// place to the right. (P-2922 fig. 3 and RM-4162-PR examples 3 and 5,
/// against P-3131 and P-3149 fig. 1.)
pub fn units_column(year: Year) -> usize {
    match year {
        Year::Y1964 => 16,
        Year::Y1965 => 17,
    }
}

/// Whether a number is typed without an exponent when no form is given.
///
/// In 1965: "between .001 and 1,000,000". The JOSS of the 1964 film was
/// quicker to use an exponent for a small number: its `Type all` has
/// `L =       .2` but `t =      2·10*(-2)` and `Q =      1·10*(-3)`
/// (RM-4162-PR, example 3), so there the line is drawn at a tenth.
pub fn is_fixed(n: Num, year: Year) -> bool {
    let smallest = if year == Year::Y1964 { -1 } else { -3 };
    n.is_zero() || (smallest..=5).contains(&n.exponent())
}

/// The digits of a number as JOSS types it when left to itself: the part
/// before the point (with its sign) and the part from the point on.
/// 1.5 gives ("1", ".5"); −.618 gives ("-", ".618"); 2.68811714·10*43 gives
/// ("2", ".68811714·10*43").
pub fn free(n: Num, year: Year) -> (String, String) {
    let sign = if n.is_negative() { "-" } else { "" };
    if is_fixed(n, year) {
        let (whole, fraction) = n.fixed_digits();
        let tail = if fraction.is_empty() {
            String::new()
        } else {
            format!(".{fraction}")
        };
        (format!("{sign}{whole}"), tail)
    } else {
        let (whole, fraction) = n.dp().abs().fixed_digits();
        let tail = if fraction.is_empty() {
            String::new()
        } else {
            format!(".{fraction}")
        };
        // A negative power is typed in parentheses: 2·10*(-4).
        let power = if n.exponent() < 0 {
            format!("({})", n.exponent())
        } else {
            n.exponent().to_string()
        };
        (format!("{sign}{whole}"), format!("{tail}{DOT}10*{power}"))
    }
}

/// A number as it appears inside a sentence JOSS types (a step number, say).
pub fn plain(n: Num) -> String {
    let (head, tail) = free(n, Year::Y1965);
    format!("{head}{tail}")
}

/// The start of a value line, up to and including the `=`: the label set
/// right so that the `=` falls in its column, or, if the label is too long
/// for that, starting at the margin.
pub fn label(label: &str) -> String {
    let width = label.chars().count();
    format!("{}{label} =", " ".repeat((EQUALS_COLUMN - 1).saturating_sub(width)))
}

/// The line JOSS types for one value: `       x =       3`.
pub fn value_line(name: &str, n: Num, year: Year) -> String {
    let (head, tail) = free(n, year);
    let mut line = label(name);
    let used = line.chars().count();
    // After a long label the number follows the `=` at once: a place for
    // the sign in 1964, a space and then a place for the sign in 1965.
    let least = usize::from(year == Year::Y1965) + usize::from(!n.is_negative());
    let head_width = head.chars().count();
    let pad = (units_column(year) + 1).saturating_sub(used + head_width).max(least);
    line.push_str(&" ".repeat(pad));
    line.push_str(&head);
    line.push_str(&tail);
    line
}

/// One blank to be filled in a form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Field {
    /// Columns in the form.
    pub start: usize,
    pub len: usize,
    pub kind: FieldKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldKind {
    /// Underscores, with a point if there are to be decimals: `___.__`.
    Fixed { decimals: usize, point: bool },
    /// A row of periods: "a tabular form of scientific notation in which
    /// only the digit part and the corresponding exponent part are typed
    /// with the base ten understood".
    Scientific,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFault {
    FieldsRunTogether,
    TooFewValues,
    TooManyValues,
    CantExpress,
    /// "Field definition has fewer than 7 periods."
    ShortField,
}

/// The fewest periods that make a field. One period alone is punctuation:
/// the summary's own example is `dist. = ..........`. Two to six are a
/// mistake: "Field definition has fewer than 7 periods."
pub const SHORTEST_SCIENTIFIC: usize = 7;

/// Finds the fields of a form.
pub fn fields(form: &str) -> Result<Vec<Field>, FormFault> {
    let chars: Vec<char> = form.chars().collect();
    let mut found = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '_' && chars[i] != '.' {
            i += 1;
            continue;
        }
        let mut start = i;
        while i < chars.len() && (chars[i] == '_' || chars[i] == '.') {
            i += 1;
        }
        let run = &chars[start..i];
        // A hyphen typed against the front of a blank keeps a column for the
        // sign: `-_.______` (RM-4162-PR, forms 2 and 3).
        let signed = start > 0 && chars[start - 1] == '-' && run.contains(&'_');
        if signed {
            start -= 1;
        }
        let extra = usize::from(signed);
        let points = run.iter().filter(|c| **c == '.').count();
        if points == run.len() {
            if run.len() >= SHORTEST_SCIENTIFIC {
                found.push(Field {
                    start,
                    len: run.len(),
                    kind: FieldKind::Scientific,
                });
            } else if run.len() > 1 {
                return Err(FormFault::ShortField);
            }
            continue;
        }
        // A period that ends a sentence straight after a field is not part
        // of it: `x = ___.` followed by nothing or a space.
        if points > 1 {
            return Err(FormFault::FieldsRunTogether);
        }
        let decimals = match run.iter().position(|c| *c == '.') {
            Some(at) => run.len() - at - 1,
            None => 0,
        };
        found.push(Field {
            start,
            len: run.len() + extra,
            kind: FieldKind::Fixed {
                decimals,
                point: points == 1,
            },
        });
    }
    Ok(found)
}

fn fixed_field(n: Num, width: usize, decimals: usize, point: bool) -> Option<String> {
    let rounded = n.round_places(decimals as i32);
    let (whole, mut fraction) = rounded.fixed_digits();
    while fraction.len() < decimals {
        fraction.push('0');
    }
    let mut text = String::new();
    if rounded.is_negative() {
        text.push('-');
    }
    // No nought in front of a fraction: `-.6180`. A whole number that is
    // zero is typed as `0`.
    if !whole.is_empty() && (whole != "0" || decimals == 0) {
        text.push_str(&whole);
    }
    if point {
        text.push('.');
        text.push_str(&fraction);
    }
    let used = text.chars().count();
    if used > width {
        return None;
    }
    Some(format!("{}{}", " ".repeat(width - used), text))
}

/// A row of periods: a sign or a space, the digit part, the sign of the
/// power or a space, and the power in two digits. Thirteen periods hold
/// ` 5.4598150 01`.
fn scientific_field(n: Num, width: usize) -> Option<String> {
    // sign, digit, point, decimals, power sign, two digits
    let decimals = width.checked_sub(6).filter(|d| *d >= 1)?;
    let rounded = n.round_digits((decimals as u32 + 1).min(9));
    let nine = if rounded.is_zero() {
        "000000000".to_string()
    } else {
        rounded.mantissa().to_string()
    };
    let mut text = String::new();
    text.push(if rounded.is_negative() { '-' } else { ' ' });
    text.push_str(&nine[..1]);
    text.push('.');
    for i in 0..decimals {
        text.push(nine.chars().nth(i + 1).unwrap_or('0'));
    }
    let e = rounded.exponent();
    text.push(if e < 0 { '-' } else { ' ' });
    text.push_str(&format!("{:02}", e.abs()));
    Some(text)
}

/// The form with its fields filled in.
pub fn fill(form: &str, values: &[Num]) -> Result<String, FormFault> {
    let found = fields(form)?;
    if values.len() < found.len() {
        return Err(FormFault::TooFewValues);
    }
    if values.len() > found.len() {
        return Err(FormFault::TooManyValues);
    }
    let mut line: Vec<char> = form.chars().collect();
    for (field, value) in found.iter().zip(values) {
        let text = match field.kind {
            FieldKind::Fixed { decimals, point } => fixed_field(*value, field.len, decimals, point),
            FieldKind::Scientific => scientific_field(*value, field.len),
        }
        .ok_or(FormFault::CantExpress)?;
        for (i, c) in text.chars().enumerate() {
            line[field.start + i] = c;
        }
    }
    Ok(line.into_iter().collect::<String>().trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(text: &str) -> Num {
        match text.strip_prefix('-') {
            Some(rest) => Num::numeral(rest).unwrap().neg(),
            None => Num::numeral(text).unwrap(),
        }
    }

    #[test]
    fn the_quadratic_form_of_1965() {
        // Figure 2 of P-3149: the form that works, and the three that do not.
        let form = "a =___   b =___   c =___   roots: ___.____ ___.____";
        let values = [n("1"), n("-1"), n("-6"), n("3"), n("-2")];
        assert_eq!(
            fill(form, &values).unwrap(),
            "a =  1   b = -1   c = -6   roots:   3.0000  -2.0000"
        );
        let values = [n("1"), n("-1"), n("-1"), n("1.61803399"), n("-.618033989")];
        assert_eq!(
            fill(form, &values).unwrap(),
            "a =  1   b = -1   c = -1   roots:   1.6180   -.6180"
        );
        assert_eq!(fill(form, &values[..2]), Err(FormFault::TooFewValues));
        let no_room = "a =___   b =___   c =___   roots:      .____      .____";
        assert_eq!(
            fill(no_room, &[n("1"), n("-1"), n("-6"), n("3"), n("-2")]),
            Err(FormFault::CantExpress)
        );
        let together = "a =___   b =___   c =___   roots: ___._________.____";
        assert_eq!(fill(together, &values), Err(FormFault::FieldsRunTogether));
    }

    #[test]
    fn the_forms_of_the_1964_film() {
        // RM-4162-PR: form 1 of the first program, and form 3 of the second.
        let form = "___  __._______  __._______   .............";
        let row = [n("4"), n("2"), n("1.38629436"), n("54.59815")];
        assert_eq!(fill(form, &row).unwrap(), "  4   2.0000000   1.3862944    5.4598150 01");
        let row = [n("1"), n("1"), n("0"), n("2.71828183")];
        assert_eq!(fill(form, &row).unwrap(), "  1   1.0000000    .0000000    2.7182818 00");
        let form = "__.__ ms.   -_.______ amp.";
        assert_eq!(fill(form, &[n("0"), n("0")]).unwrap(), "  .00 ms.     .000000 amp.");
        assert_eq!(
            fill(form, &[n("1.2"), n("-1.405807")]).unwrap(),
            " 1.20 ms.   -1.405807 amp."
        );
        assert_eq!(
            fill(form, &[n(".2"), n("1.778902")]).unwrap(),
            "  .20 ms.    1.778902 amp."
        );
    }

    #[test]
    fn the_value_lines_of_the_1964_film() {
        // RM-4162-PR, example 3: `Type all.`
        assert_eq!(
            value_line("d", n(".0002"), Year::Y1964),
            format!("       d =      2{DOT}10*(-4)")
        );
        assert_eq!(
            value_line("f", n("501.716868"), Year::Y1964),
            "       f =    501.716868"
        );
        assert_eq!(value_line("L", n(".2"), Year::Y1964), "       L =       .2");
        assert_eq!(value_line("R", n("100"), Year::Y1964), "       R =    100");
        assert_eq!(
            value_line("t", n(".02"), Year::Y1964),
            format!("       t =      2{DOT}10*(-2)")
        );
        assert_eq!(value_line("t", n(".02"), Year::Y1965), "       t =        .02");
    }

    #[test]
    fn the_value_lines_of_1965() {
        // P-3131, April 1965: one column further right than in 1964.
        let y = Year::Y1965;
        assert_eq!(value_line("2+2", n("4"), y), "     2+2 =       4");
        assert_eq!(value_line("sgn(-3.5)", n("-1"), y), "sgn(-3.5) =     -1");
        assert_eq!(
            value_line("4·arg(3,3)", n("3.14159265"), y),
            "4·arg(3,3) =     3.14159265"
        );
        assert_eq!(value_line("1.23456·10*2", n("123.456"), y), "1.23456·10*2 =  123.456");
        assert_eq!(value_line("max(6·x, 20, x*3)", n("27"), y), "max(6·x, 20, x*3) =  27");
        assert_eq!(value_line("y", n(".0992174964"), y), "       y =        .0992174964");
        // P-2922, August 1964.
        let y = Year::Y1964;
        assert_eq!(value_line("2+2", n("4"), y), "     2+2 =      4");
        assert_eq!(value_line("sgn(-3.5)", n("-1"), y), "sgn(-3.5) =    -1");
        assert_eq!(value_line("1.23456·10*2", n("123.456"), y), "1.23456·10*2 = 123.456");
        assert_eq!(value_line("sin(.5)*2+cos(.5)*2", n("1"), y), "sin(.5)*2+cos(.5)*2 = 1");
        assert_eq!(
            value_line(
                "exp(x)",
                n("2.68811714").mul(Num::int(10).pow(Num::int(43)).unwrap()).unwrap(),
                y
            ),
            format!("  exp(x) =      2.68811714{DOT}10*43")
        );
    }

    #[test]
    fn free_format() {
        assert_eq!(free(n("1.5"), Year::Y1965), ("1".into(), ".5".into()));
        assert_eq!(free(n("-.618"), Year::Y1965), ("-".into(), ".618".into()));
        assert_eq!(free(n("1000000"), Year::Y1965).1, format!("{DOT}10*6"));
        assert_eq!(free(n(".001"), Year::Y1965), ("".into(), ".001".into()));
        assert_eq!(plain(n(".0001")), format!("1{DOT}10*(-4)"));
        assert_eq!(plain(n("999999.5")), "999999.5");
    }
}
