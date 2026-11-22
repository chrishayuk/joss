//! A sitting at a JOSS console.
//!
//! JOSS is a conversation. The user types a line; JOSS does what it says, or
//! keeps it as a step of a stored program, or says what is wrong with it; and
//! then it is the user's turn again. The state between turns is the point of
//! the system: the steps, the forms and the values stay, and so does a task
//! that has been broken off. "If the user specifies a process imperfectly,
//! JOSS will call it to his attention with an error message and stand by to
//! help with the changes. Frequently the user can direct JOSS to make the
//! correction and continue without having to start over" (P-3149, p. 5).
//!
//! So a task here is not a call stack of this program's. It is a list of
//! frames kept in the session, each naming the step it is at, and `Go.` picks
//! it up where it stopped.

use crate::format::{self, FormFault};
use crate::messages::{self as say, Message, Trouble};
use crate::num::{Fault, Num};
use crate::parse::{self, BinOp, Cond, Expr, Func, Item, Kind, Line, Place, Rel, Run, Stmt, Unit, Verb};
use std::collections::BTreeMap;

/// Lines on a page: the paper is thrown when the count reaches 55 (the
/// listing's "LINE NUMBER FOR EJECT").
pub const PAGE_LINES: u32 = 54;
/// The line the typewriter is on after the heading of a new page: the line
/// with the time, the date and the user's initials, and the blank lines
/// under it.
pub const FIRST_LINE: u32 = 7;

/// Something JOSS typed.
#[derive(Clone, Debug, PartialEq)]
pub enum Out {
    /// A whole line.
    Line(String),
    /// The start of a line, after which JOSS hands the typewriter back for
    /// the user to finish it: `x(1) = ` from a `Demand`.
    Ask(String),
    /// The paper moved to the top of a new page.
    Page,
}

/// Which JOSS. The language did not change between these; what JOSS said did.
///
/// "After a year's experience with neutral, impersonal messages, we
/// introduced personal pronouns and some informal diction." (P-3149, p. 6)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Year {
    /// The JOSS of the film made in March 1964: "Error above:  Zero divisor."
    Y1964,
    /// The JOSS of early 1965: "I have a zero divisor."
    #[default]
    Y1965,
}

/// What the console is waiting for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Waiting {
    /// An instruction, a step or a form heading.
    Instruction,
    /// The line of a form, `Form n:` having just been typed.
    Form,
    /// A value, for a `Demand`.
    Value,
}

#[derive(Clone, Debug)]
struct Step {
    /// The number as typed (`1.35`), for typing back.
    label: String,
    /// The text after the number, as typed.
    text: String,
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    One(Num),
    /// The elements of a vector or array that have been given values. "It
    /// need not be dense, since values are looked up by scanning the list for
    /// a match on the explicit index." (P-2922, p. 30)
    Many(BTreeMap<Vec<i64>, Num>),
}

/// The values a `for` clause runs through, worked out when the `Do` is met.
#[derive(Clone, Debug)]
struct Iteration {
    letter: char,
    indices: Vec<i64>,
    /// Each run is a start and then (step, limit) pairs; a single value is a
    /// start with no pairs.
    runs: Vec<(Num, Vec<(Num, Num)>)>,
    run: usize,
    leg: usize,
    value: Num,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    /// About to begin the unit (again, if iterating).
    Begin,
    /// About to do the step at `at`.
    Do,
    /// The step at `at` is done; move on.
    Next,
}

#[derive(Clone, Debug)]
struct Frame {
    /// The part being done, or `None` when a single step is being done.
    part: Option<i64>,
    /// Where to begin: the step named by `Do step`, or the first of the part.
    first: Num,
    at: Num,
    phase: Phase,
    iteration: Option<Iteration>,
}

/// How a broken-off task can be taken up again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Resume {
    /// Running now.
    No,
    /// With `Go.`
    Go,
    /// With the value a `Demand` is waiting for.
    Value,
}

#[derive(Clone, Debug)]
struct Task {
    frames: Vec<Frame>,
    resume: Resume,
}

/// What doing an instruction leads to.
enum Flow {
    Next,
    Call(Frame),
    To(Unit, Num),
    Done,
    Stop,
    Demand(char, Vec<i64>),
    /// `Go.`
    Go,
    /// `Cancel.`
    Cancel,
}

pub struct Session {
    steps: BTreeMap<Num, Step>,
    forms: BTreeMap<i64, String>,
    values: BTreeMap<char, Value>,
    task: Option<Task>,
    pending_form: Option<i64>,
    out: Vec<Out>,
    /// The line of the page the typewriter is on, from 1.
    line: u32,
    /// Steps JOSS may do in answer to one line before the console's
    /// interrupt key is pressed for it. `None` is no limit.
    pub patience: Option<u64>,
    fuel: u64,
    /// Lines JOSS may type in answer to the next line fed before the
    /// interrupt key is pressed. Set it before `feed`; it is used once.
    pub interrupt_after: Option<usize>,
    interrupt_at: Option<usize>,
    typed: usize,
    /// Words of storage in the user's block.
    pub storage: usize,
    pub year: Year,
    /// What `Type time.` answers: the time of day as JOSS wrote it, 1230
    /// for half past twelve.
    pub time: u32,
    /// What `Type users.` answers.
    pub users: u32,
}

impl Default for Session {
    fn default() -> Self {
        Session::new()
    }
}

type Outcome<T> = Result<T, Trouble>;

fn element_label(letter: char, indices: &[i64]) -> String {
    if indices.is_empty() {
        letter.to_string()
    } else {
        let list: Vec<String> = indices.iter().map(|i| i.to_string()).collect();
        format!("{letter}({})", list.join(","))
    }
}

/// Small letters before capitals, each in alphabetical order (the order of
/// the values in every `Type all` in the papers).
fn letter_order(c: char) -> (bool, char) {
    (c.is_ascii_uppercase(), c)
}

fn fault(fault: Fault) -> Trouble {
    Trouble::Said(match fault {
        Fault::Overflow => Message::Overflow,
        Fault::ZeroDivisor => Message::ZeroDivisor,
        Fault::NegativeSqrt => Message::NegativeSqrt,
        Fault::BadLog => Message::BadLog,
        Fault::NegativeBase => Message::NegativeBase,
        Fault::ZeroToNegative => Message::ZeroToNegative,
        Fault::BadAngle => Message::BadAngle,
        Fault::BadArg => Message::ArgOfOrigin,
    })
}

impl Session {
    pub fn new() -> Session {
        Session {
            steps: BTreeMap::new(),
            forms: BTreeMap::new(),
            values: BTreeMap::new(),
            task: None,
            pending_form: None,
            out: Vec::new(),
            line: FIRST_LINE,
            patience: None,
            fuel: 0,
            interrupt_after: None,
            interrupt_at: None,
            typed: 0,
            storage: say::STORAGE_WORDS,
            year: Year::default(),
            time: 1000,
            users: 1,
        }
    }

    pub fn waiting(&self) -> Waiting {
        if self.pending_form.is_some() {
            Waiting::Form
        } else if matches!(&self.task, Some(task) if task.resume == Resume::Value) {
            Waiting::Value
        } else {
            Waiting::Instruction
        }
    }

    /// The line of the page the typewriter is on: the value of `$`.
    pub fn line_number(&self) -> u32 {
        self.line
    }

    /// Whether a task has been broken off and is waiting for `Go.`
    pub fn suspended(&self) -> bool {
        matches!(&self.task, Some(task) if task.resume == Resume::Go)
    }

    /// Gives JOSS the line the user has typed and returns what JOSS types
    /// in answer.
    pub fn feed(&mut self, typed: &str) -> Vec<Out> {
        self.out.clear();
        self.fuel = self.patience.unwrap_or(u64::MAX);
        self.interrupt_at = self.interrupt_after.take();
        self.typed = 0;
        let typed = crate::chars::canonical(typed, self.pending_form.is_some());
        // The user's own line takes a line of the page.
        self.carriage_return();
        match self.waiting() {
            Waiting::Form => {
                let number = self.pending_form.take().unwrap_or(0);
                let had = self.forms.get(&number).map(|old| say::text_words(old)).unwrap_or(0);
                if self.used() + say::text_words(&typed) > self.storage + had {
                    self.type_line(Message::MoreStorage.text(self.year));
                } else {
                    self.forms.insert(number, typed.trim_end().to_string());
                }
            }
            Waiting::Value => self.answer(&typed),
            Waiting::Instruction => self.instruction(&typed),
        }
        std::mem::take(&mut self.out)
    }

    fn carriage_return(&mut self) {
        self.line += 1;
        if self.line > PAGE_LINES {
            self.new_page();
        }
    }

    fn new_page(&mut self) {
        self.line = FIRST_LINE;
        self.out.push(Out::Page);
    }

    fn type_line(&mut self, text: impl Into<String>) {
        self.out.push(Out::Line(text.into()));
        self.typed += 1;
        self.carriage_return();
    }

    /// Types lines for a step of a task, unless the interrupt key is
    /// pressed first. JOSS breaks off between lines, and the step is done
    /// again from its beginning when the user says `Go.`
    fn type_lines(&mut self, lines: Vec<String>, at: Option<Num>) -> Outcome<()> {
        for line in lines {
            if at.is_some() && self.interrupt_at.is_some_and(|limit| self.typed >= limit) {
                return Err(Trouble::Interrupt);
            }
            self.type_line(line);
        }
        Ok(())
    }

    /// Reports trouble with a line given directly. "Eh?" and the want of
    /// storage are typed bare; everything else is an "Error above".
    fn complain_above(&mut self, trouble: Trouble) {
        let text = trouble.text(self.year);
        match trouble {
            Trouble::Said(Message::Eh | Message::MoreStorage) => self.type_line(text),
            Trouble::Interrupt => {}
            _ => self.type_line(say::error_above(&text)),
        }
    }

    fn instruction(&mut self, typed: &str) {
        match parse::line(typed) {
            Err(_) => self.complain_above(Message::Eh.into()),
            Ok(Line::Empty | Line::Killed) => {}
            Ok(Line::Step { number, label, text }) => {
                if let Err(trouble) = self.keep_step(number, &label, text) {
                    self.complain_above(trouble);
                }
            }
            Ok(Line::Form(number)) => match self.eval(&number) {
                Ok(value) => match value.to_i64().filter(|n| (1..1_000_000_000).contains(n)) {
                    Some(n) => self.pending_form = Some(n),
                    None => self.complain_above(Message::FormNumber.into()),
                },
                Err(trouble) => self.complain_above(trouble),
            },
            Ok(Line::ShortSet(place, value)) => self.direct(&Stmt {
                verb: Verb::Set(place, value),
                cond: None,
            }),
            Ok(Line::Direct(stmt)) => self.direct(&stmt),
        }
    }

    /// Keeps a step. Nothing of it is read but its number: "Statements that
    /// are to be stored internally are not scanned until actual execution
    /// time" (RM-4162-PR, p. 35).
    fn keep_step(&mut self, number: Num, label: &str, text: String) -> Outcome<()> {
        if label.chars().filter(|c| c.is_ascii_digit()).count() > 9 {
            return Err(Message::LongStepLabel.into());
        }
        if number < Num::int(1) || number.exponent() > 8 {
            return Err(Message::StepNumber.into());
        }
        if text.is_empty() {
            return Err(Message::Eh.into());
        }
        // `1.50` is step 1.5, and is typed back so.
        let label = if label.contains('.') {
            label.trim_end_matches('0').trim_end_matches('.')
        } else {
            label
        };
        let label = label.trim_start_matches('0').to_string();
        let had = self
            .steps
            .get(&number)
            .map(|old| say::text_words(&old.text))
            .unwrap_or(0);
        if self.used() + say::text_words(&text) > self.storage + had {
            return Err(Message::MoreStorage.into());
        }
        self.steps.insert(number, Step { label, text });
        Ok(())
    }

    /// Words of the user's block in use: what `Type size.` answers.
    pub fn used(&self) -> usize {
        let steps: usize = self.steps.values().map(|s| say::text_words(&s.text)).sum();
        let forms: usize = self.forms.values().map(|f| say::text_words(f)).sum();
        let values: usize = self
            .values
            .values()
            .map(|v| match v {
                Value::One(_) => say::VALUE_WORDS,
                Value::Many(elements) => elements.len() * say::ELEMENT_WORDS,
            })
            .sum();
        let frames = self.task.as_ref().map(|t| t.frames.len()).unwrap_or(0) * say::FRAME_WORDS;
        steps + forms + values + frames
    }

    /// Carries out an instruction given directly.
    fn direct(&mut self, stmt: &Stmt) {
        if stmt.verb.indirect_only() {
            return self.complain_above(Message::NotDirectly.into());
        }
        let flow = match stmt.cond.as_ref().map(|cond| self.holds(cond)) {
            Some(Ok(false)) => return,
            Some(Err(trouble)) => Err(trouble),
            _ => self.perform(&stmt.verb, None),
        };
        match flow {
            Err(trouble) => self.complain_above(trouble),
            Ok(Flow::Call(frame)) => {
                // "Only one suspended task is allowed and that may be
                // continued by Go or cancelled by Cancel or Do."
                self.task = None;
                if self.used() + say::FRAME_WORDS > self.storage {
                    return self.complain_above(Message::MoreStorage.into());
                }
                self.task = Some(Task {
                    frames: vec![frame],
                    resume: Resume::No,
                });
                self.run();
            }
            Ok(Flow::Go) => match &mut self.task {
                Some(task) if task.resume == Resume::Go => {
                    task.resume = Resume::No;
                    self.run();
                }
                _ => self.complain_above(Message::NothingToDo.into()),
            },
            Ok(Flow::Cancel) => self.task = None,
            Ok(_) => {}
        }
    }

    /// The answer to a `Demand`: an expression. If it will not do, that is
    /// an error at the step, and `Go.` asks again.
    fn answer(&mut self, typed: &str) {
        let Some(at) = self.task.as_ref().and_then(|t| t.frames.last()).map(|f| f.at) else {
            return;
        };
        let demand = self.steps.get(&at).and_then(|step| parse::step(&step.text).ok());
        let value = match parse::answer(typed) {
            Ok(value) => self.eval(&value),
            Err(_) => Err(Message::Eh.into()),
        };
        let stored = match (demand, value) {
            (
                Some(Stmt {
                    verb: Verb::Demand(place, _),
                    ..
                }),
                Ok(value),
            ) => self
                .locate(&place)
                .and_then(|(letter, indices)| self.assign(letter, &indices, value)),
            (_, Err(trouble)) => Err(trouble),
            // The step is no longer a Demand: nothing to give the value to.
            (_, Ok(_)) => Ok(()),
        };
        match stored {
            Ok(()) => {
                if let Some(task) = &mut self.task {
                    task.resume = Resume::No;
                    if let Some(frame) = task.frames.last_mut() {
                        frame.phase = Phase::Next;
                    }
                }
                self.run();
            }
            Err(trouble) => {
                if let Some(task) = &mut self.task {
                    task.resume = Resume::Go;
                }
                let text = trouble.text(self.year);
                self.type_line(say::error_at_step(&format::plain(at), &text));
            }
        }
    }

    /// Does steps until the task ends or is broken off.
    fn run(&mut self) {
        loop {
            let Some(task) = &mut self.task else { return };
            let Some(frame) = task.frames.last_mut() else {
                self.task = None;
                return;
            };
            match frame.phase {
                Phase::Begin => {
                    frame.at = frame.first;
                    frame.phase = Phase::Do;
                    let part = frame.part;
                    let first = frame.first;
                    let value = frame.iteration.as_ref().map(|i| (i.letter, i.indices.clone(), i.value));
                    // The part or step may have been deleted since the Do.
                    let found = match part {
                        Some(p) => self.part_steps(p).next().map(|(n, _)| *n),
                        None => self.steps.contains_key(&first).then_some(first),
                    };
                    match found {
                        Some(n) => self.top().at = n,
                        None => {
                            let lost = if part.is_some() {
                                Message::NoPartForIteration
                            } else {
                                Message::NoStepForIteration
                            };
                            return self.during(lost);
                        }
                    }
                    if let Some((letter, indices, value)) = value {
                        self.store(letter, &indices, value);
                    }
                }
                Phase::Do => {
                    let at = frame.at;
                    if self.fuel == 0 || self.interrupt_at.is_some_and(|limit| self.typed >= limit) {
                        return self.interrupt(at);
                    }
                    self.fuel -= 1;
                    // A step deleted while the task was broken off is passed over.
                    let Some(step) = self.steps.get(&at) else {
                        frame.phase = Phase::Next;
                        continue;
                    };
                    let text = step.text.clone();
                    match self.do_step(&text, at) {
                        Ok(false) => {}
                        Ok(true) => return,
                        Err(Trouble::Interrupt) => return self.interrupt(at),
                        Err(trouble) => {
                            if let Some(task) = &mut self.task {
                                task.resume = Resume::Go;
                            }
                            let text = trouble.text(self.year);
                            // Running out of room is typed bare.
                            if trouble == Trouble::Said(Message::MoreStorage) {
                                return self.type_line(text);
                            }
                            return self.type_line(say::error_at_step(&format::plain(at), &text));
                        }
                    }
                }
                Phase::Next => {
                    let (part, at) = (frame.part, frame.at);
                    let next = part.and_then(|p| {
                        self.steps
                            .range((std::ops::Bound::Excluded(at), std::ops::Bound::Unbounded))
                            .next()
                            .map(|(n, _)| *n)
                            .filter(|n| n.ip().to_i64() == Some(p))
                    });
                    match next {
                        Some(n) => {
                            frame.at = n;
                            frame.phase = Phase::Do;
                        }
                        None => self.finish(),
                    }
                }
            }
        }
    }

    fn interrupt(&mut self, at: Num) {
        self.interrupt_at = None;
        if let Some(task) = &mut self.task {
            task.resume = Resume::Go;
        }
        let line = say::interrupted(&format::plain(at), self.year);
        self.type_line(line);
    }

    /// Trouble in going round a `for` again: the step or part it repeats
    /// has gone. It is reported against the step holding the `Do`. If that
    /// has gone too, there is nothing left to point at: "It's a mess."
    fn during(&mut self, lost: Message) {
        let Some(task) = &mut self.task else { return };
        task.resume = Resume::Go;
        task.frames.last_mut().expect("a frame is being begun").phase = Phase::Begin;
        let text = lost.text(self.year);
        let caller = task.frames.iter().rev().nth(1).map(|frame| frame.at);
        match caller {
            None => {
                self.task = None;
                self.type_line(say::error_above(text));
            }
            Some(at) if self.steps.contains_key(&at) => {
                self.type_line(say::error_during_step(&format::plain(at), text))
            }
            Some(_) => {
                self.task = None;
                self.type_line(say::MESS);
            }
        }
    }

    /// The unit of the top frame has been done once. Do it again for the
    /// next value, or go back to whoever asked for it.
    fn finish(&mut self) {
        let Some(task) = &mut self.task else { return };
        let Some(frame) = task.frames.last_mut() else { return };
        if let Some(iteration) = &mut frame.iteration {
            if iteration.advance() {
                frame.phase = Phase::Begin;
                return;
            }
        }
        task.frames.pop();
        match task.frames.last_mut() {
            Some(caller) => caller.phase = Phase::Next,
            None => self.task = None,
        }
    }

    /// Does one step of the stored program. `Ok(true)` means the task has
    /// been broken off and the typewriter goes back to the user.
    fn do_step(&mut self, text: &str, at: Num) -> Outcome<bool> {
        let stmt = parse::step(text).map_err(|_| Trouble::Said(Message::Eh))?;
        if stmt.verb.direct_only() {
            return Err(Message::NotIndirectly.into());
        }
        if let Some(cond) = &stmt.cond {
            if !self.holds(cond)? {
                self.top().phase = Phase::Next;
                return Ok(false);
            }
        }
        match self.perform(&stmt.verb, Some(at))? {
            Flow::Next | Flow::Go | Flow::Cancel => self.top().phase = Phase::Next,
            Flow::Call(frame) => {
                if self.used() + say::FRAME_WORDS > self.storage {
                    return Err(Message::MoreStorage.into());
                }
                self.top().phase = Phase::Next;
                if let Some(task) = &mut self.task {
                    task.frames.push(frame);
                }
            }
            Flow::To(unit, number) => {
                let target = match unit {
                    Unit::Step => self.existing_step(number)?,
                    Unit::Part => self.first_of_part(number)?,
                };
                // The sequence goes on from there: in that part, if it is
                // another part, and back to whoever asked when it ends.
                let frame = self.top();
                frame.part = target.ip().to_i64();
                frame.at = target;
                frame.phase = Phase::Do;
            }
            Flow::Done => {
                // The rest of the part is skipped; an iteration goes on.
                self.finish();
            }
            Flow::Stop => {
                self.top().phase = Phase::Next;
                if let Some(task) = &mut self.task {
                    task.resume = Resume::Go;
                }
                self.type_line(say::stopped(&format::plain(at)));
                return Ok(true);
            }
            Flow::Demand(letter, indices) => {
                if let Some(task) = &mut self.task {
                    task.resume = Resume::Value;
                }
                self.out.push(Out::Ask(format!(
                    "{} ",
                    format::label(&element_label(letter, &indices))
                )));
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn top(&mut self) -> &mut Frame {
        self.task
            .as_mut()
            .and_then(|t| t.frames.last_mut())
            .expect("a step is done inside a task")
    }

    fn existing_step(&self, number: Num) -> Outcome<Num> {
        if self.steps.contains_key(&number) {
            Ok(number)
        } else if number < Num::int(1) || number.exponent() > 8 {
            Err(Message::StepNumber.into())
        } else {
            Err(Message::NoStep.into())
        }
    }

    fn part_number(&self, number: Num) -> Outcome<i64> {
        number
            .to_i64()
            .filter(|p| (1..1_000_000_000).contains(p))
            .ok_or(Message::PartNumber.into())
    }

    fn form_number(&self, number: Num) -> Outcome<i64> {
        number
            .to_i64()
            .filter(|p| (1..1_000_000_000).contains(p))
            .ok_or(Message::FormNumber.into())
    }

    fn part_steps(&self, part: i64) -> impl Iterator<Item = (&Num, &Step)> {
        self.steps
            .range(Num::int(part)..)
            .take_while(move |(n, _)| n.ip().to_i64() == Some(part))
    }

    fn first_of_part(&self, number: Num) -> Outcome<Num> {
        let part = self.part_number(number)?;
        self.part_steps(part)
            .next()
            .map(|(n, _)| *n)
            .ok_or(Message::NoPart.into())
    }

    fn form(&self, number: Num) -> Outcome<String> {
        let n = self.form_number(number)?;
        self.forms.get(&n).cloned().ok_or(Message::NoForm.into())
    }

    /// Carries out the verb. `at` is the step it is part of, if it is one.
    fn perform(&mut self, verb: &Verb, at: Option<Num>) -> Outcome<Flow> {
        match verb {
            Verb::Set(place, value) => {
                let value = self.eval(value)?;
                let (letter, indices) = self.locate(place)?;
                self.assign(letter, &indices, value)?;
            }
            Verb::Do(unit, number, iteration) => {
                let number = self.eval(number)?;
                let (part, first) = match unit {
                    Unit::Step => (None, self.existing_step(number)?),
                    Unit::Part => {
                        let first = self.first_of_part(number)?;
                        (first.ip().to_i64(), first)
                    }
                };
                let iteration = match iteration {
                    Some(clause) => Some(self.iteration(clause)?),
                    None => None,
                };
                return Ok(Flow::Call(Frame {
                    part,
                    first,
                    at: first,
                    phase: Phase::Begin,
                    iteration,
                }));
            }
            Verb::Type(items) => {
                // Everything is worked out before anything is typed, so an
                // error leaves the page as it was.
                let mut lines = Vec::new();
                for item in items {
                    match item {
                        Item::Blank => lines.push(String::new()),
                        Item::Value(Expr::Var(place), label) if place.indices.is_empty() => {
                            match self.values.get(&place.letter) {
                                // A letter that has elements: all of them.
                                Some(Value::Many(elements)) => {
                                    for (indices, value) in elements {
                                        lines.push(format::value_line(
                                            &element_label(place.letter, indices),
                                            *value,
                                            self.year,
                                        ));
                                    }
                                }
                                Some(Value::One(value)) => lines.push(format::value_line(label, *value, self.year)),
                                None => return Err(Trouble::Undefined(label.clone())),
                            }
                        }
                        Item::Value(value, label) => {
                            lines.push(format::value_line(label, self.eval(value)?, self.year))
                        }
                    }
                }
                self.type_lines(lines, at)?;
            }
            Verb::TypeInForm(values, form) => {
                let number = self.eval(form)?;
                let form = self.form(number)?;
                let mut numbers = Vec::new();
                for value in values {
                    if let Expr::Var(place) = value {
                        if place.indices.is_empty() && matches!(self.values.get(&place.letter), Some(Value::Many(_))) {
                            return Err(Message::IndividualValues.into());
                        }
                    }
                    numbers.push(self.eval(value)?);
                }
                let line = format::fill(&form, &numbers).map_err(|fault| {
                    Trouble::Said(match fault {
                        FormFault::FieldsRunTogether => Message::FieldsRunTogether,
                        FormFault::TooFewValues => Message::TooFewValues,
                        FormFault::TooManyValues => Message::TooManyValues,
                        FormFault::CantExpress => Message::CantExpress,
                        FormFault::ShortField => Message::ShortField,
                    })
                })?;
                self.type_lines(vec![line], at)?;
            }
            Verb::TypeText(text) => self.type_lines(vec![text.clone()], at)?,
            Verb::TypeStep(number) => {
                let number = self.eval(number)?;
                let number = self.existing_step(number)?;
                let step = &self.steps[&number];
                let line = format!("{} {}", step.label, step.text);
                self.type_line(line);
            }
            Verb::TypePart(number) => {
                let number = self.eval(number)?;
                let part = self.part_number(number)?;
                let lines: Vec<String> = self
                    .part_steps(part)
                    .map(|(_, s)| format!("{} {}", s.label, s.text))
                    .collect();
                if lines.is_empty() {
                    return Err(Message::NoPart.into());
                }
                for line in lines {
                    self.type_line(line);
                }
            }
            Verb::TypeForm(number) => {
                // The form itself, without its heading: a way to type a
                // line of text, such as the heading of a table.
                let number = self.eval(number)?;
                let form = self.form(number)?;
                self.type_lines(vec![form], at)?;
            }
            Verb::TypeAll(kind) => self.type_all(*kind),
            Verb::TypeSize => self.type_line(self.used().to_string()),
            Verb::TypeTime => self.type_line(format!("{:04}", self.time)),
            Verb::TypeUsers => self.type_line(self.users.to_string()),
            Verb::DeleteValues(places) => {
                let mut located = Vec::new();
                for place in places {
                    located.push(self.locate(place)?);
                }
                for (letter, indices) in located {
                    if indices.is_empty() {
                        self.values.remove(&letter);
                    } else if let Some(Value::Many(elements)) = self.values.get_mut(&letter) {
                        elements.remove(&indices);
                        if elements.is_empty() {
                            self.values.remove(&letter);
                        }
                    }
                }
            }
            Verb::DeleteStep(number) => {
                let number = self.eval(number)?;
                let number = self.existing_step(number)?;
                self.steps.remove(&number);
            }
            Verb::DeletePart(number) => {
                let number = self.eval(number)?;
                let part = self.part_number(number)?;
                let numbers: Vec<Num> = self.part_steps(part).map(|(n, _)| *n).collect();
                if numbers.is_empty() {
                    return Err(Message::NoPart.into());
                }
                for n in numbers {
                    self.steps.remove(&n);
                }
            }
            Verb::DeleteForm(number) => {
                let number = self.eval(number)?;
                let n = self.form_number(number)?;
                self.forms.remove(&n).ok_or(Trouble::Said(Message::NoForm))?;
            }
            Verb::DeleteAll(kind) => {
                if matches!(kind, None | Some(Kind::Steps | Kind::Parts)) {
                    // With the steps goes any task that was broken off.
                    self.steps.clear();
                    if at.is_none() {
                        self.task = None;
                    }
                }
                if matches!(kind, None | Some(Kind::Forms)) {
                    self.forms.clear();
                }
                if matches!(kind, None | Some(Kind::Values)) {
                    self.values.clear();
                }
            }
            Verb::Line => self.type_lines(vec![String::new()], at)?,
            Verb::Page => self.new_page(),
            Verb::To(unit, number) => return Ok(Flow::To(*unit, self.eval(number)?)),
            Verb::Done => return Ok(Flow::Done),
            Verb::Stop => return Ok(Flow::Stop),
            Verb::Demand(place, _) => {
                let (letter, indices) = self.locate(place)?;
                return Ok(Flow::Demand(letter, indices));
            }
            Verb::Cancel => return Ok(Flow::Cancel),
            Verb::Go => return Ok(Flow::Go),
        }
        Ok(Flow::Next)
    }

    /// `Type all.` and its kinds. A blank line first; each part and each
    /// form followed by a blank line; one more blank line after the steps
    /// and after the forms when everything is being typed; the values, and
    /// a blank line. (P-2922 fig. 3d and 3e; P-3131 pp. 2-8.)
    ///
    /// The JOSS of the film (March 1964) put no blank line between one form
    /// and the next (RM-4162-PR, example 5).
    fn type_all(&mut self, kind: Option<Kind>) {
        let all = kind.is_none();
        let mut lines = vec![String::new()];
        if all || matches!(kind, Some(Kind::Steps | Kind::Parts)) {
            let mut last_part = None;
            for (number, step) in &self.steps {
                let part = number.ip().to_i64();
                if last_part.is_some() && last_part != part {
                    lines.push(String::new());
                }
                last_part = part;
                lines.push(format!("{} {}", step.label, step.text));
            }
            if last_part.is_some() {
                lines.push(String::new());
                if all {
                    lines.push(String::new());
                }
            }
        }
        if all || kind == Some(Kind::Forms) {
            for (n, form) in &self.forms {
                lines.push(format!("Form {n}:"));
                lines.push(form.clone());
                if self.year == Year::Y1965 {
                    lines.push(String::new());
                }
            }
            if !self.forms.is_empty() {
                if self.year == Year::Y1964 {
                    lines.push(String::new());
                }
                if all {
                    lines.push(String::new());
                }
            }
        }
        if all || kind == Some(Kind::Values) {
            let values = self.value_lines();
            if !values.is_empty() {
                lines.extend(values);
                lines.push(String::new());
            }
        }
        for line in lines {
            self.type_line(line);
        }
    }

    fn value_lines(&self) -> Vec<String> {
        let mut letters: Vec<char> = self.values.keys().copied().collect();
        letters.sort_by_key(|c| letter_order(*c));
        let mut lines = Vec::new();
        for letter in letters {
            match &self.values[&letter] {
                Value::One(value) => lines.push(format::value_line(&letter.to_string(), *value, self.year)),
                Value::Many(elements) => {
                    for (indices, value) in elements {
                        lines.push(format::value_line(&element_label(letter, indices), *value, self.year));
                    }
                }
            }
        }
        lines
    }

    /// Works out the values a `for` clause runs through. "All of this is,
    /// of course, finalized at the time of execution" (RM-4162-PR, p. 34).
    fn iteration(&mut self, clause: &parse::For) -> Outcome<Iteration> {
        let (letter, indices) = self.locate(&clause.place)?;
        let mut runs = Vec::new();
        for run in &clause.runs {
            match run {
                Run::Value(value) => runs.push((self.eval(value)?, Vec::new())),
                Run::Range(first, legs) => {
                    let first = self.eval(first)?;
                    let mut from = first;
                    let mut pairs = Vec::new();
                    for (by, limit) in legs {
                        let (by, limit) = (self.eval(by)?, self.eval(limit)?);
                        // The step must lead from where it starts toward
                        // the limit: -1(1)-4 is refused.
                        let toward = limit.sub(from).map_err(fault)?.sign();
                        if by.is_zero() || (toward != 0 && toward != by.sign()) {
                            return Err(Message::IllegalIteration.into());
                        }
                        pairs.push((by, limit));
                        from = limit;
                    }
                    runs.push((first, pairs));
                }
            }
        }
        let value = runs[0].0;
        Ok(Iteration {
            letter,
            indices,
            runs,
            run: 0,
            leg: 0,
            value,
        })
    }

    fn has(&self, letter: char, indices: &[i64]) -> bool {
        match self.values.get(&letter) {
            Some(Value::One(_)) => indices.is_empty(),
            Some(Value::Many(elements)) => elements.contains_key(indices),
            None => false,
        }
    }

    /// Gives a value, if there is room for one more.
    fn assign(&mut self, letter: char, indices: &[i64], value: Num) -> Outcome<()> {
        let cost = if indices.is_empty() {
            say::VALUE_WORDS
        } else {
            say::ELEMENT_WORDS
        };
        if !self.has(letter, indices) && self.used() + cost > self.storage {
            return Err(Message::MoreStorage.into());
        }
        self.store(letter, indices, value);
        Ok(())
    }

    /// "A new assignment overrides a previous assignment even to the extent
    /// of changing dimensions." (JOSS program notes, p. 2)
    fn store(&mut self, letter: char, indices: &[i64], value: Num) {
        if indices.is_empty() {
            self.values.insert(letter, Value::One(value));
            return;
        }
        match self.values.get_mut(&letter) {
            Some(Value::Many(elements)) if elements.keys().next().is_some_and(|k| k.len() == indices.len()) => {
                elements.insert(indices.to_vec(), value);
            }
            _ => {
                self.values
                    .insert(letter, Value::Many(BTreeMap::from([(indices.to_vec(), value)])));
            }
        }
    }

    /// The letter and the worked-out indices of a place.
    fn locate(&self, place: &Place) -> Outcome<(char, Vec<i64>)> {
        let mut indices = Vec::new();
        for index in &place.indices {
            let value = self.eval(index)?;
            indices.push(
                value
                    .to_i64()
                    .filter(|i| (0..=say::LARGEST_INDEX).contains(i))
                    .ok_or(Trouble::Said(Message::BadIndex))?,
            );
        }
        if indices.len() > 2 {
            return Err(Message::Eh.into());
        }
        Ok((place.letter, indices))
    }

    fn holds(&self, cond: &Cond) -> Outcome<bool> {
        Ok(match cond {
            Cond::Chain(first, rest) => {
                let mut left = self.eval(first)?;
                let mut all = true;
                for (rel, expr) in rest {
                    let right = self.eval(expr)?;
                    all &= match rel {
                        Rel::Eq => left == right,
                        Rel::Ne => left != right,
                        Rel::Le => left <= right,
                        Rel::Ge => left >= right,
                        Rel::Lt => left < right,
                        Rel::Gt => left > right,
                    };
                    left = right;
                }
                all
            }
            // Both sides are worked out, so that a mistake in either is found.
            Cond::And(a, b) => {
                let (a, b) = (self.holds(a)?, self.holds(b)?);
                a && b
            }
            Cond::Or(a, b) => {
                let (a, b) = (self.holds(a)?, self.holds(b)?);
                a || b
            }
        })
    }

    fn eval(&self, expr: &Expr) -> Outcome<Num> {
        Ok(match expr {
            Expr::Num(n) => *n,
            Expr::TooLong => return Err(Message::TooManyDigits.into()),
            Expr::Dollar => Num::int(self.line as i64),
            Expr::Var(place) => {
                let (letter, indices) = self.locate(place)?;
                match (self.values.get(&letter), indices.is_empty()) {
                    (Some(Value::One(value)), true) => *value,
                    (Some(Value::Many(elements)), false) => match elements.get(&indices) {
                        Some(value) => *value,
                        None => return Err(Trouble::Undefined(element_label(letter, &indices))),
                    },
                    _ => return Err(Trouble::Undefined(element_label(letter, &indices))),
                }
            }
            Expr::Neg(inner) => self.eval(inner)?.neg(),
            Expr::Abs(inner) => self.eval(inner)?.abs(),
            Expr::Bin(op, left, right) => {
                let (a, b) = (self.eval(left)?, self.eval(right)?);
                match op {
                    BinOp::Add => a.add(b),
                    BinOp::Sub => a.sub(b),
                    BinOp::Mul => a.mul(b),
                    BinOp::Div => a.div(b),
                    BinOp::Pow => a.pow(b),
                }
                .map_err(fault)?
            }
            Expr::Call(func, args) => {
                let mut values = Vec::new();
                for arg in args {
                    values.push(self.eval(arg)?);
                }
                let x = values[0];
                match func {
                    Func::Sqrt => x.sqrt(),
                    Func::Log => x.log(),
                    Func::Exp => x.exp(),
                    Func::Sin => x.sin(),
                    Func::Cos => x.cos(),
                    Func::Arg => Num::arg(x, values[1]),
                    Func::Ip => Ok(x.ip()),
                    Func::Fp => Ok(x.fp()),
                    Func::Dp => Ok(x.dp()),
                    Func::Xp => Ok(x.xp()),
                    Func::Sgn => Ok(x.sgn()),
                    Func::Max => Ok(values.iter().copied().max().unwrap_or(x)),
                    Func::Min => Ok(values.iter().copied().min().unwrap_or(x)),
                }
                .map_err(fault)?
            }
        })
    }
}

impl Iteration {
    /// Moves to the next value. False when there are no more.
    ///
    /// In a run `a(b)c`, when the next step would reach or pass `c`, `c`
    /// itself is the last value: 1(2)4 is 1, 3, 4. (The listing, routine X7.)
    fn advance(&mut self) -> bool {
        loop {
            let (_, legs) = &self.runs[self.run];
            if let Some((by, limit)) = legs.get(self.leg) {
                if self.value != *limit {
                    let next = self.value.add(*by).unwrap_or(*limit);
                    let passed = if by.is_negative() {
                        next <= *limit
                    } else {
                        next >= *limit
                    };
                    self.value = if passed { *limit } else { next };
                    return true;
                }
                self.leg += 1;
                continue;
            }
            self.run += 1;
            self.leg = 0;
            match self.runs.get(self.run) {
                Some((first, _)) => {
                    self.value = *first;
                    return true;
                }
                None => return false,
            }
        }
    }
}
