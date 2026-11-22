//! What JOSS says.
//!
//! "For its comments, it simply selects from a stock of forty 'canned'
//! messages. ... After a year's experience with neutral, impersonal messages,
//! we introduced personal pronouns and some informal diction." (P-3149, p. 6)
//!
//! The 1965 wording is not taken from the papers, which show about a dozen
//! of the forty. It is taken from the program: the JOHNNIAC assembly listing
//! headed "JOSS 666, JCS 1-9-65" keeps each message as octal words, five
//! characters to a word, and they were decoded with the listing's own
//! character table (`sources/AUDIT.md`,
//! `sources/listing-1965/transcripts/messages.txt`). The numbers E0 to E31
//! are the listing's.
//!
//! The 1964 wording is known only where a 1964 page shows it: four messages
//! and the interrupt. Where no 1964 page shows a message, the 1965 words are
//! used in 1964 as well, and that is a gap, not a finding. (An earlier,
//! undated listing has a third and older wording, "Attempting division by
//! zero." and the like, which matches neither.)

use crate::session::Year;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Message {
    /// E0. Typed bare, with no "Error" in front.
    MoreStorage,
    /// E1.
    Overflow,
    /// E3.
    ZeroDivisor,
    /// E4.
    BadIndex,
    /// E5.
    TooManyDigits,
    /// E6. Typed bare when the line was given directly.
    Eh,
    /// E7.
    NothingToDo,
    /// E8.
    TooManyValues,
    /// E9.
    NotIndirectly,
    /// E10.
    NotDirectly,
    /// E11.
    LongStepLabel,
    /// E12.
    FormNumber,
    /// E13.
    StepNumber,
    /// E14.
    NegativeSqrt,
    /// E15.
    ShortField,
    /// E16.
    NoStep,
    /// E17.
    NoPart,
    /// E18.
    PartNumber,
    /// E19.
    NoForm,
    /// E20.
    BadLog,
    /// E21.
    NegativeBase,
    /// E22.
    BadAngle,
    /// E23.
    ZeroToNegative,
    /// E24.
    TooFewValues,
    /// E25.
    IndividualValues,
    /// E26.
    NoPartForIteration,
    /// E27.
    NoStepForIteration,
    /// E28.
    IllegalIteration,
    /// E30.
    FieldsRunTogether,
    /// E31.
    CantExpress,
    /// Not in the listing: `arg(0,0)`, which has no direction. SUPPLIED: the
    /// situation exists, and how JOSS answered it was not recovered.
    ArgOfOrigin,
}

/// Every message, for lists and for tests.
pub const ALL: &[Message] = &[
    Message::MoreStorage,
    Message::Overflow,
    Message::ZeroDivisor,
    Message::BadIndex,
    Message::TooManyDigits,
    Message::Eh,
    Message::NothingToDo,
    Message::TooManyValues,
    Message::NotIndirectly,
    Message::NotDirectly,
    Message::LongStepLabel,
    Message::FormNumber,
    Message::StepNumber,
    Message::NegativeSqrt,
    Message::ShortField,
    Message::NoStep,
    Message::NoPart,
    Message::PartNumber,
    Message::NoForm,
    Message::BadLog,
    Message::NegativeBase,
    Message::BadAngle,
    Message::ZeroToNegative,
    Message::TooFewValues,
    Message::IndividualValues,
    Message::NoPartForIteration,
    Message::NoStepForIteration,
    Message::IllegalIteration,
    Message::FieldsRunTogether,
    Message::CantExpress,
    Message::ArgOfOrigin,
];

impl Message {
    pub fn text(self, year: Year) -> &'static str {
        use Message::*;
        if year == Year::Y1964 {
            // As printed in RM-4162-PR (film of March 1964) and P-2922
            // (August 1964).
            match self {
                ZeroDivisor => return "Zero divisor.",
                NegativeSqrt => return "Negative argument for sqrt.",
                NoPart => return "No such part.",
                // "an error message which will print out the words
                // 'Insufficient storage space'" (RM-4162-PR, p. 37). The
                // full stop is from the earlier listing.
                MoreStorage => return "Insufficient storage space.",
                _ => {}
            }
        }
        match self {
            MoreStorage => "I need more storage space.",
            Overflow => "I have an overflow.",
            ZeroDivisor => "I have a zero divisor.",
            BadIndex => "Index value must be integer and 0≤index≤99.",
            TooManyDigits => "Please limit numbers to 9 significant digits.",
            Eh => "Eh?",
            NothingToDo => "You haven't told me to do anything yet.",
            TooManyValues => "I have too many values for the form.",
            NotIndirectly => "Don't give this command indirectly.",
            NotDirectly => "Don't give this command directly.",
            LongStepLabel => "Please limit step labels to 9 digits.",
            FormNumber => "Form number must be integer and 0<form<10*9.",
            StepNumber => "Step number must satisfy 1≤step<10*9.",
            NegativeSqrt => "I have a negative argument for sqrt.",
            ShortField => "Field definition has fewer than 7 periods.",
            NoStep => "I can't find the required step.",
            NoPart => "I can't find the required part.",
            PartNumber => "Part number must be integer and 0<part<10*9.",
            NoForm => "I can't find the required form.",
            BadLog => "I have an argument ≤ 0 for log.",
            NegativeBase => "I have a negative base to a fractional power.",
            BadAngle => "Please keep |x|<100 for sin(x) or cos(x).",
            ZeroToNegative => "I have zero to a negative power.",
            TooFewValues => "I have too few values for the form.",
            IndividualValues => "I need individual values for a form.",
            NoPartForIteration => "I can't find the part for iteration.",
            NoStepForIteration => "I can't find the step for iteration.",
            IllegalIteration => "Illegal set of values for iteration.",
            FieldsRunTogether => "Your fields run together in the form.",
            CantExpress => "I can't express value in your form.",
            ArgOfOrigin => "I have arguments of zero for arg.",
        }
    }
}

/// What stopped a step: one of the canned messages, or a name that has no
/// value, or the interrupt key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Trouble {
    Said(Message),
    /// E2: "x is undefined.", "a(1,2) is undefined."
    Undefined(String),
    /// The INTERRUPT key.
    Interrupt,
}

impl From<Message> for Trouble {
    fn from(message: Message) -> Trouble {
        Trouble::Said(message)
    }
}

impl Trouble {
    pub fn text(&self, year: Year) -> String {
        match self {
            Trouble::Said(message) => message.text(year).to_string(),
            // The film of 1964 shows only the bare word.
            Trouble::Undefined(_) if year == Year::Y1964 => "Undefined.".to_string(),
            Trouble::Undefined(name) => format!("{name} is undefined."),
            Trouble::Interrupt => String::new(),
        }
    }
}

/// A colon and two spaces. Shaw's note of 10-10-64 budgets the line:
/// "Error during step ____:__  5 1 6 1 4 1 10 1 2".
pub fn error_at_step(step: &str, message: &str) -> String {
    format!("Error at step {step}:  {message}")
}

pub fn error_during_step(step: &str, message: &str) -> String {
    format!("Error during step {step}:  {message}")
}

pub fn error_above(message: &str) -> String {
    format!("Error above:  {message}")
}

/// The INTERRUPT key, pressed while a stored program is running.
pub fn interrupted(step: &str, year: Year) -> String {
    match year {
        Year::Y1964 => format!("Interrupted at step {step}."),
        Year::Y1965 => format!("I'm at step {step}."),
    }
}

pub fn stopped(step: &str) -> String {
    format!("Stopped by step {step}.")
}

/// Two spaces after "mess.", as in the listing and in figure 2 of P-3149.
pub const MESS: &str = "It's a mess.  Let's start over.";

/// Typed by JOSS on its own account when the operators wanted the machine
/// back (P-3149, figure 2). Nothing here ever types it.
pub const WIND_UP: &str = "Please wind up your work and turn off as soon as possible.";

pub const LARGEST_INDEX: i64 = 99;

/// "1024 words for each user which covers tables, steps, forms and values."
/// (JOSS program notes, p. 2). The handwritten storage map of the same
/// papers leaves "Available Space 720" of them for the user's own material
/// once JOSS's tables are in.
pub const STORAGE_WORDS: usize = 720;

// How the words are spent is this implementation's estimate. Text is kept
// five characters to a word, as the listing keeps its own messages.
pub const VALUE_WORDS: usize = 3;
pub const ELEMENT_WORDS: usize = 4;
/// One level of a task: where it is, and the values a `for` runs through.
pub const FRAME_WORDS: usize = 12;

pub fn text_words(text: &str) -> usize {
    2 + text.chars().count().div_ceil(5)
}
