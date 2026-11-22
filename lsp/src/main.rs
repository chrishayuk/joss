//! A language server for JOSS.
//!
//! It speaks the Language Server Protocol over standard input and output, so
//! any editor with an LSP client can use it. Everything it knows about a file
//! comes from `src/analysis.rs`, the same code behind `joss --check`, which
//! in turn asks the interpreter's own lexer and parser.
//!
//! The server reads one message at a time and keeps nothing but the open
//! documents, which lets it also be compiled for WASI and run inside an
//! editor or a web page with nothing to install (see `editors/vscode` and
//! `web/lsp-worker.js`).
//!
//! JOSS counts columns, as a typewriter does: one character, one column. The
//! protocol counts UTF-16 code units. `Doc` converts between the two.

use joss::analysis::{analyse, Analysis, Kind, LineKind, Named, Severity, Token};
use joss::format::{self, FieldKind};
use joss::lex::{self, Tok, CONNECTIVES, VERBS};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

const SEMANTIC_TYPES: &[&str] = &[
    "comment",
    "string",
    "number",
    "variable",
    "function",
    "parameter",
    "operator",
    "keyword",
    "type",
    "label",
];
const SEMANTIC_MODIFIERS: &[&str] = &["definition", "defaultLibrary"];

/// The characters of the JOSS typewriter that a modern keyboard lacks: the
/// character, what can be typed for it, and what it means.
const SPECIALS: &[(&str, &str, &str)] = &[
    ("·", "@", "times"),
    ("≠", "!=", "is not equal to"),
    ("≤", "<=", "is at most"),
    ("≥", ">=", "is at least"),
];

/// What a word of the language is for: a heading, a paragraph and an example.
/// All of it is JOSS as it stood in January 1965 (the one-page summary in
/// P-3149 and the listing of 9 January), which is what `src/parse.rs` reads.
fn word_help(word: &str) -> Option<(&'static str, &'static str, &'static str)> {
    Some(match word {
        "Set" => (
            "gives a letter a value",
            "Works out the expression and keeps the value under the letter. A letter may have one or two indices, each a whole number from 0 to 99. Given directly, the word `Set` may be left out: `x = 3`.",
            "Set x = 3.\nSet a(1,2) = x·2.",
        ),
        "Do" => (
            "does a step or a part",
            "Does one step, or every step of a part in order of their numbers, and then comes back. With `for`, it does them once for each value. A `Do` given directly drops any task that was broken off.",
            "Do part 1.\nDo step 1.2.\nDo part 2 for x = 1(1)10.",
        ),
        "Type" => (
            "types values, text or the stored program",
            "Types each value on a line of its own, labelled with the expression as it was written. An underscore in the list is a blank line. With `in form`, the values go into the blanks of a form. `Type` also types a text in quotes, a step, a part, a form, `all`, `size`, `time` and `users`.",
            "Type x, x*2.\nType a, b, c in form 1.\nType \"Root\".\nType all steps.",
        ),
        "Delete" => (
            "forgets values, steps, parts or forms",
            "`Delete x, y.` forgets the values of letters. `Delete step`, `Delete part`, `Delete form` and `Delete all` forget what they name. Only `Delete x, y.` and `Delete all values.` may be kept as steps. The others are given directly.",
            "Delete x, a(1).\nDelete step 1.2.\nDelete part 1.\nDelete all.",
        ),
        "Line" => ("types a blank line", "Types a line with nothing on it.", "1.2 Line if fp(x/5) = 1/5."),
        "Page" => (
            "starts a new page",
            "Moves the paper to the top of a new page.",
            "2.1 Page.",
        ),
        "To" => (
            "goes on from another step",
            "Only as a step. JOSS goes on from the step named, or from the first step of the part named, and not from the next step.",
            "1.4 To part 1 if |y| ≥ e.\n2.3 To step 2.1.",
        ),
        "Done" => (
            "leaves the part",
            "Only as a step. The rest of the part is passed over. If the part is being done `for` several values, JOSS goes on to the next value.",
            "3.1 Done if i = n.",
        ),
        "Stop" => (
            "breaks off the task",
            "Only as a step. JOSS types `Stopped by step 1.3.` and gives the typewriter back. `Go.` carries on from the next step.",
            "1.3 Stop if x > 100.",
        ),
        "Demand" => (
            "asks for a value",
            "Only as a step. JOSS types the name and `=`, and waits. The answer may be any expression.",
            "11.1 Demand a(i,j).",
        ),
        "Cancel" => (
            "drops a task that was broken off",
            "Only directly. After it, `Go.` has nothing to take up.",
            "Cancel.",
        ),
        "Go" => (
            "takes up a task that was broken off",
            "Only directly. Carries on after `Stop`, after an error in a step, or after the interrupt key. After an error the step is done again, so the mistake can be put right first. With nothing to carry on, JOSS says \"You haven't told me to do anything yet.\"",
            "Set e = .00005.\nGo.",
        ),
        "Form" => (
            "the heading of a form",
            "Only directly. `Form 1:` says that the next line typed is form 1, a picture of a line of output. A run of underscores is a blank for a number in fixed point, with a point where the decimal point goes. A run of seven or more periods is a blank in scientific notation. Everything else is typed as it stands.",
            "Form 1:\na = __   b = __   c = ___.______\nType a, b, c in form 1.",
        ),
        "step" => (
            "one stored instruction",
            "A step is a line typed with a number in front, such as 1.2. It is kept and not done until a `Do` reaches it. Typing a step again replaces it.",
            "Do step 1.2.\nType step 1.2.\nDelete step 1.2.",
        ),
        "part" => (
            "the steps with the same whole number",
            "Steps 1.1, 1.2 and 1.35 make part 1. The steps of a part are done in order of their numbers. A part number is a whole number.",
            "Do part 1.\nType part 1.\nDelete part 1.",
        ),
        "form" => (
            "a picture of a line of output",
            "A form is typed after the heading `Form 1:`. `Type ... in form 1.` fills its blanks with values, in order. `Type form 1.` types the form itself.",
            "Type a, b, c in form 1.\nType form 2.\nDelete form 1.",
        ),
        "all" => (
            "everything",
            "`Type all.` types every step, every form and every value. `Delete all.` forgets them. With `steps`, `parts`, `forms` or `values` after it, it means all of those.",
            "Type all.\nDelete all values.",
        ),
        "steps" => ("every step", "After `all`.", "Type all steps.\nDelete all steps."),
        "parts" => ("every part", "After `all`.", "Type all parts.\nDelete all parts."),
        "forms" => ("every form", "After `all`.", "Type all forms.\nDelete all forms."),
        "values" => (
            "every value",
            "After `all`. `Type all values.` types each letter that has a value, small letters before capitals.",
            "Type all values.\nDelete all values.",
        ),
        "size" => (
            "storage in use",
            "`Type size.` types how many words of storage the steps, forms and values are using.",
            "Type size.",
        ),
        "time" => (
            "the time of day",
            "`Type time.` types the time as four digits. 1230 is half past twelve.",
            "Type time.",
        ),
        "users" => (
            "people at consoles",
            "`Type users.` types how many people are using JOSS.",
            "Type users.",
        ),
        "if" => (
            "a condition",
            "May end any instruction. The instruction is done only when the condition holds. The relations are `=`, `≠`, `<`, `≤`, `>` and `≥`, and they may be chained: `0 ≤ x < 10`. `and` and `or` join conditions.",
            "1.2 Type x if x ≠ 0.\n1.4 To part 1 if |y| ≥ e.",
        ),
        "for" => (
            "a range of values",
            "After `Do`. The step or part is done once for each value of the letter. A value may stand alone, or a run may be written first(step)last, and several may be listed with commas. A run ends on its limit: `1(2)4` gives 1, 3, 4.",
            "Do part 1 for x = 2(1)50.\nDo part 2 for x = .5(.5)3, 3.5, 4.",
        ),
        "in" => (
            "into a form",
            "`Type ... in form 1.` puts the values into the blanks of form 1. There must be as many values as blanks.",
            "Type a, b, c in form 1.",
        ),
        "and" => (
            "both conditions hold",
            "Joins two conditions after `if`. `and` and `or` are taken from left to right. Neither comes before the other.",
            "1.1 Type x if x > 0 and x < 10.",
        ),
        "or" => (
            "either condition holds",
            "Joins two conditions after `if`. `and` and `or` are taken from left to right. Neither comes before the other.",
            "1.1 Type x if x < 0 or x > 10.",
        ),
        _ => return None,
    })
}

/// The functions: the name, how a call is written, what it gives, and an
/// example.
const FUNCTIONS_HELP: &[(&str, &str, &str, &str)] = &[
    (
        "sqrt",
        "sqrt(x)",
        "The square root of x. x must not be negative.",
        "Type sqrt(144).",
    ),
    (
        "log",
        "log(x)",
        "The natural logarithm of x. x must be above zero.",
        "Type log(10).",
    ),
    ("exp", "exp(x)", "e to the power x.", "Type exp(1)."),
    (
        "sin",
        "sin(x)",
        "The sine of x, with x in radians. |x| must be below 100.",
        "Type sin(.2).",
    ),
    (
        "cos",
        "cos(x)",
        "The cosine of x, with x in radians. |x| must be below 100.",
        "Type cos(.2).",
    ),
    (
        "arg",
        "arg(x, y)",
        "The angle of the point (x, y), in radians, from -π to π. `4·arg(3,3)` is π.",
        "Type arg(3,3).",
    ),
    (
        "ip",
        "ip(x)",
        "The integer part of x, with its sign. `ip(-3.5)` is -3.",
        "Type ip(123.456).",
    ),
    (
        "fp",
        "fp(x)",
        "The fraction part of x, with its sign. `ip(x) + fp(x)` is x.",
        "1.2 Type a, b, c in form 1 if fp(c) = 0.",
    ),
    (
        "dp",
        "dp(x)",
        "The digit part of x: its digits with the point after the first. `dp(123.456)` is 1.23456.",
        "Type dp(123.456).",
    ),
    (
        "xp",
        "xp(x)",
        "The exponent part of x: the power of ten that goes with the digit part. `xp(123.456)` is 2.",
        "Type xp(123.456).",
    ),
    (
        "sgn",
        "sgn(x)",
        "The sign of x: -1, 0 or 1.",
        "Type sgn(-3.5), sgn(0), sgn(3.5).",
    ),
    (
        "max",
        "max(a, b, ...)",
        "The largest of two or more values.",
        "Type max(6·x, 20, x*3).",
    ),
    (
        "min",
        "min(a, b, ...)",
        "The smallest of two or more values.",
        "Type min(6·x, 20, x*3).",
    ),
];

/// How a call of a function is written, what it gives, and an example.
fn function_help(name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    FUNCTIONS_HELP
        .iter()
        .find(|(known, ..)| *known == name)
        .map(|(_, signature, body, example)| (*signature, *body, *example))
}

/// What an operator or a piece of punctuation is for. The symbol is as JOSS
/// holds it: `·`, not `@`.
fn symbol_help(symbol: &str) -> Option<(&'static str, &'static str, &'static str)> {
    Some(match symbol {
        "+" => ("plus", "Addition, or a sign in front of a term.", "Type x+2."),
        "-" => (
            "minus",
            "Subtraction, or a sign in front of a term. The sign takes the whole term: `-x*2` is minus the square of x.",
            "Type x-2, -x*2.",
        ),
        "·" => (
            "times",
            "Multiplication. It is always written: `2·x`, never `2x`. On a keyboard without it, type `@`.",
            "Type 2·x.",
        ),
        "/" => ("divided by", "Division. `·` and `/` are done from left to right.", "Type x/2."),
        "*" => (
            "to the power",
            "`x*2` is x squared. The star is not times: times is `·`. The power may have its own sign: `10*-3`. Powers are done before `·` and `/`, and those before `+` and `-`.",
            "Type x*2, 1.23456·10*2.",
        ),
        "=" => (
            "equals",
            "After `Set` and after `for`, it gives the letter its value. After `if`, it is the relation \"is equal to\".",
            "Set x = 3.\n1.2 Type x if fp(x/2) = 0.",
        ),
        "≠" => (
            "is not equal to",
            "A relation, used after `if`. On a keyboard without it, type `!=`.",
            "1.2 Type x if x ≠ 0.",
        ),
        "≤" => (
            "is at most",
            "A relation, used after `if`: less than or equal to. Relations may be chained: `0 ≤ x < 10`. On a keyboard without it, type `<=`.",
            "1.1 Type x if 0 ≤ x < 10.",
        ),
        "≥" => (
            "is at least",
            "A relation, used after `if`: greater than or equal to. On a keyboard without it, type `>=`.",
            "1.4 To part 1 if |y| ≥ e.",
        ),
        "<" => (
            "is less than",
            "A relation, used after `if`. Relations may be chained: `0 ≤ x < 10`.",
            "1.1 Type x if x < 10.",
        ),
        ">" => (
            "is greater than",
            "A relation, used after `if`.",
            "1.1 Do part 2 for d = 2(1)ip[sqrt(x)] if x > 3.",
        ),
        "(" | ")" | "[" | "]" => (
            "brackets",
            "Round and square brackets mean the same, and each closes with its own kind. They group part of an expression, hold the indices of a letter and the arguments of a function, and hold the step of a run after `for`.",
            "Type [(|x-5|·3+4)·2-15]·3+10.\nDo part 1 for x = 1(1)10.",
        ),
        "|" => (
            "absolute value",
            "`|x|` is x without its sign.",
            "Type |x-5|.",
        ),
        "." => (
            "full stop",
            "Ends every instruction. A point with a digit after it belongs to the number: in `Set x = 1.5.` only the last point ends the instruction.",
            "Set x = 1.5.",
        ),
        "," => (
            "comma",
            "Separates the things in a list: the values after `Type`, the letters after `Delete`, the indices of a letter, the arguments of a function, the values after `for`.",
            "Type a, b, a(1,2).",
        ),
        ":" => (
            "colon",
            "Ends the heading of a form. The form itself is the next line.",
            "Form 1:\nx = __.______   y = __.______",
        ),
        _ => return None,
    })
}

struct Doc {
    /// The lines of the file, as `analyse` counts them, a character at a time.
    lines: Vec<Vec<char>>,
    analysis: Analysis,
}

impl Doc {
    fn new(text: &str) -> Self {
        Doc {
            lines: text.lines().map(|line| line.chars().collect()).collect(),
            analysis: analyse(text),
        }
    }

    fn width(&self, line: usize) -> usize {
        self.lines.get(line).map_or(0, Vec::len)
    }

    /// A column (a count of characters) as a count of UTF-16 code units.
    fn units(&self, line: usize, column: usize) -> usize {
        match self.lines.get(line) {
            Some(chars) => chars[..column.min(chars.len())].iter().map(|c| c.len_utf16()).sum(),
            None => 0,
        }
    }

    /// The column that a count of UTF-16 code units reaches.
    fn column(&self, line: usize, mut units: usize) -> usize {
        let Some(chars) = self.lines.get(line) else {
            return 0;
        };
        for (column, c) in chars.iter().enumerate() {
            if units < c.len_utf16() {
                return column;
            }
            units -= c.len_utf16();
        }
        chars.len()
    }

    fn range(&self, line: usize, start: usize, end: usize) -> Value {
        json!({
            "start": { "line": line, "character": self.units(line, start) },
            "end": { "line": line, "character": self.units(line, end) },
        })
    }

    fn token_range(&self, t: &Token) -> Value {
        self.range(t.line, t.start, t.end)
    }

    fn line_range(&self, line: usize) -> Value {
        self.range(line, 0, self.width(line))
    }

    /// The line and column of a position sent by the editor.
    fn place(&self, position: &Value) -> (usize, usize) {
        let line = position["line"].as_u64().unwrap_or(0) as usize;
        let units = position["character"].as_u64().unwrap_or(0) as usize;
        (line, self.column(line, units))
    }

    fn token_at(&self, position: &Value) -> Option<&Token> {
        let (line, column) = self.place(position);
        self.analysis.token_at(line, column)
    }

    /// What is in the file between two columns of a line.
    fn raw(&self, line: usize, start: usize, end: usize) -> String {
        match self.lines.get(line) {
            Some(chars) => chars[start.min(chars.len())..end.min(chars.len())].iter().collect(),
            None => String::new(),
        }
    }

    /// A token as it was typed: `!=` where JOSS holds `≠`.
    fn typed(&self, t: &Token) -> String {
        self.raw(t.line, t.start, t.end)
    }

    /// A whole line, without the spaces around it.
    fn line_text(&self, line: usize) -> String {
        self.raw(line, 0, usize::MAX).trim().to_string()
    }

    fn kind(&self, line: usize) -> &LineKind {
        self.analysis.lines.get(line).map_or(&LineKind::Empty, |l| &l.kind)
    }

    /// The number in front of a step, on a line that is one.
    fn step_number(&self, line: usize) -> Option<&Token> {
        self.analysis
            .tokens
            .iter()
            .find(|t| t.line == line && t.kind == Kind::StepNumber)
    }

    /// The step, part or form that a token names: a numeral after `step`,
    /// `part` or `form`, or the number in front of a step.
    fn named(&self, t: &Token) -> Option<Named> {
        match (t.names, t.kind, self.kind(t.line)) {
            (Some(named), _, _) => Some(named),
            (None, Kind::StepNumber, LineKind::Step { number, .. }) => Some(Named::Step(*number)),
            _ => None,
        }
    }

    /// The tokens that were typed with a stand-in, and are not in text.
    fn stand_ins(&self) -> Vec<&Token> {
        self.analysis
            .tokens
            .iter()
            .filter(|t| t.kind == Kind::Operator && self.typed(t) != t.text)
            .collect()
    }
}

fn times(n: usize) -> String {
    match n {
        0 => "nowhere".into(),
        1 => "once".into(),
        2 => "twice".into(),
        n => format!("{n} times"),
    }
}

fn count(n: usize, one: &str) -> String {
    match n {
        1 => format!("1 {one}"),
        n => format!("{n} {one}s"),
    }
}

fn fenced(lines: &[String]) -> String {
    format!("```joss\n{}\n```", lines.join("\n"))
}

/// Templates offered at the start of a line: a label, what it is, and the
/// text in the snippet form editors use.
const TEMPLATES: &[(&str, &str, &str)] = &[
    (
        "Do part for",
        "Do part 1 for i = 1(1)10.",
        "Do part ${1:1} for ${2:i} = ${3:1}(${4:1})${5:10}.",
    ),
    ("Form", "Form 1: and the picture on the next line", "Form ${1:1}:\n$0"),
    (
        "Type in form",
        "Type a, b in form 1.",
        "Type ${1:a}, ${2:b} in form ${3:1}.",
    ),
    ("Type text", "Type \"...\".", "Type \"$1\"."),
    ("Set", "Set x = ...", "Set ${1:x} = $0."),
];

struct Server {
    docs: BTreeMap<String, Doc>,
    shutdown: bool,
    exit: bool,
}

impl Server {
    fn new() -> Self {
        Server {
            docs: BTreeMap::new(),
            shutdown: false,
            exit: false,
        }
    }

    /// Handles one incoming message and returns the messages to send back.
    fn handle(&mut self, message: &Value) -> Vec<Value> {
        let method = message["method"].as_str().unwrap_or("");
        let params = &message["params"];
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("").to_string();

        let Some(id) = message.get("id").filter(|_| !method.is_empty()) else {
            // A notification, or a reply to something we never ask.
            return match method {
                "textDocument/didOpen" => self.open(uri, params["textDocument"]["text"].as_str().unwrap_or("")),
                "textDocument/didChange" => {
                    // Full sync: the last change holds the whole text.
                    let changes = params["contentChanges"].as_array();
                    match changes.and_then(|c| c.last()).and_then(|c| c["text"].as_str()) {
                        Some(text) => self.open(uri, text),
                        None => vec![],
                    }
                }
                "textDocument/didClose" => {
                    self.docs.remove(&uri);
                    vec![json!({ "jsonrpc": "2.0", "method": "textDocument/publishDiagnostics",
                        "params": { "uri": uri, "diagnostics": [] } })]
                }
                "exit" => {
                    self.exit = true;
                    vec![]
                }
                _ => vec![],
            };
        };

        let reply = |result: Value| vec![json!({ "jsonrpc": "2.0", "id": id, "result": result })];
        let fail = |code: i32, text: &str| {
            vec![json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": text } })]
        };

        match method {
            "initialize" => return reply(self.capabilities()),
            "shutdown" => {
                self.shutdown = true;
                return reply(Value::Null);
            }
            _ => {}
        }
        if !method.starts_with("textDocument/") {
            return fail(-32601, &format!("method not supported: {method}"));
        }
        let Some(doc) = self.docs.get(&uri) else {
            return reply(Value::Null);
        };
        let position = &params["position"];

        match method {
            "textDocument/hover" => reply(hover(doc, position)),
            "textDocument/definition" => reply(definition(doc, &uri, position)),
            "textDocument/references" => {
                let with_declaration = params["context"]["includeDeclaration"].as_bool().unwrap_or(true);
                let found: Vec<Value> = related(doc, position)
                    .into_iter()
                    .filter(|(_, defines)| with_declaration || !defines)
                    .map(|(t, _)| json!({ "uri": uri, "range": doc.token_range(t) }))
                    .collect();
                reply(json!(found))
            }
            "textDocument/documentHighlight" => {
                let found: Vec<Value> = related(doc, position)
                    .into_iter()
                    .map(|(t, defines)| json!({ "range": doc.token_range(t), "kind": if defines { 3 } else { 2 } }))
                    .collect();
                reply(json!(found))
            }
            "textDocument/prepareRename" => reply(prepare_rename(doc, position)),
            "textDocument/rename" => match rename(doc, &uri, position, params["newName"].as_str().unwrap_or("")) {
                Ok(edit) => reply(edit),
                Err(text) => fail(-32602, &text),
            },
            "textDocument/completion" => reply(completion(doc, position)),
            "textDocument/signatureHelp" => reply(signature_help(doc, position)),
            "textDocument/documentSymbol" => reply(symbols(doc)),
            "textDocument/foldingRange" => reply(folding(doc)),
            "textDocument/semanticTokens/full" => reply(json!({ "data": semantic_tokens(doc) })),
            "textDocument/formatting" => reply(json!(real_characters(doc, None))),
            "textDocument/codeAction" => reply(code_actions(doc, &uri, &params["range"])),
            _ => fail(-32601, &format!("method not supported: {method}")),
        }
    }

    fn capabilities(&self) -> Value {
        json!({
            "serverInfo": { "name": "joss-lsp", "version": env!("CARGO_PKG_VERSION") },
            "capabilities": {
                "positionEncoding": "utf-16",
                "textDocumentSync": { "openClose": true, "change": 1 },
                "hoverProvider": true,
                "definitionProvider": true,
                "referencesProvider": true,
                "documentHighlightProvider": true,
                "renameProvider": { "prepareProvider": true },
                "completionProvider": { "triggerCharacters": ["@", "="] },
                "signatureHelpProvider": { "triggerCharacters": ["(", "[", ","], "retriggerCharacters": [","] },
                "documentSymbolProvider": true,
                "foldingRangeProvider": true,
                "documentFormattingProvider": true,
                "codeActionProvider": { "codeActionKinds": ["quickfix", "source.fixAll"] },
                "semanticTokensProvider": {
                    "legend": { "tokenTypes": SEMANTIC_TYPES, "tokenModifiers": SEMANTIC_MODIFIERS },
                    "full": true,
                },
            },
        })
    }

    fn open(&mut self, uri: String, text: &str) -> Vec<Value> {
        let doc = Doc::new(text);
        let diagnostics: Vec<Value> = doc
            .analysis
            .diagnostics
            .iter()
            .map(|d| {
                let mut out = json!({
                    "range": doc.range(d.line, d.start, d.end),
                    "severity": match d.severity { Severity::Error => 1, Severity::Warning => 2, Severity::Hint => 4 },
                    "code": d.code,
                    "source": "joss",
                    "message": d.message,
                });
                if d.unnecessary {
                    out["tags"] = json!([1]);
                }
                out
            })
            .collect();
        self.docs.insert(uri.clone(), doc);
        vec![json!({ "jsonrpc": "2.0", "method": "textDocument/publishDiagnostics",
            "params": { "uri": uri, "diagnostics": diagnostics } })]
    }
}

/// A heading, a paragraph and an example, as the Markdown of a hover.
fn entry(title: &str, (heading, body, example): (&str, &str, &str)) -> String {
    let mut text = format!("**`{title}`** {heading}\n\n{body}");
    if !example.is_empty() {
        text.push_str(&format!("\n\n```joss\n{example}\n```"));
    }
    text
}

/// What to say about a step, a part or a form named by number.
fn describe(doc: &Doc, named: Named) -> String {
    let a = &doc.analysis;
    match named {
        Named::Step(n) => {
            let label = format::plain(n);
            match a.definition(named) {
                Some(line) => format!(
                    "**Step {label}**\n\n{}\n\nIt is on line {}, in part {}.",
                    fenced(&[doc.line_text(line)]),
                    line + 1,
                    format::plain(n.ip())
                ),
                None => format!("**Step {label}**\n\nThere is no step {label} in this file."),
            }
        }
        Named::Part(p) => {
            let lines = a.part_lines(p);
            if lines.is_empty() {
                return format!("**Part {p}**\n\nPart {p} has no steps in this file.");
            }
            let steps: Vec<String> = lines.iter().map(|&line| doc.line_text(line)).collect();
            format!(
                "**Part {p}** {}\n\n{}\n\nA part is every step whose number has the same whole part. The steps are done in order of their numbers.",
                count(steps.len(), "step"),
                fenced(&steps)
            )
        }
        Named::Form(n) => match a.definition(named) {
            Some(line) => {
                let picture = doc.raw(line, 0, usize::MAX);
                let blanks = match format::fields(&picture) {
                    Ok(found) => format!("It has {}.", count(found.len(), "blank")),
                    Err(_) => "Its blanks cannot be told apart.".to_string(),
                };
                format!(
                    "**Form {n}**\n\n```joss\nForm {n}:\n{picture}\n```\n\n{blanks} `Type ... in form {n}.` fills them in order."
                )
            }
            None => format!("**Form {n}**\n\nThere is no form {n} in this file."),
        },
    }
}

/// What to say about a blank in the picture of a form.
fn describe_field(doc: &Doc, t: &Token) -> String {
    let picture = doc.raw(t.line, 0, usize::MAX);
    let field = format::fields(&picture)
        .ok()
        .and_then(|found| found.into_iter().find(|f| f.start == t.start));
    match field.map(|f| (f.len, f.kind)) {
        Some((width, FieldKind::Fixed { decimals, point })) => {
            let signed = doc.raw(t.line, t.start, t.start + 1) == "-";
            let mut text = format!(
                "**Fixed blank** {} wide, {}\n\nA number is typed here in fixed point, set to the right and rounded to fit.",
                count(width, "column"),
                match (point, decimals) {
                    (false, _) => "no decimal point".to_string(),
                    (true, 0) => "a point and no decimals".to_string(),
                    (true, d) => count(d, "decimal"),
                }
            );
            if signed {
                text.push_str(" The hyphen in front keeps a column for the sign.");
            }
            text.push_str(" A number too big for it stops the step: \"I can't express value in your form.\"");
            text
        }
        Some((width, FieldKind::Scientific)) => format!(
            "**Scientific blank** {} wide, {}\n\nA row of seven or more periods. A number is typed here as a sign, a digit part and a power of ten in two digits, with the ten left out. Thirteen periods hold ` 5.4598150 01`.",
            count(width, "column"),
            count(width - 6, "decimal")
        ),
        None => "**Blank** in a form".to_string(),
    }
}

/// What to say about a letter: where it is given a value.
fn describe_letter(doc: &Doc, letter: &str) -> String {
    let uses = doc.analysis.letter_uses(letter);
    let mut lines: Vec<usize> = uses.iter().filter(|t| t.defines).map(|t| t.line).collect();
    lines.dedup();
    let mut text = format!("**Letter `{letter}`**\n\n");
    if lines.is_empty() {
        text.push_str("Nothing in this file gives it a value.");
    } else {
        let shown: Vec<String> = lines.iter().take(8).map(|&line| doc.line_text(line)).collect();
        let numbers: Vec<String> = lines.iter().map(|line| (line + 1).to_string()).collect();
        text.push_str(&format!(
            "It is given a value on line{} {}.\n\n{}",
            if numbers.len() == 1 { "" } else { "s" },
            numbers.join(", "),
            fenced(&shown)
        ));
    }
    text.push_str(&format!(
        "\n\nIt is written {}. JOSS has 52 names: the small letters and the capitals, and `{}` is not `{}`.",
        times(uses.len()),
        letter,
        if letter.chars().all(|c| c.is_ascii_lowercase()) {
            letter.to_ascii_uppercase()
        } else {
            letter.to_ascii_lowercase()
        }
    ));
    text
}

fn hover(doc: &Doc, position: &Value) -> Value {
    let Some(t) = doc.token_at(position) else {
        return Value::Null;
    };
    let a = &doc.analysis;
    let in_form = matches!(doc.kind(t.line), LineKind::FormBody(_));
    let text = match t.kind {
        Kind::Comment => return Value::Null,
        Kind::StepNumber => match doc.named(t) {
            Some(Named::Step(n)) => {
                let label = format::plain(n);
                let part = n.ip().to_i64().unwrap_or(0);
                let steps: Vec<String> = a
                    .steps
                    .keys()
                    .filter(|s| s.ip().to_i64() == Some(part))
                    .map(|s| format::plain(*s))
                    .collect();
                let mut text = format!(
                    "**Step {label}** of part {part}\n\nA line with a number in front is kept as a step and done later, by `Do`. Part {part} has {}: {}.",
                    count(steps.len(), "step"),
                    steps.join(", ")
                );
                let last = a.definition(Named::Step(n));
                if let Some(line) = last.filter(|&line| line != t.line) {
                    text.push_str(&format!(
                        "\n\nStep {label} is typed again on line {}, which replaces this one.",
                        line + 1
                    ));
                }
                text
            }
            _ => "**Step number**\n\nA step number is above zero, and its whole part, which is the part it belongs to, is at least 1."
                .to_string(),
        },
        Kind::Verb | Kind::Noun | Kind::Connective => match word_help(&t.text) {
            Some(help) => entry(&t.text, help),
            None => return Value::Null,
        },
        Kind::Function => match function_help(&t.text) {
            Some((signature, body, example)) => entry(signature, ("function", body, example)),
            None => return Value::Null,
        },
        Kind::Variable if t.text == "$" => entry(
            "$",
            (
                "the line of the page",
                "The number of the line of the page that the typewriter is on.",
                "1.1 Page if $ > 50.",
            ),
        ),
        Kind::Variable => describe_letter(doc, &t.text),
        Kind::Number => match t.names {
            Some(named) => describe(doc, named),
            None => "**Number**\n\nDigits, with at most one point. JOSS keeps nine significant digits, and a longer numeral is refused: \"Please limit numbers to 9 significant digits.\" A numeral has no sign and no power of ten of its own: write `-2.5·10*(-8)`."
                .to_string(),
        },
        Kind::Text if in_form => "**Words of a form**\n\nEverything in a form that is not a blank is typed as it stands.".to_string(),
        Kind::Text => "**Text**\n\n`Type \"...\".` types what is between the quotes, as it stands.\n\n```joss\n1.5 Type \"Root\".\n```"
            .to_string(),
        Kind::Field if in_form => describe_field(doc, t),
        Kind::Field => entry(
            "_",
            (
                "a blank line",
                "In the list after `Type`, an underscore asks for a blank line.",
                "Type a, _, b.",
            ),
        ),
        Kind::Operator | Kind::Punctuation => match symbol_help(&t.text) {
            Some(help) => {
                let mut text = entry(&t.text, help);
                let typed = doc.typed(t);
                if typed != t.text {
                    text.push_str(&format!(
                        "\n\nTyped here as `{typed}`, which stands in for `{}`. JOSS always types `{}`.",
                        t.text, t.text
                    ));
                }
                text
            }
            None => return Value::Null,
        },
        Kind::Unknown => "This is not JOSS. A word of two or more letters must be one of the words of the language: a name is a single letter.".to_string(),
    };
    json!({ "contents": { "kind": "markdown", "value": text }, "range": doc.token_range(t) })
}

fn definition(doc: &Doc, uri: &str, position: &Value) -> Value {
    let Some(t) = doc.token_at(position) else {
        return Value::Null;
    };
    let a = &doc.analysis;
    if t.kind == Kind::Variable {
        let found: Vec<Value> = a
            .letter_uses(&t.text)
            .into_iter()
            .filter(|u| u.defines)
            .map(|u| json!({ "uri": uri, "range": doc.token_range(u) }))
            .collect();
        return if found.is_empty() { Value::Null } else { json!(found) };
    }
    let Some(named) = doc.named(t) else {
        return Value::Null;
    };
    let Some(line) = a.definition(named) else {
        return Value::Null;
    };
    let range = match named {
        // The heading `Form n:` is the line above the picture.
        Named::Form(_) => doc.line_range(line.saturating_sub(1)),
        _ => match doc.step_number(line) {
            Some(number) => doc.token_range(number),
            None => doc.line_range(line),
        },
    };
    json!({ "uri": uri, "range": range })
}

/// The tokens that mean the same thing as the one under the cursor, each
/// with whether it defines the thing or only mentions it.
fn related<'a>(doc: &'a Doc, position: &Value) -> Vec<(&'a Token, bool)> {
    let a = &doc.analysis;
    let Some(t) = doc.token_at(position) else {
        return vec![];
    };
    if t.kind == Kind::Variable {
        return a.letter_uses(&t.text).into_iter().map(|u| (u, u.defines)).collect();
    }
    let Some(named) = doc.named(t) else {
        return vec![];
    };
    let mut found: Vec<(&Token, bool)> = a
        .mentions(named)
        .into_iter()
        .map(|m| (m, matches!(doc.kind(m.line), LineKind::FormHead(_))))
        .collect();
    // The numbers in front of the steps themselves.
    let lines: Vec<usize> = match named {
        Named::Step(n) => a.steps.get(&n).cloned().unwrap_or_default(),
        Named::Part(p) => a
            .steps
            .iter()
            .filter(|(n, _)| n.ip().to_i64() == Some(p))
            .flat_map(|(_, lines)| lines.iter().copied())
            .collect(),
        Named::Form(_) => vec![],
    };
    found.extend(
        lines
            .into_iter()
            .filter_map(|line| doc.step_number(line))
            .map(|n| (n, true)),
    );
    found.sort_by_key(|(t, _)| (t.line, t.start));
    found
}

fn is_letter(t: &Token) -> bool {
    t.kind == Kind::Variable && t.text != "$"
}

fn prepare_rename(doc: &Doc, position: &Value) -> Value {
    match doc.token_at(position) {
        Some(t) if is_letter(t) => json!({ "range": doc.token_range(t), "placeholder": t.text }),
        _ => Value::Null,
    }
}

fn rename(doc: &Doc, uri: &str, position: &Value, new_name: &str) -> Result<Value, String> {
    let a = &doc.analysis;
    let Some(t) = doc.token_at(position).filter(|t| is_letter(t)) else {
        return Err("only a letter can be renamed: steps, parts and forms are known by their numbers".into());
    };
    let mut letters = new_name.chars();
    let is_name = matches!((letters.next(), letters.next()), (Some(c), None) if c.is_ascii_alphabetic());
    if !is_name {
        return Err(format!(
            "{new_name} is not a JOSS name: a name is one letter, a to z or A to Z"
        ));
    }
    if new_name != t.text && !a.letter_uses(new_name).is_empty() {
        return Err(format!("the letter {new_name} is already used in this file"));
    }
    // A letter never stands against another letter (two together are a
    // word), so one letter can take the place of another as it stands.
    let edits: Vec<Value> = a
        .letter_uses(&t.text)
        .into_iter()
        .map(|u| json!({ "range": doc.token_range(u), "newText": new_name }))
        .collect();
    Ok(json!({ "changes": { uri: edits } }))
}

fn item(label: &str, kind: u32, detail: &str, documentation: &str, order: &str) -> Value {
    json!({
        "label": label,
        "kind": kind,
        "detail": detail,
        "documentation": documentation,
        "sortText": format!("{order}{label}"),
    })
}

fn word_item(word: &str, order: &str) -> Value {
    let (heading, body, _) = word_help(word).unwrap_or(("", "", ""));
    item(word, 14, heading, body, order)
}

fn completion(doc: &Doc, position: &Value) -> Value {
    let a = &doc.analysis;
    let (line, column) = doc.place(position);
    let before = doc.raw(line, 0, column);
    let first = before.trim_start().chars().next();
    // A note, the picture of a form, or the inside of a text: nothing to offer.
    let quoted = before.chars().filter(|c| *c == '"').count() % 2 == 1;
    if first == Some('*') || matches!(doc.kind(line), LineKind::FormBody(_)) || quoted {
        return json!([]);
    }
    // A stand-in has just been typed: offer the character it stands for.
    for (real, stand_in, meaning) in SPECIALS {
        if before.ends_with(stand_in) {
            let start = column - stand_in.chars().count();
            return json!([{
                "label": real,
                "kind": 24,
                "detail": format!("{meaning} (typed as {stand_in})"),
                "filterText": stand_in,
                "textEdit": { "range": doc.range(line, start, column), "newText": real },
                "preselect": true,
            }]);
        }
    }

    let mut tokens = lex::lex(&before);
    // The word being typed is not yet part of the line.
    if matches!(tokens.last(), Some(t) if t.end == before.chars().count() && matches!(t.tok, Tok::Word(_) | Tok::Letter(_)))
    {
        tokens.pop();
    }
    let numbered = matches!(tokens.first(), Some(t) if matches!(t.tok, Tok::Number(_)));
    let rest = &tokens[usize::from(numbered)..];

    let mut items = Vec::new();
    let letters = |items: &mut Vec<Value>| {
        let mut seen: Vec<&str> = Vec::new();
        for t in a.tokens.iter().filter(|t| is_letter(t)) {
            if seen.contains(&t.text.as_str()) {
                continue;
            }
            seen.push(&t.text);
            let given = a.letter_uses(&t.text).into_iter().find(|u| u.defines);
            let detail = match given {
                Some(u) => format!("line {}: {}", u.line + 1, doc.line_text(u.line)),
                None => "never given a value".to_string(),
            };
            items.push(item(&t.text, 6, &detail, "", "0"));
        }
    };
    let functions = |items: &mut Vec<Value>| {
        for (name, signature, body, _) in FUNCTIONS_HELP {
            items.push(item(name, 3, signature, body, "1"));
        }
    };
    let specials = |items: &mut Vec<Value>| {
        for (real, stand_in, meaning) in SPECIALS {
            let mut special = item(real, 24, &format!("{meaning} (typed as {stand_in})"), "", "9");
            special["filterText"] = json!(stand_in);
            items.push(special);
        }
    };
    let words = |items: &mut Vec<Value>, list: &[&str], order: &str| {
        for word in list {
            items.push(word_item(word, order));
        }
    };

    let last = rest.last().map(|t| &t.tok);
    match last {
        // The start of a line, or of a step: a verb.
        None => {
            for verb in VERBS {
                let direct_only = matches!(*verb, "Cancel" | "Go" | "Form");
                let step_only = matches!(*verb, "To" | "Done" | "Stop" | "Demand");
                if (numbered && direct_only) || (!numbered && step_only) {
                    continue;
                }
                items.push(word_item(verb, "2"));
            }
            for (label, detail, insert) in TEMPLATES {
                if numbered && *label == "Form" {
                    continue;
                }
                items.push(json!({
                    "label": label,
                    "kind": 15,
                    "detail": detail,
                    "insertText": insert,
                    "insertTextFormat": 2,
                    "sortText": format!("3{label}"),
                }));
            }
        }
        Some(Tok::Word(word)) => match word.as_str() {
            "Do" | "To" => words(&mut items, &["step", "part"], "2"),
            "Type" => {
                words(
                    &mut items,
                    &["step", "part", "form", "all", "size", "time", "users"],
                    "2",
                );
                letters(&mut items);
                functions(&mut items);
            }
            "Delete" => {
                words(&mut items, &["step", "part", "form", "all"], "2");
                letters(&mut items);
            }
            "all" => words(&mut items, &["steps", "parts", "forms", "values"], "2"),
            "in" => words(&mut items, &["form"], "2"),
            "Set" | "Demand" | "for" => letters(&mut items),
            "step" => {
                for (n, lines) in &a.steps {
                    let line = lines.last().copied().unwrap_or(0);
                    items.push(item(&format::plain(*n), 18, &doc.line_text(line), "", "0"));
                }
            }
            "part" => {
                for p in a.parts() {
                    let steps = a.part_lines(p);
                    let detail = format!("{}: {}", count(steps.len(), "step"), doc.line_text(steps[0]));
                    items.push(item(&p.to_string(), 18, &detail, "", "0"));
                }
            }
            "form" | "Form" => {
                for (n, lines) in &a.forms {
                    let line = lines.last().copied().unwrap_or(0);
                    items.push(item(&n.to_string(), 18, &doc.raw(line, 0, usize::MAX), "", "0"));
                }
            }
            _ => {}
        },
        // The instruction is over.
        Some(Tok::Period) => {}
        // Anywhere else: an expression, or the clause after it.
        Some(_) => {
            letters(&mut items);
            functions(&mut items);
            words(&mut items, CONNECTIVES, "2");
            specials(&mut items);
        }
    }
    json!(items)
}

fn signature_help(doc: &Doc, position: &Value) -> Value {
    let (line, column) = doc.place(position);
    // The brackets still open at the cursor, innermost last: the token
    // before each, and the commas seen in it so far.
    let mut open: Vec<(Option<&Token>, usize)> = Vec::new();
    let mut previous: Option<&Token> = None;
    for t in doc.analysis.tokens.iter().filter(|t| t.line == line && t.end <= column) {
        match (t.kind, t.text.as_str()) {
            (Kind::Punctuation, "(" | "[") => open.push((previous, 0)),
            (Kind::Punctuation, ")" | "]") => {
                open.pop();
            }
            (Kind::Punctuation, ",") => {
                if let Some(top) = open.last_mut() {
                    top.1 += 1;
                }
            }
            _ => {}
        }
        previous = Some(t);
    }
    let Some(&(Some(callee), commas)) = open.last() else {
        return Value::Null;
    };
    if callee.kind != Kind::Function {
        return Value::Null;
    }
    let Some((label, body, _)) = function_help(&callee.text) else {
        return Value::Null;
    };
    // Where each parameter is in the label.
    let inside = label.find('(').map_or(label.len(), |at| at + 1);
    let mut parameters = Vec::new();
    let mut at = inside;
    for part in label[inside..label.len() - 1].split(", ") {
        parameters.push(json!({ "label": [at, at + part.len()] }));
        at += part.len() + 2;
    }
    let active = commas.min(parameters.len().saturating_sub(1));
    json!({
        "signatures": [{ "label": label, "documentation": body, "parameters": parameters }],
        "activeSignature": 0,
        "activeParameter": active,
    })
}

fn symbols(doc: &Doc) -> Value {
    let a = &doc.analysis;
    let mut top: Vec<(usize, Value)> = Vec::new();
    for part in a.parts() {
        let lines = a.part_lines(part);
        let (Some(&first), Some(&last)) = (lines.iter().min(), lines.iter().max()) else {
            continue;
        };
        let children: Vec<Value> = lines
            .iter()
            .map(|&line| {
                let number = doc.step_number(line);
                let label = number.map_or(String::new(), |n| doc.typed(n));
                let after = number.map_or(0, |n| n.end);
                json!({
                    "name": format!("Step {label}"),
                    "detail": doc.raw(line, after, usize::MAX).trim(),
                    "kind": 6,
                    "range": doc.line_range(line),
                    "selectionRange": number.map_or(doc.line_range(line), |n| doc.token_range(n)),
                })
            })
            .collect();
        let selection = doc
            .step_number(lines[0])
            .map_or(doc.line_range(lines[0]), |n| doc.token_range(n));
        top.push((
            first,
            json!({
                "name": format!("Part {part}"),
                "detail": count(lines.len(), "step"),
                "kind": 2,
                "range": {
                    "start": { "line": first, "character": 0 },
                    "end": { "line": last, "character": doc.units(last, doc.width(last)) },
                },
                "selectionRange": selection,
                "children": children,
            }),
        ));
    }
    for (n, lines) in &a.forms {
        let Some(&body) = lines.last() else { continue };
        let head = body.saturating_sub(1);
        top.push((
            head,
            json!({
                "name": format!("Form {n}"),
                "detail": doc.line_text(body),
                "kind": 23,
                "range": {
                    "start": { "line": head, "character": 0 },
                    "end": { "line": body, "character": doc.units(body, doc.width(body)) },
                },
                "selectionRange": doc.line_range(head),
                "children": [],
            }),
        ));
    }
    top.sort_by_key(|(line, _)| *line);
    json!(top.into_iter().map(|(_, symbol)| symbol).collect::<Vec<_>>())
}

fn folding(doc: &Doc) -> Value {
    let a = &doc.analysis;
    let mut out = Vec::new();
    let mut fold = |first: usize, last: usize, kind: Option<&str>| {
        if last > first {
            let mut range = json!({ "startLine": first, "endLine": last });
            if let Some(kind) = kind {
                range["kind"] = json!(kind);
            }
            out.push(range);
        }
    };
    // Each part: its steps as they stand together in the file. Notes and
    // empty lines between two steps of a part do not break it.
    let mut part: Option<(i64, usize, usize)> = None;
    // Runs of notes.
    let mut notes: Option<(usize, usize)> = None;
    for (line, info) in a.lines.iter().enumerate() {
        match info.kind {
            LineKind::Step { part: p, .. } => match &mut part {
                Some((open, _, last)) if *open == p => *last = line,
                _ => {
                    if let Some((_, first, last)) = part.replace((p, line, line)) {
                        fold(first, last, None);
                    }
                }
            },
            LineKind::Empty | LineKind::Comment => {}
            _ => {
                if let Some((_, first, last)) = part.take() {
                    fold(first, last, None);
                }
            }
        }
        if info.kind == LineKind::Comment {
            notes = Some((notes.map_or(line, |(first, _)| first), line));
        } else if let Some((first, last)) = notes.take() {
            fold(first, last, Some("comment"));
        }
        if let LineKind::FormBody(_) = info.kind {
            fold(line.saturating_sub(1), line, None);
        }
    }
    if let Some((_, first, last)) = part {
        fold(first, last, None);
    }
    if let Some((first, last)) = notes {
        fold(first, last, Some("comment"));
    }
    json!(out)
}

/// The semantic type and modifiers of a token, or nothing for punctuation
/// and for text that means nothing.
fn semantic(doc: &Doc, t: &Token) -> Option<(&'static str, u32)> {
    const DEFINITION: u32 = 1;
    const LIBRARY: u32 = 2;
    Some(match t.kind {
        Kind::Comment => ("comment", 0),
        Kind::StepNumber => ("label", DEFINITION),
        Kind::Verb | Kind::Connective => ("keyword", 0),
        Kind::Noun => ("type", 0),
        Kind::Function => ("function", LIBRARY),
        Kind::Variable if t.text == "$" => ("variable", LIBRARY),
        Kind::Variable if t.defines => ("variable", DEFINITION),
        Kind::Variable => ("variable", 0),
        Kind::Number if t.names.is_some() => {
            let heading = matches!(doc.kind(t.line), LineKind::FormHead(_));
            ("label", if heading { DEFINITION } else { 0 })
        }
        Kind::Number => ("number", 0),
        Kind::Text => ("string", 0),
        Kind::Field => ("parameter", 0),
        Kind::Operator => ("operator", 0),
        Kind::Punctuation | Kind::Unknown => return None,
    })
}

/// Semantic tokens, in the protocol's five-numbers-per-token encoding.
fn semantic_tokens(doc: &Doc) -> Vec<u32> {
    let type_index = |name: &str| SEMANTIC_TYPES.iter().position(|&t| t == name).unwrap() as u32;
    let mut tokens: Vec<&Token> = doc.analysis.tokens.iter().collect();
    tokens.sort_by_key(|t| (t.line, t.start));
    let mut data = Vec::new();
    let (mut last_line, mut last_start) = (0u32, 0u32);
    for t in tokens {
        let Some((kind, modifiers)) = semantic(doc, t) else {
            continue;
        };
        let start = doc.units(t.line, t.start) as u32;
        let length = doc.units(t.line, t.end) as u32 - start;
        if length == 0 {
            continue;
        }
        let line = t.line as u32;
        let delta_line = line - last_line;
        let delta_start = if delta_line == 0 { start - last_start } else { start };
        data.extend([delta_line, delta_start, length, type_index(kind), modifiers]);
        (last_line, last_start) = (line, start);
    }
    data
}

/// The edits that put the characters of the JOSS typewriter where a stand-in
/// was typed: `·` for `@`, `≠` for `!=`, `≤` for `<=`, `≥` for `>=`. Text in
/// quotes, notes and forms are left alone. With lines given, only the
/// stand-ins on those lines.
fn real_characters(doc: &Doc, lines: Option<(usize, usize)>) -> Vec<Value> {
    doc.stand_ins()
        .into_iter()
        .filter(|t| lines.is_none_or(|(first, last)| (first..=last).contains(&t.line)))
        .map(|t| json!({ "range": doc.token_range(t), "newText": t.text }))
        .collect()
}

fn code_actions(doc: &Doc, uri: &str, range: &Value) -> Value {
    let all = real_characters(doc, None);
    if all.is_empty() {
        return json!([]);
    }
    let first = range["start"]["line"].as_u64().unwrap_or(0) as usize;
    let last = range["end"]["line"].as_u64().map_or(first, |line| line as usize);
    let mut actions = Vec::new();
    for t in doc.stand_ins().into_iter().filter(|t| (first..=last).contains(&t.line)) {
        actions.push(json!({
            "title": format!("Write {} for {}", t.text, doc.typed(t)),
            "kind": "quickfix",
            "edit": { "changes": { uri: [{ "range": doc.token_range(t), "newText": t.text }] } },
        }));
    }
    actions.push(json!({
        "title": "Write the JOSS characters · ≠ ≤ ≥ for every stand-in",
        "kind": "source.fixAll",
        "edit": { "changes": { uri: all } },
    }));
    json!(actions)
}

fn read_message(input: &mut impl BufRead) -> io::Result<Option<Value>> {
    let mut length = None;
    loop {
        let mut line = String::new();
        if input.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        let line = line.trim_end();
        if line.is_empty() {
            if length.is_some() {
                break;
            }
            continue;
        }
        if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
            length = value.trim().parse::<usize>().ok();
        }
    }
    let mut body = vec![0; length.unwrap()];
    input.read_exact(&mut body)?;
    serde_json::from_slice(&body)
        .map(Some)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn main() -> io::Result<()> {
    if std::env::args().any(|arg| arg == "--version") {
        println!("joss-lsp {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut output = io::stdout().lock();
    let mut server = Server::new();
    while let Some(message) = read_message(&mut input)? {
        for reply in server.handle(&message) {
            let body = reply.to_string();
            write!(output, "Content-Length: {}\r\n\r\n{}", body.len(), body)?;
        }
        output.flush()?;
        if server.exit {
            break;
        }
    }
    std::process::exit(if server.shutdown { 0 } else { 1 });
}

#[cfg(test)]
mod tests {
    use super::*;
    use joss::lex::{FUNCTIONS, NOUNS};

    const URI: &str = "file:///test.joss";
    /// The hypotenuse program of P-3131 (28 April 1965), with a condition
    /// added that uses the stand-ins.
    const PROGRAM: &str = "\
*  Stored program for computing the hypotenuse:
1.1 Set c=sqrt(a*2+b*2).
1.2 Type a,b,c in form 1 if fp(c) != 0 or c >= 5.
Form 1:
a = __   b = __   c = ___.______
2.1 Do part 1 for b=1(1)a.
2.2 Line.
Do part 2 for a=1(1)4.
";

    fn server_with(text: &str) -> (Server, Vec<Value>) {
        let mut server = Server::new();
        let out = server.handle(&json!({ "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": { "textDocument": { "uri": URI, "languageId": "joss", "version": 1, "text": text } } }));
        (server, out)
    }

    fn ask(server: &mut Server, method: &str, line: u32, character: u32, extra: Value) -> Value {
        let mut params =
            json!({ "textDocument": { "uri": URI }, "position": { "line": line, "character": character } });
        for (key, value) in extra.as_object().into_iter().flatten() {
            params[key] = value.clone();
        }
        let mut out = server.handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params }));
        assert_eq!(out.len(), 1);
        out.remove(0)
    }

    fn hover_text(server: &mut Server, line: u32, character: u32) -> String {
        let reply = ask(server, "textDocument/hover", line, character, json!({}));
        reply["result"]["contents"]["value"].as_str().unwrap_or("").to_string()
    }

    fn labels(reply: &Value) -> Vec<&str> {
        let items = reply["result"].as_array().unwrap();
        items.iter().map(|i| i["label"].as_str().unwrap()).collect()
    }

    fn range(line: u32, start: u32, end: u32) -> Value {
        json!({ "start": { "line": line, "character": start }, "end": { "line": line, "character": end } })
    }

    #[test]
    fn initialise_advertises_what_is_implemented() {
        let mut server = Server::new();
        let out = server.handle(&json!({ "jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {} }));
        let capabilities = &out[0]["result"]["capabilities"];
        assert_eq!(capabilities["hoverProvider"], true);
        assert_eq!(capabilities["renameProvider"]["prepareProvider"], true);
        assert_eq!(capabilities["documentFormattingProvider"], true);
        assert_eq!(
            capabilities["semanticTokensProvider"]["legend"]["tokenTypes"][4],
            "function"
        );
        assert_eq!(out[0]["result"]["serverInfo"]["name"], "joss-lsp");
    }

    #[test]
    fn a_good_program_publishes_no_diagnostics_and_a_bad_one_does() {
        let (_, out) = server_with(PROGRAM);
        assert_eq!(out[0]["method"], "textDocument/publishDiagnostics");
        assert_eq!(out[0]["params"]["diagnostics"], json!([]));

        let (_, out) = server_with("Set x=1.\nType x+.\n");
        let diagnostics = out[0]["params"]["diagnostics"].as_array().unwrap();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0]["severity"], 1);
        assert_eq!(diagnostics[0]["code"], "eh");
        assert_eq!(diagnostics[0]["source"], "joss");
        assert_eq!(diagnostics[0]["message"], "Eh? This line is not JOSS, from here on.");
        assert_eq!(diagnostics[0]["range"], range(1, 7, 8));

        // A step that a later line replaces is dead text.
        let (_, out) = server_with("Set x=1.\n1.1 Type x.\n1.1 Type x+1.\n");
        let hint = &out[0]["params"]["diagnostics"][0];
        assert_eq!(
            (&hint["severity"], &hint["code"], &hint["tags"]),
            (&json!(4), &json!("replaced"), &json!([1]))
        );
        assert_eq!(hint["range"], range(1, 0, 11));

        let (mut server, _) = server_with("Type 2+2+.\n");
        let out = server.handle(&json!({ "jsonrpc": "2.0", "method": "textDocument/didClose",
            "params": { "textDocument": { "uri": URI } } }));
        assert_eq!(out[0]["params"]["diagnostics"], json!([]));
    }

    #[test]
    fn positions_are_in_utf16_code_units() {
        // The clef is one character to JOSS and two code units to an editor.
        let (mut server, out) = server_with("Set x=1.\nType \"\u{1D11E}\" if x >= y.\n");
        let diagnostics = out[0]["params"]["diagnostics"].as_array().unwrap();
        assert_eq!(diagnostics[0]["code"], "never-set");
        assert_eq!(diagnostics[0]["range"], range(1, 18, 19));
        let ge = ask(&mut server, "textDocument/hover", 1, 16, json!({}));
        assert_eq!(ge["result"]["range"], range(1, 15, 17));
        assert!(hover_text(&mut server, 1, 13).starts_with("**Letter `x`**"));
        let tokens = ask(&mut server, "textDocument/semanticTokens/full", 0, 0, json!({}));
        let data: Vec<u64> = tokens["result"]["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n.as_u64().unwrap())
            .collect();
        // The text on line 1: starts at 5 and is four units long.
        assert!(data.chunks(5).any(|t| t == [0, 5, 4, 1, 0]), "{data:?}");
    }

    #[test]
    fn hover_explains_every_word_of_the_language() {
        for word in VERBS.iter().chain(NOUNS).chain(CONNECTIVES) {
            let (heading, body, example) = word_help(word).unwrap_or_else(|| panic!("nothing about {word}"));
            assert!(!heading.is_empty() && !body.is_empty() && !example.is_empty(), "{word}");
        }
        for name in FUNCTIONS {
            let (signature, _, example) = function_help(name).unwrap_or_else(|| panic!("nothing about {name}"));
            assert!(signature.starts_with(&format!("{name}(")));
            assert!(example.contains(&format!("{name}(")), "{name}");
        }
        for symbol in [
            "+", "-", "·", "/", "*", "=", "≠", "≤", "≥", "<", ">", "(", ")", "[", "]", "|", ".", ",", ":",
        ] {
            assert!(symbol_help(symbol).is_some(), "{symbol}");
        }
    }

    #[test]
    fn every_example_in_the_help_is_joss() {
        // Each example is given to the analysis. A line that is not JOSS
        // would be reported as "eh", and so would one of the wrong kind
        // (a `To` given directly).
        let mut examples: Vec<(&str, &str)> = Vec::new();
        for word in VERBS.iter().chain(NOUNS).chain(CONNECTIVES) {
            examples.push((word, word_help(word).unwrap().2));
        }
        for (name, _, _, example) in FUNCTIONS_HELP {
            examples.push((name, example));
        }
        for symbol in [
            "+", "-", "·", "/", "*", "=", "≠", "≤", "≥", "<", ">", "(", "|", ".", ",", ":",
        ] {
            examples.push((symbol, symbol_help(symbol).unwrap().2));
        }
        for (what, example) in examples {
            let found = analyse(example);
            let wrong: Vec<_> = found
                .diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Error)
                .collect();
            assert!(wrong.is_empty(), "the example for {what} is not JOSS: {wrong:?}");
            assert!(example.contains(what) || what.len() > 1, "{what}");
        }
    }

    #[test]
    fn hover_explains_words_functions_and_symbols() {
        let (mut server, _) = server_with(PROGRAM);
        let set = hover_text(&mut server, 1, 5);
        assert!(set.starts_with("**`Set`** gives a letter a value"), "{set}");
        assert!(set.contains("```joss\nSet x = 3."), "{set}");
        let sqrt = hover_text(&mut server, 1, 11);
        assert!(sqrt.starts_with("**`sqrt(x)`** function\n\nThe square root"), "{sqrt}");
        let power = hover_text(&mut server, 1, 16);
        assert!(
            power.starts_with("**`*`** to the power") && power.contains("not times"),
            "{power}"
        );
        assert!(hover_text(&mut server, 2, 15).starts_with("**`in`** into a form"));
        assert!(hover_text(&mut server, 2, 19).starts_with("**`form`** a picture of a line of output"));
        assert!(hover_text(&mut server, 2, 25).starts_with("**`if`** a condition"));
        assert!(hover_text(&mut server, 2, 39).starts_with("**`or`** either condition holds"));
        assert!(hover_text(&mut server, 5, 14).starts_with("**`for`** a range of values"));
        assert!(hover_text(&mut server, 6, 5).starts_with("**`Line`** types a blank line"));
        assert!(hover_text(&mut server, 5, 21).starts_with("**`(`** brackets"));
        assert_eq!(hover_text(&mut server, 0, 5), "", "a note has nothing to explain");

        // A stand-in is explained as the character it stands for.
        let ne = hover_text(&mut server, 2, 34);
        assert!(ne.starts_with("**`≠`** is not equal to"), "{ne}");
        assert!(ne.contains("Typed here as `!=`"), "{ne}");
        let reply = ask(&mut server, "textDocument/hover", 2, 45, json!({}));
        assert_eq!(reply["result"]["range"], range(2, 44, 46));
        assert!(reply["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .starts_with("**`≥`** is at least"));
    }

    #[test]
    fn hover_shows_what_a_number_or_a_letter_refers_to() {
        let (mut server, _) = server_with(PROGRAM);
        // "part 1" shows the steps of the part.
        let part = hover_text(&mut server, 5, 12);
        assert!(part.starts_with("**Part 1** 2 steps"), "{part}");
        assert!(
            part.contains("1.1 Set c=sqrt(a*2+b*2).\n1.2 Type a,b,c in form 1"),
            "{part}"
        );
        // "form 1" shows the picture.
        let form = hover_text(&mut server, 2, 23);
        assert!(form.starts_with("**Form 1**"), "{form}");
        assert!(form.contains("Form 1:\na = __   b = __   c = ___.______\n"), "{form}");
        assert!(form.contains("It has 3 blanks."), "{form}");
        // The number in front of a step says which part it is in.
        let step = hover_text(&mut server, 6, 1);
        assert!(step.starts_with("**Step 2.2** of part 2"), "{step}");
        assert!(step.contains("Part 2 has 2 steps: 2.1, 2.2."), "{step}");
        // A letter shows where it is given a value.
        let c = hover_text(&mut server, 2, 13);
        assert!(c.starts_with("**Letter `c`**"), "{c}");
        assert!(c.contains("It is given a value on line 2."), "{c}");
        assert!(c.contains("1.1 Set c=sqrt(a*2+b*2)."), "{c}");
        assert!(c.contains("It is written 4 times."), "{c}");
        let a = hover_text(&mut server, 1, 15);
        assert!(a.contains("on line 8.") && a.contains("Do part 2 for a=1(1)4."), "{a}");

        // The blanks of a form.
        let fixed = hover_text(&mut server, 4, 30);
        assert!(
            fixed.starts_with("**Fixed blank** 10 columns wide, 6 decimals"),
            "{fixed}"
        );
        let whole = hover_text(&mut server, 4, 4);
        assert!(
            whole.starts_with("**Fixed blank** 2 columns wide, no decimal point"),
            "{whole}"
        );
        assert!(hover_text(&mut server, 4, 0).starts_with("**Words of a form**"));

        let (mut server, _) =
            server_with("Set x=1.\nForm 2:\nx = ........... and -_.___\nType step 3.1.\nType x, _, $.\n");
        let scientific = hover_text(&mut server, 2, 6);
        assert!(
            scientific.starts_with("**Scientific blank** 11 columns wide, 5 decimals"),
            "{scientific}"
        );
        let signed = hover_text(&mut server, 2, 21);
        assert!(
            signed.starts_with("**Fixed blank** 6 columns wide, 3 decimals"),
            "{signed}"
        );
        assert!(signed.contains("keeps a column for the sign"), "{signed}");
        assert!(hover_text(&mut server, 3, 11).contains("There is no step 3.1 in this file."));
        assert!(hover_text(&mut server, 4, 8).starts_with("**`_`** a blank line"));
        assert!(hover_text(&mut server, 4, 11).starts_with("**`$`** the line of the page"));
    }

    #[test]
    fn definition_goes_to_the_step_the_form_or_the_setting_of_a_letter() {
        let (mut server, _) = server_with(PROGRAM);
        // "part 1" in the Do: the first step of part 1.
        let part = ask(&mut server, "textDocument/definition", 5, 12, json!({}));
        assert_eq!(part["result"]["range"], range(1, 0, 3));
        // "form 1": the heading of the form.
        let form = ask(&mut server, "textDocument/definition", 2, 23, json!({}));
        assert_eq!(form["result"]["range"], range(3, 0, 7));
        // A letter: everywhere it is given a value.
        let b = ask(&mut server, "textDocument/definition", 1, 19, json!({}));
        assert_eq!(b["result"], json!([{ "uri": URI, "range": range(5, 18, 19) }]));
        // A word has no definition.
        let word = ask(&mut server, "textDocument/definition", 1, 5, json!({}));
        assert_eq!(word["result"], Value::Null);

        let (mut server, _) = server_with("1.1 Demand x.\n1.2 To step 1.1 if x < 0.\n1.1 Demand y.\nSet x = 0.\n");
        let step = ask(&mut server, "textDocument/definition", 1, 13, json!({}));
        assert_eq!(
            step["result"]["range"],
            range(2, 0, 3),
            "the step as it stands at the end"
        );
        let x = ask(&mut server, "textDocument/definition", 1, 19, json!({}));
        assert_eq!(x["result"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn references_and_highlights_find_letters_and_mentions() {
        let (mut server, _) = server_with(PROGRAM);
        let all = ask(&mut server, "textDocument/references", 1, 8, json!({}));
        assert_eq!(all["result"].as_array().unwrap().len(), 4, "every c");
        let uses = ask(
            &mut server,
            "textDocument/references",
            1,
            8,
            json!({ "context": { "includeDeclaration": false } }),
        );
        assert_eq!(uses["result"].as_array().unwrap().len(), 3);
        // Part 1: its two step numbers and the Do that names it.
        let part = ask(&mut server, "textDocument/references", 5, 12, json!({}));
        let ranges: Vec<&Value> = part["result"].as_array().unwrap().iter().map(|r| &r["range"]).collect();
        assert_eq!(ranges, [&range(1, 0, 3), &range(2, 0, 3), &range(5, 12, 13)]);
        // From the step number itself, a step: only itself, nothing names it.
        let step = ask(&mut server, "textDocument/references", 1, 1, json!({}));
        assert_eq!(step["result"].as_array().unwrap().len(), 1);
        // Form 1: the heading and the Type.
        let highlights = ask(&mut server, "textDocument/documentHighlight", 3, 5, json!({}));
        assert_eq!(
            highlights["result"],
            json!([{ "range": range(2, 23, 24), "kind": 2 }, { "range": range(3, 5, 6), "kind": 3 }])
        );
        let letter = ask(&mut server, "textDocument/documentHighlight", 5, 18, json!({}));
        assert_eq!(
            letter["result"],
            json!([
                { "range": range(1, 19, 20), "kind": 2 },
                { "range": range(2, 11, 12), "kind": 2 },
                { "range": range(5, 18, 19), "kind": 3 },
            ])
        );
    }

    #[test]
    fn rename_changes_a_letter_and_refuses_what_is_not_one() {
        let (mut server, _) = server_with(PROGRAM);
        let prepared = ask(&mut server, "textDocument/prepareRename", 1, 8, json!({}));
        assert_eq!(
            prepared["result"],
            json!({ "range": range(1, 8, 9), "placeholder": "c" })
        );
        let word = ask(&mut server, "textDocument/prepareRename", 1, 5, json!({}));
        assert_eq!(word["result"], Value::Null);

        let renamed = ask(&mut server, "textDocument/rename", 1, 8, json!({ "newName": "h" }));
        let edits = renamed["result"]["changes"][URI].as_array().unwrap();
        assert_eq!(edits.len(), 4);
        assert!(edits.iter().all(|e| e["newText"] == "h"));
        assert_eq!(edits[1]["range"], range(2, 13, 14));
        // A capital is another name.
        let capital = ask(&mut server, "textDocument/rename", 1, 8, json!({ "newName": "C" }));
        assert_eq!(capital["result"]["changes"][URI].as_array().unwrap().len(), 4);

        let clash = ask(&mut server, "textDocument/rename", 1, 8, json!({ "newName": "a" }));
        assert_eq!(clash["error"]["message"], "the letter a is already used in this file");
        let long = ask(&mut server, "textDocument/rename", 1, 8, json!({ "newName": "side" }));
        assert!(long["error"]["message"]
            .as_str()
            .unwrap()
            .starts_with("side is not a JOSS name"));
        let digit = ask(&mut server, "textDocument/rename", 1, 8, json!({ "newName": "7" }));
        assert!(digit["error"].is_object());
        let number = ask(&mut server, "textDocument/rename", 5, 12, json!({ "newName": "3" }));
        assert!(number["error"]["message"]
            .as_str()
            .unwrap()
            .starts_with("only a letter can be renamed"));
    }

    #[test]
    fn completion_depends_on_where_the_cursor_is() {
        let text = format!("{PROGRAM}\n3.1 \nDo \nType \nDelete all \nDo part \nType step \nType a in form \nSet x = 2+\nType \"a \n* \nSet x = 1. \nSet y = x @\nType x if x <=\nDo part 1 f\n");
        let (mut server, _) = server_with(&text);
        let at = |server: &mut Server, line: u32, character: u32| {
            ask(server, "textDocument/completion", line, character, json!({}))
        };

        // The start of a line: the verbs that may be given directly.
        let start = at(&mut server, 8, 0);
        let verbs = labels(&start);
        for wanted in ["Set", "Do", "Type", "Delete", "Form", "Go", "Cancel", "Do part for"] {
            assert!(verbs.contains(&wanted), "{wanted}");
        }
        for step_only in ["To", "Done", "Stop", "Demand", "sqrt", "part"] {
            assert!(!verbs.contains(&step_only), "{step_only}");
        }
        // After a step number: the verbs that may be kept as steps.
        let step = at(&mut server, 9, 4);
        let verbs = labels(&step);
        for wanted in ["Set", "Do", "Type", "To", "Done", "Stop", "Demand"] {
            assert!(verbs.contains(&wanted), "{wanted}");
        }
        for direct_only in ["Go", "Cancel", "Form"] {
            assert!(!verbs.contains(&direct_only), "{direct_only}");
        }

        assert_eq!(labels(&at(&mut server, 10, 3)), ["step", "part"]);
        let after_type = at(&mut server, 11, 5);
        let offered = labels(&after_type);
        for wanted in [
            "step", "part", "form", "all", "size", "time", "users", "sqrt", "max", "a", "b", "c",
        ] {
            assert!(offered.contains(&wanted), "{wanted}");
        }
        assert_eq!(labels(&at(&mut server, 12, 11)), ["steps", "parts", "forms", "values"]);
        assert_eq!(labels(&at(&mut server, 13, 8)), ["1", "2"]);
        assert_eq!(labels(&at(&mut server, 14, 10)), ["1.1", "1.2", "2.1", "2.2"]);
        let forms = at(&mut server, 15, 15);
        assert_eq!(labels(&forms), ["1"]);
        assert_eq!(forms["result"][0]["detail"], "a = __   b = __   c = ___.______");

        // In an expression: letters, functions, the clause words and the
        // characters that are not on the keyboard.
        let expression = at(&mut server, 16, 10);
        let offered = labels(&expression);
        for wanted in ["a", "x", "sqrt", "arg", "if", "for", "·", "≠", "≤", "≥"] {
            assert!(offered.contains(&wanted), "{wanted}");
        }
        assert!(!offered.contains(&"Set"));
        let times = expression["result"]
            .as_array()
            .unwrap()
            .iter()
            .find(|i| i["label"] == "·")
            .unwrap();
        assert_eq!(times["filterText"], "@");
        let x = expression["result"]
            .as_array()
            .unwrap()
            .iter()
            .find(|i| i["label"] == "x")
            .unwrap();
        assert_eq!(x["detail"], "line 17: Set x = 2+");

        // Nothing inside a text, in a note, or after the full stop.
        assert_eq!(labels(&at(&mut server, 17, 8)), [] as [&str; 0]);
        assert_eq!(labels(&at(&mut server, 18, 2)), [] as [&str; 0]);
        assert_eq!(labels(&at(&mut server, 19, 11)), [] as [&str; 0]);
        assert_eq!(labels(&at(&mut server, 4, 3)), [] as [&str; 0], "the picture of a form");

        // A stand-in just typed is offered as the real character, in its place.
        let typed = at(&mut server, 20, 11);
        assert_eq!(labels(&typed), ["·"]);
        assert_eq!(
            typed["result"][0]["textEdit"],
            json!({ "range": range(20, 10, 11), "newText": "·" })
        );
        let le = at(&mut server, 21, 14);
        assert_eq!(
            le["result"][0]["textEdit"],
            json!({ "range": range(21, 12, 14), "newText": "≤" })
        );

        // A word half typed does not hide what came before it.
        assert!(labels(&at(&mut server, 22, 11)).contains(&"for"));
    }

    #[test]
    fn signature_help_counts_the_arguments() {
        let (mut server, _) =
            server_with("Set x=3.\nType arg(x, 3), max(6·x, 20, x*3, sqrt[x]), ip(x).\nType a(1,2).\n");
        let help = ask(&mut server, "textDocument/signatureHelp", 1, 12, json!({}));
        assert_eq!(help["result"]["signatures"][0]["label"], "arg(x, y)");
        assert_eq!(
            help["result"]["signatures"][0]["parameters"],
            json!([{ "label": [4, 5] }, { "label": [7, 8] }])
        );
        assert_eq!(help["result"]["activeParameter"], 1);
        let first = ask(&mut server, "textDocument/signatureHelp", 1, 9, json!({}));
        assert_eq!(first["result"]["activeParameter"], 0);
        // max takes two or more: past the second, the dots stay lit.
        let third = ask(&mut server, "textDocument/signatureHelp", 1, 31, json!({}));
        assert_eq!(third["result"]["signatures"][0]["label"], "max(a, b, ...)");
        assert_eq!(third["result"]["activeParameter"], 2);
        let fourth = ask(&mut server, "textDocument/signatureHelp", 1, 35, json!({}));
        assert_eq!(fourth["result"]["activeParameter"], 2);
        // Square brackets do as well as round, and the inner call wins.
        let inner = ask(&mut server, "textDocument/signatureHelp", 1, 39, json!({}));
        assert_eq!(inner["result"]["signatures"][0]["label"], "sqrt(x)");
        // The indices of a letter are not arguments, and nor is the end of a line.
        let index = ask(&mut server, "textDocument/signatureHelp", 2, 9, json!({}));
        assert_eq!(index["result"], Value::Null);
        let outside = ask(&mut server, "textDocument/signatureHelp", 1, 52, json!({}));
        assert_eq!(outside["result"], Value::Null);
    }

    #[test]
    fn symbols_list_parts_with_their_steps_and_forms() {
        let (mut server, _) = server_with(PROGRAM);
        let reply = ask(&mut server, "textDocument/documentSymbol", 0, 0, json!({}));
        let symbols = reply["result"].as_array().unwrap();
        let names: Vec<&str> = symbols.iter().map(|s| s["name"].as_str().unwrap()).collect();
        assert_eq!(names, ["Part 1", "Form 1", "Part 2"]);
        assert_eq!(symbols[0]["detail"], "2 steps");
        assert_eq!(symbols[0]["range"]["start"]["line"], 1);
        assert_eq!(symbols[0]["range"]["end"], json!({ "line": 2, "character": 49 }));
        assert_eq!(symbols[0]["selectionRange"], range(1, 0, 3));
        let steps = symbols[0]["children"].as_array().unwrap();
        assert_eq!(steps[0]["name"], "Step 1.1");
        assert_eq!(steps[0]["detail"], "Set c=sqrt(a*2+b*2).");
        assert_eq!(steps[1]["selectionRange"], range(2, 0, 3));
        assert_eq!(symbols[1]["detail"], "a = __   b = __   c = ___.______");
        assert_eq!(symbols[1]["range"]["end"]["line"], 4);
    }

    #[test]
    fn folding_covers_each_part_forms_and_runs_of_notes() {
        let (mut server, _) = server_with(PROGRAM);
        let reply = ask(&mut server, "textDocument/foldingRange", 0, 0, json!({}));
        assert_eq!(
            reply["result"],
            json!([
                { "startLine": 1, "endLine": 2 },
                { "startLine": 3, "endLine": 4 },
                { "startLine": 5, "endLine": 6 },
            ])
        );
        let (mut server, _) =
            server_with("* one\n* two\n1.1 Line.\n\n* a note inside\n1.2 Line.\n2.1 Page.\nDo part 1.\n");
        let reply = ask(&mut server, "textDocument/foldingRange", 0, 0, json!({}));
        assert_eq!(
            reply["result"],
            json!([{ "startLine": 0, "endLine": 1, "kind": "comment" }, { "startLine": 2, "endLine": 5 }])
        );
    }

    #[test]
    fn semantic_tokens_are_encoded_as_deltas() {
        let (mut server, _) = server_with("* note\n1.1 Type x if x >= 0.\nSet x = sqrt(4).\n");
        let reply = ask(&mut server, "textDocument/semanticTokens/full", 0, 0, json!({}));
        assert_eq!(
            reply["result"]["data"],
            json!([
                0, 0, 6, 0, 0, // * note
                1, 0, 3, 9, 1, // 1.1: a label, defined here
                0, 4, 4, 7, 0, // Type
                0, 5, 1, 3, 0, // x
                0, 2, 2, 7, 0, // if
                0, 3, 1, 3, 0, // x
                0, 2, 2, 6, 0, // >=
                0, 3, 1, 2, 0, // 0
                1, 0, 3, 7, 0, // Set
                0, 4, 1, 3, 1, // x, given a value
                0, 2, 1, 6, 0, // =
                0, 2, 4, 4, 2, // sqrt
                0, 5, 1, 2, 0, // 4
            ])
        );
    }

    #[test]
    fn stand_ins_can_be_replaced_by_the_real_characters() {
        let text = "Set x = 2@3.\nType \"a @ b <= c\" if x != 6.\n* 2@3 in a note\nForm 1:\nx <= __\nType x in form 1 if 0 <= x >= 0.\n";
        let (mut server, _) = server_with(text);
        let formatted = ask(&mut server, "textDocument/formatting", 0, 0, json!({}));
        assert_eq!(
            formatted["result"],
            json!([
                { "range": range(0, 9, 10), "newText": "·" },
                { "range": range(1, 23, 25), "newText": "≠" },
                { "range": range(5, 22, 24), "newText": "≤" },
                { "range": range(5, 27, 29), "newText": "≥" },
            ]),
            "text in quotes, notes and forms are left alone"
        );
        let actions = ask(
            &mut server,
            "textDocument/codeAction",
            0,
            0,
            json!({ "range": range(1, 0, 0) }),
        );
        let actions = actions["result"].as_array().unwrap();
        assert_eq!(actions.len(), 2);
        assert_eq!(actions[0]["title"], "Write ≠ for !=");
        assert_eq!(actions[0]["kind"], "quickfix");
        assert_eq!(
            actions[0]["edit"]["changes"][URI],
            json!([{ "range": range(1, 23, 25), "newText": "≠" }])
        );
        assert_eq!(actions[1]["kind"], "source.fixAll");
        assert_eq!(actions[1]["edit"]["changes"][URI].as_array().unwrap().len(), 4);

        // A file with the real characters needs nothing.
        let (mut server, _) = server_with("Set x = 2·3.\nType x if x ≠ 6.\n");
        let formatted = ask(&mut server, "textDocument/formatting", 0, 0, json!({}));
        assert_eq!(formatted["result"], json!([]));
        let actions = ask(
            &mut server,
            "textDocument/codeAction",
            0,
            0,
            json!({ "range": range(0, 0, 0) }),
        );
        assert_eq!(actions["result"], json!([]));
    }

    #[test]
    fn unknown_requests_and_documents_are_answered_politely() {
        let (mut server, _) = server_with(PROGRAM);
        let out = server
            .handle(&json!({ "jsonrpc": "2.0", "id": 9, "method": "workspace/symbol", "params": { "query": "" } }));
        assert_eq!(out[0]["error"]["code"], -32601);
        let out = server.handle(&json!({ "jsonrpc": "2.0", "id": 10, "method": "textDocument/hover",
            "params": { "textDocument": { "uri": "file:///other.joss" }, "position": { "line": 0, "character": 0 } } }));
        assert_eq!(out[0]["result"], Value::Null);
        // Past the end of the file, and past the end of a line.
        assert_eq!(hover_text(&mut server, 99, 0), "");
        let past = ask(&mut server, "textDocument/completion", 99, 4, json!({}));
        assert!(past["result"].as_array().unwrap().len() > 5);
        let out = server.handle(&json!({ "jsonrpc": "2.0", "id": 11, "method": "shutdown" }));
        assert_eq!(out[0]["result"], Value::Null);
        assert!(server.shutdown);
    }

    #[test]
    fn the_sessions_of_1964_and_1965_can_be_opened() {
        // What the users typed in the original sessions, as a file. Nothing
        // in them is an error to the editor except what was an error then.
        let transcripts = concat!(env!("CARGO_MANIFEST_DIR"), "/../transcripts");
        let mut seen = 0;
        for entry in std::fs::read_dir(transcripts).unwrap() {
            let path = entry.unwrap().path();
            let text = std::fs::read_to_string(&path).unwrap();
            let typed: String = text
                .lines()
                .filter_map(|line| line.strip_prefix("U: ").or(if line == "U:" { Some("") } else { None }))
                .map(|line| format!("{line}\n"))
                .collect();
            let (mut server, out) = server_with(&typed);
            assert_eq!(out[0]["method"], "textDocument/publishDiagnostics");
            let lines = typed.lines().count() as u32;
            for line in 0..lines {
                let width = typed.lines().nth(line as usize).unwrap().encode_utf16().count() as u32;
                for character in 0..=width {
                    for method in [
                        "textDocument/hover",
                        "textDocument/definition",
                        "textDocument/completion",
                    ] {
                        ask(&mut server, method, line, character, json!({}));
                    }
                    ask(&mut server, "textDocument/signatureHelp", line, character, json!({}));
                    ask(
                        &mut server,
                        "textDocument/documentHighlight",
                        line,
                        character,
                        json!({}),
                    );
                }
            }
            for method in [
                "textDocument/documentSymbol",
                "textDocument/foldingRange",
                "textDocument/semanticTokens/full",
                "textDocument/formatting",
            ] {
                let reply = ask(&mut server, method, 0, 0, json!({}));
                assert!(reply["error"].is_null(), "{method}");
            }
            seen += 1;
        }
        assert!(seen >= 5);
    }
}
