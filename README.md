# joss

JOSS, the JOHNNIAC Open-Shop System that J. C. Shaw wrote at RAND in
1963-64, rebuilt in Rust with no dependencies from the papers and the
program listing that survive, and held to one test: type the console
sessions printed in those papers again, and get the same page.

```
Type 2+2.
     2+2 =       4
```

JOSS was a conversation at a typewriter. You typed a line; JOSS did what it
said, or kept it as a step of a program, or told you what was wrong with it;
and then it was your turn again. When a program went wrong you did not start
over. You put the one thing right and typed `Go.`

```
1.1 Set x(1) = [-b+sqrt(b*2-4·a·c)] / [2·a].
Set b = -1.
Set c = -6.
Do part 1.
Error at step 1.1:  a is undefined.
Set a = 0.
Go.
Error at step 1.1:  I have a zero divisor.
Set a = 1.
Go.
```

This is the JOSS of the JOHNNIAC, as it stood in 1964 and 1965. It is not
the larger JOSS that followed on the PDP-6 in 1966, which is what most
descriptions of "JOSS" describe.

## Install

You need [Rust](https://rustup.rs). To install the console and the language
server from this repository:

```bash
cargo install --git https://github.com/chrishayuk/joss joss joss-lsp
```

That puts `joss` and `joss-lsp` in `~/.cargo/bin`. Then:

```bash
joss                       # sit at the console
echo 'Type 2+2.' | joss
```

For Visual Studio Code, download `joss-language-0.1.1.vsix` from the
[latest release](https://github.com/chrishayuk/joss/releases/latest) and
install it:

```bash
code --install-extension joss-language-0.1.1.vsix
```

Open any `.joss` file and you get highlighting, hover help, problems as you
type, and a run button. The extension carries its own copy of the
interpreter and language server, so it works without the `cargo install`
above. For Neovim, Helix and other editors see
[`editors/README.md`](editors/README.md).

Or try it with nothing installed, in the
[playground](https://chrishayuk.com/demos/joss).

## Build and run

```bash
cargo build --release
target/release/joss                              # a console: type a line, JOSS answers
target/release/joss examples/1965-primes.joss    # replay a file of typed lines
```

A `.joss` file is what would be typed at the console, a line at a time.
`joss file` prints the page: your lines and JOSS's answers. Other ways to run
it:

| | |
|---|---|
| `joss --marked file` | the page with `U: ` or `J: ` in front of each line, to show who typed it |
| `joss --quiet file` | only what JOSS typed |
| `joss --colour file` | your lines in green, as the console's ribbon had them |
| `joss --year 1964 file` | the JOSS of March 1964: its wording and its columns (see below) |
| `joss -i file` | type the lines in a file, then hand the typewriter over: carry on at the console |
| `joss --check file` | read a file without running it and say what is wrong with it |
| `joss --verify transcripts/*.txt` | replay the original sessions and compare with the page |
| `joss --patience N` | press the interrupt key after N steps |

In a file, a line `! interrupt after 16 lines` is not typed. It is the
INTERRUPT key, pressed once JOSS has typed sixteen lines of its next answer.

## The language

A line that begins with a number is a **step**, kept for later. Steps 1.1,
1.2 and 1.35 belong to **part** 1. Anything else is done at once. Every
instruction is a sentence: a capital letter at the start, a full stop at the
end.

| | |
|---|---|
| `Set x = 3.` | give a letter a value. There are 52 names: `a`-`z` and `A`-`Z` |
| `Set a(i,j) = 0.` | a letter may have one or two indices, each 0 to 99 |
| `Type x, x*2, sqrt(x).` | type values, each on a line with its expression |
| `Type "text".` | type text |
| `Type a, b in form 1.` | type values into the blanks of a form |
| `Form 1:` | the next line typed is form 1: `x = __.__   y = ..........` |
| `Do part 1.` `Do step 1.1.` | do the steps of a part, or one step |
| `Do part 1 for x = 1(1)10, 20, 50(50)200.` | do it for each value: 1 in steps of 1 to 10, then 20, then 50 to 200 |
| `... if a<b≤c and d≠e or f=g.` | any instruction may carry a condition |
| `1.4 To step 1.1.` `To part 3.` | go on from there instead |
| `1.4 Done.` | no more of this part, this time round |
| `1.4 Stop.` | break off; `Go.` carries on |
| `1.4 Demand x.` | ask for a value: JOSS types `x = ` and waits |
| `Go.` | carry on with a task that an error, `Stop` or the interrupt key broke off |
| `Cancel.` | forget that task |
| `Type step 1.1.` `Type part 1.` `Type form 1.` | type them back |
| `Type all.` `Type all steps.` `Type all forms.` `Type all values.` | everything, or one kind |
| `Delete x, y.` `Delete step 1.1.` `Delete part 1.` `Delete form 1.` `Delete all.` | remove |
| `Line.` `Page.` | a blank line; a new page |
| `Type size.` `Type time.` `Type users.` | storage in use; the time; how many are using JOSS |
| `x=3` | `Set` without the word, when given directly |

Arithmetic is `+`, `-`, `·` (times), `/` and `*` (to the power), with
`( )` and `[ ]` for grouping, either kind, and `|x|` for the absolute value.
The functions are `sqrt`, `log` (natural), `exp`, `sin`, `cos`, `arg(x,y)`,
`ip` (integer part), `fp` (fraction part), `dp` (digit part), `xp` (exponent
part), `sgn`, `max` and `min`. `$` is the number of the line the typewriter
is on. A line that begins or ends with `*` is ignored, which makes it a note.

Numbers are nine decimal digits and a power of ten from -99 to 99, and
arithmetic is done in decimal: "The user may ask JOSS to add .1 ten times and
the result is indeed 1."

### Typing the typewriter's characters

The JOSS typewriter had keys of its own. Four are not on a keyboard now, and
each has a stand-in that is accepted when typed. JOSS always types the real
one.

| JOSS | meaning | you can type |
|------|---------|--------------|
| `·` | times | `@` |
| `≠` | is not equal to | `!=` |
| `≤` | at most | `<=` |
| `≥` | at least | `>=` |

`*` is not times. It is "to the power".

A full tutorial is in [`tutorial/`](tutorial/01-introduction.md).

## Fidelity to the original

### The sessions

RAND's papers print console sessions: the user's lines, JOSS's answers, the
mistakes. Five of them are in [`transcripts/`](transcripts/), typed in from
the scans with the columns measured. `joss --verify transcripts/*.txt`
replays each one and compares every line JOSS types with the page.
`cargo test` does the same.

| Transcript | Source | User's lines | JOSS's lines that match | that differ |
|---|---|---|---|---|
| `1964-filmed-report` | Baker, RM-4162-PR, June 1964: a session filmed on 21 March 1964 | 78 | 318 column for column, 78 against a retyped page | 0 |
| `1964-designers-view` | Shaw, P-2922, August 1964, figures 3a-3i | 95 | 134 | 0 |
| `1965-examples` | Shaw, P-3131, April 1965: eight pages in two colours | 167 | 197 | 0 |
| `1965-experience-figure-1` | Shaw, P-3149, May 1965: prime numbers | 11 | 21 | 0 |
| `1965-experience-figure-2` | Shaw, P-3149, May 1965: a quadratic equation, nine mistakes, and `Go.` | 34 | 19 | 0 |

That is all 767 lines, 689 of them column for column. (A further six blank
lines fall off the cut edge of a figure.) What this measures, exactly: each
line is compared with the transcript as typed in from the scans, after the
editorial changes listed below; spaces at the end of a line are not counted;
a line the report's typist retyped (`J~`) is compared with all its spaces
left out; and page throws and operators' messages are not compared. It is
a regression test against those transcripts, not an independent check of
the scans.

Three numbers on the pages of 1964 are not what correctly rounded
arithmetic gives, each by one unit in its last digit:

- `log(2)` is typed `.69314718` three times. log 2 is .69314718056, which
  rounds to `.693147181`.
- `[.7*5-2.3*(-.7)]/[7*6-|9*5-5*7|*(2/3)]` is typed ending `…357`;
  correctly rounded functions give `…356`.
- In a table of a hundred exponentials, the row for 72 ends `1.8586717`.
  e to the 72nd, rounded to nine digits and then to eight, is `1.8586718`.

All three come out as printed when the arithmetic follows the listing of
January 1965: routine F1 for `log`, F2 for `exp`, and =35 for `a*b`
([`src/routines.rs`](src/routines.rs), `Num::pow` in
[`src/num.rs`](src/num.rs)). F1 cuts a logarithm below 1 off at nine
digits instead of rounding it, so log 2 is .693147180; F2 works from
x·log10(e) with an eleven-digit constant, which at x = 72 gives 1.85867174;
and =35 does a whole power below 30 by repeated squaring, each product
rounded, and the rest as exp[b·log(a)] through F1 and F2. An earlier
version of this repository used correctly rounded functions and called the
three numbers JOSS's errors; that was a guess, and the listing does not bear
it out. What is shown is that the reconstruction's reading of the routines
reproduces the pages. The original program has not been run.

Among Shaw's working papers is a handwritten note on F1 dated 11-19-64. It
reads, after a worked range for the coefficient, "Check it. Looks like
coefficient between 1 and sqrt(2) gets doubled several times." The note is
about how F1 brings the coefficient into range, which works as written; it
does not mention the cut-off ninth digit, and nothing in it explains the
printed log 2.

### Editorial changes to the transcriptions

Every change made to the text of a source is listed here and marked where it
is made, with its kind: a **slip** (wrong under any reading of the source),
a **conflict** (the source says two things and this follows the other), an
**interpretation** (our reading of what was meant), or **ours** (a
consequence of choices made for this reconstruction).

| Where | Change | Kind |
|---|---|---|
| Film, `transcripts/1964-filmed-report.txt` (1) | A comma before "in form 1" removed: RM-4162 p. 13 has it, the listing of the same step on p. 15 does not | conflict |
| Same (2) | `Delete all.` inserted: the next listing (Example 3) has nothing of the first program in it, so something cleared it; this exact line is not on any page | interpretation |
| Same (3) | A multiplication dot inserted before `sin`: p. 16 has a space, the facsimile listing on p. 29 has the dot | conflict |
| Same (4) | A space added after `cos` in JOSS's answer, as the user typed it: one of the two printed lines was retyped inaccurately | interpretation |
| Same (5), and `sources/filmed-report-1964/transcripts/example-5.txt` | A decimal point restored in a form, `_ __` to `_.__`: the scenario's copy (p. 17) and the output support it; that it dropped out is inferred | interpretation |
| Film, `page-11.txt` | The Chart A expression is taken with `-` as typed at the console (p. 11), not `+` as in the chart (p. 21); the printed answer agrees with `-` | conflict |
| Shaw's notes, `page-02.txt` | `Functiona:` read as `Functions:`, `numeriaal` as `numerical` | slip |
| Undated listing, `messages.txt` | `OVERLFOW` (a comment) and `anotther` (a message) are kept as they are and noted | slip, not corrected |

### The messages

"For its comments, it simply selects from a stock of forty 'canned'
messages." The papers show about a dozen. The rest are from the program: the
JOHNNIAC assembly listing headed "JOSS 666, JCS 1-9-65" keeps each message as
octal words, five characters to a word, and they were decoded with the
listing's own table of characters. See [`sources/AUDIT.md`](sources/AUDIT.md)
and [`src/messages.rs`](src/messages.rs).

### 1964 and 1965

The pages disagree with each other, because JOSS changed. "After a year's
experience with neutral, impersonal messages, we introduced personal pronouns
and some informal diction."

| | `--year 1964` | `--year 1965` (the default) |
|---|---|---|
| A zero divisor | `Zero divisor.` | `I have a zero divisor.` |
| A letter with no value | `Undefined.` | `x is undefined.` |
| The interrupt key | `Interrupted at step 1.1.` | `I'm at step 1.1.` |
| `Type 2+2.` | `     2+2 =      4` | `     2+2 =       4` (one column further right) |
| A small number | `.02` is typed `2·10*(-2)` | fixed point down to `.001` |
| `Type all.` | no blank line between forms | a blank line after each form |

The 1964 wording is known only where a 1964 page shows it: four messages and
the interrupt. Where no 1964 page shows a message, `--year 1964` uses the
words of 1965. An earlier, undated listing has a third and older wording
("Attempting division by zero."), which is not implemented.

### What is not original

- `log`, `exp` and `a*b` follow routines F1, F2 and =35 of the listing, as
  read here: the order of operations, constants and rounding are the
  listing's, but how JOHNNIAC rounded a quotient and a few other details of
  the machine are assumed (they are listed in `src/routines.rs`). `sin`,
  `cos` and `arg` are computed with the Rust standard library and rounded to
  nine digits; their routines (F3, F4, F10) are in the listing and have not
  been followed, except that `arg(0,0)` is 0, as F10 makes it. `sqrt` and
  the four operations are correctly rounded, as Shaw says they were (P-2922,
  p. 25).
- How much storage a step, a value and a level of a task take is an
  estimate, so `Type size.` and the point at which JOSS says "I need more
  storage space." are approximate. The block is 720 words, the figure for
  available space on Shaw's storage map.
- `Type size.`, `Type time.` and `Type users.` type a bare number, as the one
  `Type users.` in the papers does. The time is fixed unless a file sets it.
- Not here: the sign-on ("Initials please:"), the heading JOSS typed at the
  top of each page, the queue for a console, and the other seven users.
  `Read` and `Punch` are not here either, and were not working in January
  1965: they are named in the listing's verb table and written into Shaw's
  summary of the language by hand, but their routines (G9 and G10, pp.
  76-77) go straight to "Eh?". P-3149 (May 1965, p. 12 as printed) says a punched card
  file was rejected.
- The interrupt key is modelled by counting lines of output, or steps. The
  page records that it was pressed, not exactly when.
- A value too long to fit on the line after a long label should go on the
  next line. It does not.
- Shaw's typed summary of the language has `Type a,_,b, c in form 1.`, with
  the `_` written in by hand. What a blank line in the middle of a form
  would mean is not clear, and here it is "Eh?".
- A power with a sign goes in brackets, `10*(-3)`, as JOSS itself types it.
  Whether `10*-3` was accepted is not known; here it is "Eh?".
- In 1964 a small number is typed with an exponent below a tenth. The pages
  show `.2` typed plain and `.02` with an exponent; where between the two
  the line fell is not shown, and a tenth is an assumption.

## Examples

Every sitting in [`examples/`](examples/README.md) has a `.out` file with the
page it gives, and `cargo test` replays them all. There are 55. Eighteen are
the programs of the original papers, under their year, with the user's lines
as they stand in the transcripts: `1965-quadratic-with-mistakes`,
`1965-primes`, `1965-root-finding`, `1965-matrix-inversion`, `1965-table`,
`1964-film-table` and the rest. Thirty-seven were written for this
repository: Euclid's algorithm with `To`, a sieve, Simpson's rule, a bubble
sort on an indexed letter, a loan table in a form, `Stop` as a breakpoint, an
error put right with `Go.`

## Tutorial

[`tutorial/`](tutorial/01-introduction.md) is a 17-chapter tutorial, from
`Type 2+2.` to forms, `Demand`, mistakes and `Go.`, the programs of the
papers, and a reference. It has 156 sittings in it, each followed by the page
JOSS types. `cargo test` runs every one and compares (`tests/tutorial.rs`).

## In the browser

`web/` is a playground that runs the console in a web page. It opens on a
typewriter, because that is what JOSS was: paper on a platen, your lines in
green and JOSS's in black, a red light while JOSS has the typewriter, and a
soft gong when it hands it back. JOSS types at ten characters a second, as
the papers say it did, or faster if you ask. It signs on as the listing of
January 1965 does ("JOSS at your service." "Initials please:") or as the
1964 film does, and heads each page with the time, the date and your
initials. The sign-on and the headings are played by the page
(`web/typewriter.js`, which says where each comes from); everything after
them is the interpreter. `?sit=1965-primes` types a sitting in; `&speed=fast`.
Below it is a second view, for typing in a file. The interpreter
is compiled unchanged for WASI (`wasm32-wasip1`), and `web/wasi.js` is a small
host that supplies standard input and output, so no other library is needed.

JOSS waits for a line, and a WebAssembly program cannot be paused in a page
while it does. So it is not: each time a line is typed, the whole sitting is
replayed from its first line (`web/session.js`). JOSS has no clock in it and
no chance, so each replay types the same page and then one line more. The
INTERRUPT button works the same way: it records where it was pressed and
replays.

The box on the left is an editor with colours, problems as you type, hover
help and completion, all from the language server, which runs in the page as
well (`web/editor.js`, `web/lsp-worker.js`). The page can also replay each of
the original sessions and show how it compares.

```bash
node web/build.mjs                  # builds web/joss.wasm, web/joss-lsp.wasm, web/examples.json
node web/test.mjs                   # replays every example and transcript through the wasm build
python3 -m http.server -d web 8000  # then open http://localhost:8000
```

Add `?example=1965-primes` to the address to open a particular sitting, and
`&run` to type it in straight away.

## In your editor

`joss --check file.joss` reads a file without running it and says what it can
see: lines that are not JOSS (and where they stop being JOSS, which the
original never said), a `Do` of a part with no steps, a form with too few
blanks for the values, a letter that is used and never given a value, a step
that a later line replaces.

The same analysis drives a language server, `joss-lsp` in `lsp/`, with hover
help for every word and character, go to definition for steps, parts and
forms, references, rename, completion and more. `editors/vscode` is a Visual
Studio Code extension that bundles the server and the interpreter as
WebAssembly and adds syntax highlighting and a run button. See
[`editors/README.md`](editors/README.md).

The interpreter itself has no dependencies. The language server is a separate
crate in the same workspace and uses `serde_json`.

## Sources

The papers and the listings are at
[Bitsavers](https://bitsavers.org/pdf/rand/joss/). [`sources/`](sources/README.md)
has what was read out of them: the transcripts of every console session, the
notes on how each page was measured, and the audit of the listings.
[`docs/scans/`](docs/scans/README.md) has the pages that matter most, reduced.

All of the reading was done by a coding agent and its sub-agents. No person
has checked it against the pages by eye. An independent second model
(OpenAI Codex) reviewed the repository and checked twenty places in the
transcriptions against the scans, character by character; what it found and
what was done about it is in
[`docs/independent-review.md`](docs/independent-review.md).

## Tests

```bash
cargo test
```

The unit tests cover the arithmetic, the forms, the layout of a typed value
in each year, the parser and the static analysis. The integration tests
replay the five original sessions and compare every line with the page;
replay a session written to hold the reconstruction to what the listing says
about the things no page shows (`tests/sessions/`); replay every example; and
run every sitting in the tutorial.

```bash
cargo test -p joss-lsp                   # the language server
cd editors/vscode && npm install && npm run build && npm test
```

## Licence

Apache License 2.0 - see [LICENSE](LICENSE). That covers the code, the
examples and the tutorial. The page images in `docs/scans` are reproduced
for reference and remain the copyright of The RAND Corporation; the
transcripts in `transcripts/` and `sources/` are readings of those pages.
