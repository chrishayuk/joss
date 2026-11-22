//! Reads one typed line as a JOSS instruction.
//!
//! JOSS-I as it stood on 12 January 1965 (the one-page summary printed as
//! figure 3 of P-3149). A line is a direct instruction, or a step (an
//! instruction with a step number in front, kept for later), or the heading
//! of a form (`Form 2:`, the next line being the form itself).
//!
//! There is one error: a line that is not JOSS. JOSS's whole comment on it
//! was "Eh?": "the user is forced to read his step to find the error, rather
//! than possibly being misled by a message unrelated to the actual error"
//! (P-2922, p. 27). So this parser returns `Err(Eh { at })`, where `at` is the
//! column it gave up at. The original did not say where; an editor can.

use crate::lex::{lex, Tok, Token, FUNCTIONS};
use crate::num::Num;

/// "Eh?" The column is for editors, not for JOSS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Eh {
    pub at: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Func {
    Sqrt,
    Log,
    Exp,
    Sin,
    Cos,
    Arg,
    Ip,
    Fp,
    Dp,
    Xp,
    Sgn,
    Max,
    Min,
}

impl Func {
    pub fn named(name: &str) -> Option<Func> {
        Some(match name {
            "sqrt" => Func::Sqrt,
            "log" => Func::Log,
            "exp" => Func::Exp,
            "sin" => Func::Sin,
            "cos" => Func::Cos,
            "arg" => Func::Arg,
            "ip" => Func::Ip,
            "fp" => Func::Fp,
            "dp" => Func::Dp,
            "xp" => Func::Xp,
            "sgn" => Func::Sgn,
            "max" => Func::Max,
            "min" => Func::Min,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        FUNCTIONS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Num(Num),
    /// A numeral of more than nine significant digits. It reads, and is
    /// refused when it is reached: "Please limit numbers to 9 significant
    /// digits."
    TooLong,
    /// A letter, with its indices if it has any: `x`, `b(i)`, `w[a,b]`.
    Var(Place),
    /// `$`: the number of the line the typewriter is on.
    Dollar,
    Neg(Box<Expr>),
    Abs(Box<Expr>),
    Bin(BinOp, Box<Expr>, Box<Expr>),
    Call(Func, Vec<Expr>),
}

/// Something a value can be kept in.
#[derive(Clone, Debug, PartialEq)]
pub struct Place {
    pub letter: char,
    pub indices: Vec<Expr>,
    /// Columns of the letter, for editors.
    pub at: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rel {
    Eq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
}

/// `if a<b<c and d=e or f≠g`.
#[derive(Clone, Debug, PartialEq)]
pub enum Cond {
    /// A chain of comparisons, each of which must hold: `a<b<c`.
    Chain(Expr, Vec<(Rel, Expr)>),
    And(Box<Cond>, Box<Cond>),
    Or(Box<Cond>, Box<Cond>),
}

/// One item after `for x =`: a single value, or a run `a(b)c(d)e` that
/// starts at a, goes up in steps of b as far as c, then in steps of d as far
/// as e.
#[derive(Clone, Debug, PartialEq)]
pub enum Run {
    Value(Expr),
    Range(Expr, Vec<(Expr, Expr)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct For {
    pub place: Place,
    pub runs: Vec<Run>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Step,
    Part,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Steps,
    Parts,
    Forms,
    Values,
}

/// One thing in a `Type` list.
#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    /// An expression, with the text of it as typed: JOSS labels each answer
    /// "by the very expression used in the step calling for the output".
    Value(Expr, String),
    /// `_`: a blank line.
    Blank,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Verb {
    Set(Place, Expr),
    Do(Unit, Expr, Option<For>),
    Type(Vec<Item>),
    TypeInForm(Vec<Expr>, Expr),
    TypeText(String),
    TypeStep(Expr),
    TypePart(Expr),
    TypeForm(Expr),
    TypeAll(Option<Kind>),
    TypeSize,
    TypeTime,
    TypeUsers,
    /// `Delete x, y.`: values, by letter or by element.
    DeleteValues(Vec<Place>),
    DeleteStep(Expr),
    DeletePart(Expr),
    DeleteForm(Expr),
    DeleteAll(Option<Kind>),
    Line,
    Page,
    To(Unit, Expr),
    Done,
    Stop,
    Demand(Place, String),
    Cancel,
    Go,
}

impl Verb {
    /// The word the instruction begins with.
    pub fn word(&self) -> &'static str {
        match self {
            Verb::Set(..) => "Set",
            Verb::Do(..) => "Do",
            Verb::Type(..)
            | Verb::TypeInForm(..)
            | Verb::TypeText(..)
            | Verb::TypeStep(..)
            | Verb::TypePart(..)
            | Verb::TypeForm(..)
            | Verb::TypeAll(..)
            | Verb::TypeSize
            | Verb::TypeTime
            | Verb::TypeUsers => "Type",
            Verb::DeleteValues(..)
            | Verb::DeleteStep(..)
            | Verb::DeletePart(..)
            | Verb::DeleteForm(..)
            | Verb::DeleteAll(..) => "Delete",
            Verb::Line => "Line",
            Verb::Page => "Page",
            Verb::To(..) => "To",
            Verb::Done => "Done",
            Verb::Stop => "Stop",
            Verb::Demand(..) => "Demand",
            Verb::Cancel => "Cancel",
            Verb::Go => "Go",
        }
    }

    /// Instructions that may only be given directly, never kept as a step.
    pub fn direct_only(&self) -> bool {
        matches!(
            self,
            Verb::Cancel
                | Verb::Go
                | Verb::DeleteStep(..)
                | Verb::DeletePart(..)
                | Verb::DeleteForm(..)
                | Verb::DeleteAll(None | Some(Kind::Steps | Kind::Parts | Kind::Forms))
        )
    }

    /// Instructions that only make sense as a step of a stored program.
    pub fn indirect_only(&self) -> bool {
        matches!(self, Verb::To(..) | Verb::Done | Verb::Stop | Verb::Demand(..))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub verb: Verb,
    pub cond: Option<Cond>,
}

/// What a typed line turned out to be.
#[derive(Clone, Debug, PartialEq)]
pub enum Line {
    /// Nothing but spaces.
    Empty,
    /// `*` at the beginning or the end "kills an instruction line": JOSS
    /// ignores it, so it serves as a note or a cancelled line.
    Killed,
    /// An instruction to carry out now.
    Direct(Stmt),
    /// `x=a`: a `Set` without the word, given directly.
    ShortSet(Place, Expr),
    /// A step number and the text after it, to keep. The text is kept as
    /// typed and read when the step is done.
    Step { number: Num, label: String, text: String },
    /// `Form 2:`. The next line typed is the form. The number may be any
    /// expression: the January 1965 listing evaluates it (card G011E020,
    /// "EVALUATE FORM NUMBER") and then checks it (G011E030, "VALIDATE").
    Form(Expr),
}

/// The first and last characters that are not spaces, if any.
fn ends(line: &str) -> Option<(char, char)> {
    let trimmed = line.trim_matches([' ', '#', '\t']);
    Some((trimmed.chars().next()?, trimmed.chars().last()?))
}

/// Reads a line typed at the console.
pub fn line(text: &str) -> Result<Line, Eh> {
    let Some((first, last)) = ends(text) else {
        return Ok(Line::Empty);
    };
    if first == '*' || last == '*' {
        return Ok(Line::Killed);
    }
    let tokens = lex(text);
    if let Tok::Number(digits) = &tokens[0].tok {
        // A step. Its number must be a plain numeral; what follows is kept
        // as it stands.
        let number = Num::numeral(digits).ok_or(Eh { at: 0 })?;
        let rest: String = text.chars().skip(tokens[0].end).collect();
        if tokens.len() < 2 || !tokens[1].spaced {
            return Err(Eh { at: tokens[0].end });
        }
        return Ok(Line::Step {
            number,
            label: digits.clone(),
            text: rest.trim_matches([' ', '#', '\t']).to_string(),
        });
    }
    let mut p = Parser {
        tokens: &tokens,
        at: 0,
        chars: text.chars().collect(),
        ranging: false,
    };
    if p.word("Form") {
        let number = p.expr()?;
        p.expect(Tok::Colon)?;
        p.end()?;
        return Ok(Line::Form(number));
    }
    if matches!(tokens[0].tok, Tok::Letter(_)) {
        let place = p.place()?;
        p.expect(Tok::Eq)?;
        let value = p.expr()?;
        // The summary shows `x=a` with no full stop; one is allowed.
        if p.peek() == Some(&Tok::Period) {
            p.at += 1;
        }
        p.end()?;
        return Ok(Line::ShortSet(place, value));
    }
    let stmt = p.stmt()?;
    Ok(Line::Direct(stmt))
}

/// Reads what is typed in answer to a `Demand`: an expression, with or
/// without a full stop after it.
pub fn answer(text: &str) -> Result<Expr, Eh> {
    let tokens = lex(text);
    let mut p = Parser {
        tokens: &tokens,
        at: 0,
        chars: text.chars().collect(),
        ranging: false,
    };
    let value = p.expr()?;
    if p.peek() == Some(&Tok::Period) {
        p.at += 1;
    }
    p.end()?;
    Ok(value)
}

/// Reads the text of a stored step (without its number).
pub fn step(text: &str) -> Result<Stmt, Eh> {
    let tokens = lex(text);
    if tokens.is_empty() {
        return Err(Eh { at: 0 });
    }
    Parser {
        tokens: &tokens,
        at: 0,
        chars: text.chars().collect(),
        ranging: false,
    }
    .stmt()
}

struct Parser<'a> {
    tokens: &'a [Token],
    at: usize,
    chars: Vec<char>,
    /// Reading the values of a `for`, where `a(b)c` is a run from a in steps
    /// of b to c, and not the element b of a followed by c.
    ranging: bool,
}

impl Parser<'_> {
    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.at).map(|t| &t.tok)
    }

    /// The column of the token being looked at.
    fn column(&self) -> usize {
        match self.tokens.get(self.at) {
            Some(token) => token.start,
            None => self.chars.len(),
        }
    }

    fn eh<T>(&self) -> Result<T, Eh> {
        Err(Eh { at: self.column() })
    }

    fn expect(&mut self, tok: Tok) -> Result<(), Eh> {
        if self.peek() == Some(&tok) {
            self.at += 1;
            Ok(())
        } else {
            self.eh()
        }
    }

    fn end(&self) -> Result<(), Eh> {
        if self.at == self.tokens.len() {
            Ok(())
        } else {
            self.eh()
        }
    }

    /// Takes the word if it is next. A word must stand clear of what follows
    /// it: `Do part1.` is not JOSS.
    fn word(&mut self, word: &str) -> bool {
        let Some(Token {
            tok: Tok::Word(found), ..
        }) = self.tokens.get(self.at)
        else {
            return false;
        };
        if found != word {
            return false;
        }
        if let Some(next) = self.tokens.get(self.at + 1) {
            if !next.spaced && matches!(next.tok, Tok::Number(_) | Tok::Letter(_) | Tok::Word(_)) {
                return false;
            }
        }
        self.at += 1;
        true
    }

    fn is_word(&self, word: &str) -> bool {
        matches!(self.peek(), Some(Tok::Word(found)) if found == word)
    }

    fn numeral(&mut self) -> Result<Num, Eh> {
        match self.peek() {
            Some(Tok::Number(digits)) => {
                let number = Num::numeral(digits);
                match number {
                    Some(n) => {
                        self.at += 1;
                        Ok(n)
                    }
                    None => self.eh(),
                }
            }
            _ => self.eh(),
        }
    }

    fn stmt(&mut self) -> Result<Stmt, Eh> {
        let verb = self.verb()?;
        let cond = if self.word("if") { Some(self.cond()?) } else { None };
        self.expect(Tok::Period)?;
        self.end()?;
        Ok(Stmt { verb, cond })
    }

    fn unit(&mut self) -> Result<Unit, Eh> {
        if self.word("step") {
            Ok(Unit::Step)
        } else if self.word("part") {
            Ok(Unit::Part)
        } else {
            self.eh()
        }
    }

    fn kind(&mut self) -> Option<Kind> {
        for (word, kind) in [
            ("steps", Kind::Steps),
            ("parts", Kind::Parts),
            ("forms", Kind::Forms),
            ("values", Kind::Values),
        ] {
            if self.word(word) {
                return Some(kind);
            }
        }
        None
    }

    fn verb(&mut self) -> Result<Verb, Eh> {
        if self.word("Set") {
            let place = self.place()?;
            self.expect(Tok::Eq)?;
            return Ok(Verb::Set(place, self.expr()?));
        }
        if self.word("Do") {
            let unit = self.unit()?;
            let number = self.expr()?;
            let iteration = if self.word("for") {
                Some(self.iteration()?)
            } else {
                None
            };
            return Ok(Verb::Do(unit, number, iteration));
        }
        if self.word("Type") {
            return self.type_verb();
        }
        if self.word("Delete") {
            if self.word("step") {
                return Ok(Verb::DeleteStep(self.expr()?));
            }
            if self.word("part") {
                return Ok(Verb::DeletePart(self.expr()?));
            }
            if self.word("form") {
                return Ok(Verb::DeleteForm(self.expr()?));
            }
            if self.word("all") {
                return Ok(Verb::DeleteAll(self.kind()));
            }
            let mut places = vec![self.place()?];
            while self.peek() == Some(&Tok::Comma) {
                self.at += 1;
                places.push(self.place()?);
            }
            return Ok(Verb::DeleteValues(places));
        }
        if self.word("To") {
            let unit = self.unit()?;
            return Ok(Verb::To(unit, self.expr()?));
        }
        if self.word("Demand") {
            let from = self.column();
            let place = self.place()?;
            let label: String = self.chars[from..self.tokens[self.at - 1].end].iter().collect();
            return Ok(Verb::Demand(place, label));
        }
        for (word, verb) in [
            ("Line", Verb::Line),
            ("Page", Verb::Page),
            ("Done", Verb::Done),
            ("Stop", Verb::Stop),
            ("Cancel", Verb::Cancel),
            ("Go", Verb::Go),
        ] {
            if self.word(word) {
                return Ok(verb);
            }
        }
        self.eh()
    }

    fn type_verb(&mut self) -> Result<Verb, Eh> {
        if let Some(Tok::Text(text)) = self.peek() {
            let text = text.clone();
            self.at += 1;
            return Ok(Verb::TypeText(text));
        }
        if self.word("step") {
            return Ok(Verb::TypeStep(self.expr()?));
        }
        if self.word("part") {
            return Ok(Verb::TypePart(self.expr()?));
        }
        if self.word("form") {
            return Ok(Verb::TypeForm(self.expr()?));
        }
        if self.word("all") {
            return Ok(Verb::TypeAll(self.kind()));
        }
        for (word, verb) in [
            ("size", Verb::TypeSize),
            ("time", Verb::TypeTime),
            ("users", Verb::TypeUsers),
        ] {
            if self.word(word) {
                return Ok(verb);
            }
        }
        let mut items = Vec::new();
        loop {
            if self.peek() == Some(&Tok::Blank) {
                self.at += 1;
                items.push(Item::Blank);
            } else {
                let from = self.column();
                let value = self.expr()?;
                let label: String = self.chars[from..self.tokens[self.at - 1].end].iter().collect();
                items.push(Item::Value(value, label));
            }
            if self.peek() == Some(&Tok::Comma) {
                self.at += 1;
            } else {
                break;
            }
        }
        if self.is_word("in") {
            self.at += 1;
            if !self.word("form") {
                return self.eh();
            }
            let form = self.expr()?;
            let mut values = Vec::new();
            for item in items {
                match item {
                    Item::Value(value, _) => values.push(value),
                    Item::Blank => return self.eh(),
                }
            }
            return Ok(Verb::TypeInForm(values, form));
        }
        Ok(Verb::Type(items))
    }

    fn iteration(&mut self) -> Result<For, Eh> {
        let place = self.place()?;
        self.expect(Tok::Eq)?;
        self.ranging = true;
        let mut runs = Vec::new();
        loop {
            let first = self.expr()?;
            let mut legs = Vec::new();
            while matches!(self.peek(), Some(Tok::Open(_))) {
                let open = self.at;
                self.at += 1;
                let by = self.expr()?;
                self.close(open)?;
                legs.push((by, self.expr()?));
            }
            runs.push(if legs.is_empty() {
                Run::Value(first)
            } else {
                Run::Range(first, legs)
            });
            if self.peek() == Some(&Tok::Comma) {
                self.at += 1;
            } else {
                break;
            }
        }
        self.ranging = false;
        Ok(For { place, runs })
    }

    /// The bracket that closes the one opened at token `open`: `(` with `)`,
    /// `[` with `]`.
    fn close(&mut self, open: usize) -> Result<(), Eh> {
        let want = if self.tokens[open].tok == Tok::Open('(') {
            ')'
        } else {
            ']'
        };
        self.expect(Tok::Close(want))
    }

    fn place(&mut self) -> Result<Place, Eh> {
        let Some(Tok::Letter(letter)) = self.peek().cloned() else {
            return self.eh();
        };
        let at = self.column();
        self.at += 1;
        let mut indices = Vec::new();
        if matches!(self.peek(), Some(Tok::Open(_))) && !self.tokens[self.at].spaced {
            let open = self.at;
            self.at += 1;
            indices = self.list()?;
            self.close(open)?;
            // In the values of a `for`, a bracket that is followed straight
            // away by another value was not an index: it was the step of a
            // run, `a(b)c`. Leave it for the run to read.
            let value_next = matches!(
                self.peek(),
                Some(Tok::Number(_) | Tok::Letter(_) | Tok::Word(_) | Tok::Open(_) | Tok::Bar | Tok::Dollar)
            ) && !self.is_word("if");
            if self.ranging && value_next {
                self.at = open;
                indices = Vec::new();
            }
        }
        Ok(Place { letter, indices, at })
    }

    fn list(&mut self) -> Result<Vec<Expr>, Eh> {
        let mut items = vec![self.expr()?];
        while self.peek() == Some(&Tok::Comma) {
            self.at += 1;
            items.push(self.expr()?);
        }
        Ok(items)
    }

    /// "The 'and' and 'or' are evaluated from left to right" (JOSS program
    /// notes, p. 2): neither binds tighter than the other.
    fn cond(&mut self) -> Result<Cond, Eh> {
        let mut left = self.chain()?;
        loop {
            if self.word("or") {
                left = Cond::Or(Box::new(left), Box::new(self.chain()?));
            } else if self.word("and") {
                left = Cond::And(Box::new(left), Box::new(self.chain()?));
            } else {
                return Ok(left);
            }
        }
    }

    fn rel(&self) -> Option<Rel> {
        Some(match self.peek()? {
            Tok::Eq => Rel::Eq,
            Tok::Ne => Rel::Ne,
            Tok::Le => Rel::Le,
            Tok::Ge => Rel::Ge,
            Tok::Lt => Rel::Lt,
            Tok::Gt => Rel::Gt,
            _ => return None,
        })
    }

    fn chain(&mut self) -> Result<Cond, Eh> {
        let first = self.expr()?;
        let mut rest = Vec::new();
        while let Some(rel) = self.rel() {
            self.at += 1;
            rest.push((rel, self.expr()?));
        }
        if rest.is_empty() {
            return self.eh();
        }
        Ok(Cond::Chain(first, rest))
    }

    /// Sums and differences. A sign in front applies to the first term:
    /// `-x*2/2` is minus (x squared over two).
    fn expr(&mut self) -> Result<Expr, Eh> {
        let mut left = match self.peek() {
            Some(Tok::Minus) => {
                self.at += 1;
                Expr::Neg(Box::new(self.term()?))
            }
            Some(Tok::Plus) => {
                self.at += 1;
                self.term()?
            }
            _ => self.term()?,
        };
        loop {
            let op = match self.peek() {
                Some(Tok::Plus) => BinOp::Add,
                Some(Tok::Minus) => BinOp::Sub,
                _ => return Ok(left),
            };
            self.at += 1;
            left = Expr::Bin(op, Box::new(left), Box::new(self.term()?));
        }
    }

    fn term(&mut self) -> Result<Expr, Eh> {
        let mut left = self.factor()?;
        loop {
            let op = match self.peek() {
                Some(Tok::Times) => BinOp::Mul,
                Some(Tok::Over) => BinOp::Div,
                _ => return Ok(left),
            };
            self.at += 1;
            left = Expr::Bin(op, Box::new(left), Box::new(self.factor()?));
        }
    }

    fn factor(&mut self) -> Result<Expr, Eh> {
        let mut left = self.primary()?;
        while self.peek() == Some(&Tok::Power) {
            self.at += 1;
            // A negative power goes in brackets, as JOSS types it: 10*(-3).
            let right = self.primary()?;
            left = Expr::Bin(BinOp::Pow, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn primary(&mut self) -> Result<Expr, Eh> {
        match self.peek().cloned() {
            Some(Tok::Number(digits)) => {
                if Num::significant(&digits) > 9 {
                    self.at += 1;
                    return Ok(Expr::TooLong);
                }
                Ok(Expr::Num(self.numeral()?))
            }
            Some(Tok::Dollar) => {
                self.at += 1;
                Ok(Expr::Dollar)
            }
            Some(Tok::Letter(_)) => Ok(Expr::Var(self.place()?)),
            Some(Tok::Word(name)) => {
                let Some(func) = Func::named(&name) else {
                    return self.eh();
                };
                self.at += 1;
                if !matches!(self.peek(), Some(Tok::Open(_))) {
                    return self.eh();
                }
                let open = self.at;
                self.at += 1;
                let args = self.list()?;
                self.close(open)?;
                let wanted = match func {
                    Func::Arg => args.len() == 2,
                    Func::Max | Func::Min => args.len() >= 2,
                    _ => args.len() == 1,
                };
                if !wanted {
                    return Err(Eh {
                        at: self.tokens[open].start,
                    });
                }
                Ok(Expr::Call(func, args))
            }
            Some(Tok::Open(_)) => {
                let open = self.at;
                self.at += 1;
                let inner = self.expr()?;
                self.close(open)?;
                Ok(inner)
            }
            Some(Tok::Bar) => {
                self.at += 1;
                let inner = self.expr()?;
                self.expect(Tok::Bar)?;
                Ok(Expr::Abs(Box::new(inner)))
            }
            _ => self.eh(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_is_direct_or_a_step_or_a_form() {
        assert!(matches!(
            line("Set x=3."),
            Ok(Line::Direct(Stmt {
                verb: Verb::Set(..),
                cond: None
            }))
        ));
        assert!(matches!(line("1.35 Set i=i+1."), Ok(Line::Step { text, .. }) if text == "Set i=i+1."));
        assert!(matches!(line("Form 1:"), Ok(Line::Form(_))));
        assert!(matches!(line("Form n:"), Ok(Line::Form(Expr::Var(_)))));
        assert!(matches!(line("Form 1+1:"), Ok(Line::Form(Expr::Bin(..)))));
        assert!(matches!(line("x=3"), Ok(Line::ShortSet(..))));
        assert_eq!(line("* PRIME NUMBERS"), Ok(Line::Killed));
        assert_eq!(line("Set x=3. *"), Ok(Line::Killed));
        assert_eq!(line("   "), Ok(Line::Empty));
    }

    #[test]
    fn what_is_not_joss_is_eh() {
        // From figure 2 of P-3149.
        assert!(line("Do part1.1.").is_err());
        assert!(line("Do part 1.1.").is_ok()); // read; it is the part number that is wrong
        assert!(line("Set x=3").is_err());
        assert!(line("set x=3.").is_err());
        assert!(line("Type x").is_err());
        assert!(line("Type (x.").is_err());
        assert!(line("Type (x].").is_err());
    }

    #[test]
    fn steps_conditions_and_iterations() {
        let s = step("Do part 2 for d = 2(1)ip[sqrt(x)] if x > 3.").unwrap();
        assert!(
            matches!(&s.verb, Verb::Do(Unit::Part, _, Some(For { runs, .. })) if matches!(&runs[0], Run::Range(_, legs) if legs.len() == 1))
        );
        assert!(s.cond.is_some());
        let s = step("Do step 3.1 for x = 8.50(.01)8.54(.02)8.60, 9, 9.5.").unwrap();
        assert!(matches!(&s.verb, Verb::Do(Unit::Step, _, Some(For { runs, .. })) if runs.len() == 3));
        // a(b)c is a run even when a is a letter; a(1) alone is an element.
        let s = step("Do part 1 for x = a(b)c(d)e, f, g(2).").unwrap();
        assert!(
            matches!(&s.verb, Verb::Do(_, _, Some(For { runs, .. })) if runs.len() == 3
            && matches!(&runs[0], Run::Range(Expr::Var(a), legs) if a.indices.is_empty() && legs.len() == 2)
            && matches!(&runs[2], Run::Value(Expr::Var(g)) if g.indices.len() == 1))
        );
        assert!(step("Type 10*-3.").is_err());
        let s = step("Type x if a<b<c and d=e or f≠g.").unwrap();
        assert!(matches!(s.cond, Some(Cond::Or(..))));
        // Left to right: (a<b or c=d) and e≤x≤f.
        let s = step("Type x if a<b or c=d and e≤x≤f.").unwrap();
        assert!(matches!(s.cond, Some(Cond::And(left, _)) if matches!(*left, Cond::Or(..))));
        let s = step("Type x, sqrt(x), log(x), exp(x), __.").unwrap();
        assert!(
            matches!(&s.verb, Verb::Type(items) if items.len() == 5 && items[4] == Item::Blank && matches!(&items[1], Item::Value(_, label) if label == "sqrt(x)"))
        );
        assert!(
            matches!(step("Type a, b, c in form 1 if fp(c)=0.").unwrap().verb, Verb::TypeInForm(values, _) if values.len() == 3)
        );
        assert!(matches!(step("Type \"ABCDE\".").unwrap().verb, Verb::TypeText(text) if text == "ABCDE"));
        assert!(
            matches!(step("Set x(1) = [-b+sqrt(b*2-4·a·c)] / [2·a].").unwrap().verb, Verb::Set(place, _) if place.indices.len() == 1)
        );
    }
}
