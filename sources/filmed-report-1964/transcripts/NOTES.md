# NOTES — C. L. Baker, "JOSS: Scenario of a Filmed Report", RM-4162-PR, June 1964

Page references are **PDF page numbers** (= `pages/p-NN.png`); the printed page number is given
in brackets where useful. Columns are **0-based** from the JOSS left margin unless stated.

## 0. Read this first: two very different kinds of source in this document

| Kind | Where | Files | Trust for spacing |
|---|---|---|---|
| **Facsimile** of real JOSS typewriter output (JOSS typeface, boxed figures) | Appendix A Examples 1–5, PDF pp. 22–24, 27–29, 31–32 | `example-1.txt` … `example-5.txt` | High. Measured on a fitted 10-pitch grid; digits machine-checked. |
| **Retyped** scenario (report typist's typewriter, with `U:` / `J:` labels in the margin) | PDF pp. 10–18 [2–10] | `page-10.txt` … `page-18.txt` | Low for default `Type` output. Faithful to the report, but the report demonstrably **squeezes out JOSS's blank columns** (see (a)3). Form-driven output on these pages does agree with the facsimiles. |

* The PDF is a 400-dpi **bilevel** scan, so there is no ribbon-colour information. Attribution
  comes from the report's own `U:`/`J:` labels (pp. 10–18); the Appendix A figures are all JOSS
  output (they are the outputs the scenario refers to with "See Example N").
* The report itself says the scenario is "somewhat modified" (p. 3), and it is: see (f).
* Pages with no typewriter session: 1–9 (front matter, preface, summary, contents, console photo,
  introduction), 19 (closing remarks), 20 (blank), 21 Chart A, 25 Chart B, 26 Chart C, 30 Chart D,
  33–50 Appendix B (Q&A prose), 51 Appendix C (word list).

Method: character pitch fitted per page (20.33–20.35 px/char at 200 dpi, i.e. 10 chars/inch,
with a per-page skew term; mean residual under 1 px on the facsimiles); every glyph assigned to a
grid column; each transcript line then checked mechanically against the page's column-occupancy
pattern. For the three big tables every digit cell (400 dpi) was template-matched against values
computed independently; the single disagreement was confirmed as printed and is transcribed
as printed (see (f)1).

---

## (a) Default `Type` layout — measured rules and evidence

### 1. What the facsimiles establish (hard evidence)

Only source: the "values" block at the end of the two facsimile `Type all` outputs.

| File / line | Label col | `=` col | Number starts | Units digit | Decimal point |
|---|---|---|---|---|---|
| example-3 `d =      2·10*(-4)` | 7 | 9 | 16 | 16 | — |
| example-3 `f =    501.716868` | 7 | 9 | 14 | 16 | 17 |
| example-3 `k =      6.2831853` | 7 | 9 | 16 | 16 | 17 |
| example-3 `t =      2·10*(-2)` | 7 | 9 | 16 | 16 | — |
| example-3 `C =      5·10*(-7)` | 7 | 9 | 16 | 16 | — |
| example-3 `F =    503.292122` | 7 | 9 | 14 | 16 | 17 |
| example-3 `I =      3.17220634` | 7 | 9 | 16 | 16 | 17 |
| example-3 `L =       .2` | 7 | 9 | 17 | (none) | 17 |
| example-3 `Q =      1·10*(-3)` | 7 | 9 | 16 | 16 | — |
| example-3 `R =    100` | 7 | 9 | 14 | 16 | — |
| example-5 `c =     57.2957796` | 7 | 9 | 15 | 16 | 17 |
| example-5 `k =       .434294482` | 7 | 9 | 17 | (none) | 17 |
| example-5 `m =      1.62686274` | 7 | 9 | 16 | 16 | 17 |
| example-5 `p =      9` | 7 | 9 | 16 | 16 | — |
| example-5 `L =      2` | 7 | 9 | 16 | 16 | — |

Column 0 is where the step numbers (`1.1 Set …`) of the same listing start, on the same fitted grid.

Rules that follow:

* **`=` is in column 9** (10th character); a single space separates label and `=`; the
  single-letter label is in column 7. So 7 leading spaces before a one-letter label.
* **Units digit in column 16, decimal point in column 17** (1-based: 17 and 18). Six columns
  (10–15) lie between `=` and the units digit; they hold the higher integer digits, else blanks.
* **No leading zero**: `.2`, `.434294482` — the point is in column 17 and column 16 is blank.
* **Integers print with no decimal point** (`100`, `9`, `2`); **trailing zeros are suppressed**
  (`6.2831853` = 8 digits, `501.716868`).
* **Up to 9 significant digits** (`3.17220634`, `57.2957796`, `.434294482`).
* **Scientific notation**: mantissa's units digit in the units column (16), then immediately
  `·10*(-n)`: `2·10*(-4)`, `5·10*(-7)`, `1·10*(-3)`. An integral mantissa has no point. Negative
  exponents are parenthesised. With a fractional mantissa (retyped, p. 16): `1.99315603·10*(-4)`.
* **When scientific is used**: `.0002`, `.001`, `.0000005`, `.02` print as `2·10*(-4)`,
  `1·10*(-3)`, `5·10*(-7)`, `2·10*(-2)`; while `.2`, `.4658`, `.434294482`, `.69314718`,
  `.707106781` print fixed. So the fixed/scientific switch for small numbers lies somewhere in
  (.02, .2]. (Note `t = 2·10*(-2)` — .02 is already scientific.) Largest fixed value seen:
  `22026.4658`. No large-number or positive-exponent scientific output appears anywhere in this
  document, so the form of e.g. `…·10*43` is **not attested here**.
* **Order of `Type all` values**: lower-case letters alphabetically, then upper-case
  (example-3: d f k t C F I L Q R; example-5: c k m p L).

### 2. What is NOT established by facsimile

* Whether a plain `Type x.` uses the same columns as the `Type all` value block. (Very likely —
  see 3 — but there is no facsimile of a plain `Type` in this document.)
* Where a **negative sign** goes in default output (only retyped: `-1.2·1.2 = -1.44`,
  `… = -3.33630357·10*(-6)`).
* How **long labels** behave (the 38-character expression on p. 11 obviously cannot end in col 7).
* Layout for indexed labels such as `q(1,1) =` (only retyped Demand prompts, p. 18).

### 3. The retyped pages, and why their spacing is squeezed

Measured columns (0 = start of the report's text column), all from `page-NN.txt`:

| Block | Lines | Label | `=` col | Number start | Decimal-point col |
|---|---|---|---|---|---|
| p. 10–11 single results | `2+2 = 4`, `7/2 = 3.5`, `2*8 = 256`, `sqrt(49) = 7` | at col 0 | label+1 | `=`+2 | — |
| p. 12 | `    y = 22026.4658` / `ip(y) = 22026` / `fp(y) =      .4658` / `dp(y) =     2.20264658` / `xp(y) =     4` | right-aligned, ends col 4 | 6 | 8 / 8 / 13 / 12 / 12 | 13 (units col 12) |
| p. 12–13 | `      x =  1` / `sqrt(x) =  1.41421356` / ` log(x) =   .69314718` / ` exp(x) = 20.0855369` | right-aligned, ends col 6 | 8 | 11 / 11 / 12 / 10 | 12 (units col 11) |
| p. 15 | `x =   100` / `y = 22026.4658` | col 0 | 2 | 6 / 4 | 9 (units col 8) |
| p. 16 | `F = 503.292122` / `I =   3.17220634` / `d =   1.99315603·10*(-4)` | col 0 | 2 | 4 / 6 / 6 | 7 (units col 6) |

In every retyped block: labels are **right-aligned** to a common `=`, and **units digits /
decimal points are aligned**, but the gap between `=` and the widest number is always exactly one
space and the widest label always starts at the margin. On p. 12 JOSS typed `x = 1` before it
could know that `exp(3) = 20.0855369` would need another digit, yet room is there — so JOSS must
be using fixed columns, and the report typist removed the columns that were blank in the whole
block. The same values `F`, `f`, `I` that are retyped as `F = 503.292122` (p. 16) appear in
facsimile (example-3) as `       F =    503.292122`.

**Working hypothesis (consistent with every line in the document):** JOSS right-justifies the
label so that `=` lands in column 9, and puts the units digit in column 16; the retyped pages are
that layout with common blank columns deleted. Treat the `=`-spacing in `page-10` … `page-16` as
unreliable; the *relative* alignment (right-aligned labels, aligned decimal points) is reliable.

Other things the retyped default output shows: several expressions in one `Type` give one line
each; `_` in a Type list gives a blank line (p. 11: blank between `3·2 = 6` and `3/2 = 1.5`; p. 12:
blank after each `exp(x)` line); the expression is echoed as the label (`2+2-3-3+4+4 = 6`);
`7.3890561` (trailing zero of 7.38905610 dropped); `log(2.71828183) = 1`; `log(1) = 0`.

### 4. Formatted output (`Type … in form N.`) — measured

Form pictures (columns exact; the same pictures appear retyped on pp. 13/15/16/17 and, for forms
3 and "P L M"/form 2, in facsimile in examples 3 and 5):

```
Form 1 (pp. 13, 15; retyped; consistent with facsimile output in example-1):
___  __._______  __._______   .............
cols 0-2 | 5-14 | 17-26 | 30-42 (13 dots)

Form 3 (example-3, facsimile):
__.__ ms.   -_.______ amp.
cols 0-4 | text 6-8 | 12-20 (hyphen + _.______) | text 22-25

Form 2 of the second problem (example-5 facsimile / p. 17 retyped):
__    _.__   -_.__
cols 0-1 | 6-9 | 13-17 (hyphen + _.__)
```

Observed behaviour (example-1, -2, -4):

* A field is a run of `_` with an optional `.`; the value is **right-justified** in the field,
  decimal point exactly on the picture's `.`; text outside fields is copied (`ms.`, `amp.`).
* Integer field `___`: `  1`, ` 41`, `100` (cols 0–2). `__`: `81`, ` 9`.
* `__._______`: `1.0000000` occupies cols 6–14, `10.0000000` cols 5–14.
* **Leading zero suppressed, replaced by a blank**: `.0000000` (log 1), `.6931472`, `.00 ms.`,
  `.10`, `.000000 amp.`
* **Negative sign** is typed immediately left of the first digit or of the point: `-1.405807`,
  ` -.026650`, `-2.88`, ` -.61`. In these forms the field's first character is a typed hyphen
  (`-_.______`, `-_.__`); positive values leave that column blank. Whether the hyphen is a
  required "sign position" character or just counts like `_` is not stated in the document.
* A value that rounds to zero prints unsigned: example-4 row `45     .80     .00`.
* **Rounding is half-up applied to the already-9-digit value** (double rounding):
  log 7 = 1.945910149 → 9 digits 1.94591015 → `1.9459102`; exp 36 = 4.311231547·10^15 →
  4.31123155 → `4.3112316 15`; sqrt 20 = 4.472135955 → 4.47213596 → `4.4721360`; example-2
  −1.40580649… → −1.40580650 → `-1.405807`. All rows of examples 1, 2 and 4 agree with "round to
  9 significant digits, then round half-up to the field" except one last-digit case (see (f)1).
* **Scientific field** (13 dots, cols 30–42): output is blank, digit, `.`, 7 digits, blank,
  2-digit exponent: ` 2.7182818 00`, ` 2.6881171 43` — i.e. digit in col 31, point col 32,
  decimals 33–39, col 40 blank, exponent in cols 41–42, zero-padded (`00`, `01`). Col 30 and
  col 40 are presumably the mantissa-sign and exponent-sign positions; no negative case is shown.
  Narration (p. 13): "a string of dots to indicate that scientific notation is desired."
* `Type form 2.` types the form's picture line itself as a heading (` X     SQRT(X)     LOG(X)        EXP(X)`).

---

## (b) JOSS messages seen (verbatim)

| Message | Where | Trigger |
|---|---|---|
| `Press 'IN' for JOSS service.` | p. 10, page-10 | User pressed the POWER button on the console. |
| `Please identify yourself.` | p. 10 | User pressed the IN button. |
| `3-21-64 RAND` | p. 10 | Typed by JOSS after the user typed `RAND`; footnote: "This line is typed by JOSS at the top of a new page (8½"×11"). One-inch margins and similar date-lines are automatically supplied for each page." |
| `Error above:  Zero divisor.` | p. 11 | Direct `Type 7/0.` |
| `Error above:  Undefined.` | p. 12 | Direct `Type y.` with y never set. |
| `Interrupted at step 1.1.` | p. 13 (twice) | INTERRUPT button during `Do step 1.1 for x = 1(1)100.` and again after `Go.` |
| `Interrupted at step 1.05.` | p. 14 | INTERRUPT button during `Do part 1 for x = 1(1)100.` |
| `Insufficient storage space` (quoted in prose only, never shown typed) | p. 45 [37] | "When you override the limit, you come to an error message which will print out the words 'Insufficient storage space,'" |

Both `Error above:` lines are printed with **two** spaces after the colon, but they are retyped
and the typist double-spaces after colons in her own prose too — unverified.
The Demand prompt `q(1,1) = ` (p. 18) is the only other thing JOSS types.

**Not present anywhere in this document:** `Eh?`, `Error at step …`, `Stopped by step …`,
`Revoked. …`, `Done.`, or any other message. Appendix B says (p. 35 [27]): "We showed only a
couple of errors in the film. There are actually thirty error messages to which you must respond."

---

## (c) Statement forms, verbs, clauses, functions, syntax — with one example each

### Seen in use

| Feature | Example | Page |
|---|---|---|
| Direct `Type` expression | `Type 2+2.` | 10 |
| `Type` with several expressions | `Type 3+2,3-2,3·2,_,3/2,3*2.` | 11 |
| `_` in a Type list = blank line | `1.1 Type x, sqrt(x), log(x), exp(x), _.` | 12 |
| `Type … in form N` | `2.1 Type 1000·t, I·exp[-R·t/(2·L)]·sin(k·f·t) in form 3.` | 29 (ex. 3) |
| `Type form N` (types the picture line) | `2.2 Type form 2.` | 14 |
| `Type all` (steps, then forms, then values) | `Type all.` | 15, 16, 18; ex. 3, 5 |
| `Set` (direct and stored) | `Set y = exp(10).` / `1.3 Set I = k·F*2·Q/f.` | 12 / 15 |
| Stored step = statement prefixed by number | `1.1 Type x, sqrt(x), log(x), exp(x), _.` | 12 |
| Step replaces existing step of same number | `1.1 Type x, … in form 1.` | 13 |
| Step number with 2 decimals, inserted in order | `1.05 Do part 2 if fp(x/40) = 1/40.` | 14 |
| `Do step` with for-clause | `Do step 1.1 for x = 1(1)100.` | 12 |
| `Do part` (direct) | `Do part 1.` | 16 |
| `Do part` as stored step (subroutine call) | `1.3 Do part 2 for p = 81, 72, 45, 9.` | 17 |
| for-clause range a(b)c | `for x = 1(1)100` | 12 |
| for-clause value list | `for p = 81, 72, 45, 9` | 17 |
| for-clause mixed list + range | `for L = .1, .2(.2)2` | 17, 32 |
| range with expressions / variables | `for t = 0(d)100·d` ; `for x = 1(1)y` | 16 ; 18 |
| if-clause on a step | `1.07 Line if fp(x/5) = 1/5.` | 14 |
| `Line` | `2.1 Line.` | 17 |
| `Page` | `2.1 Page.` | 14 |
| `Form N:` + picture on next line | `Form 3:` / `__.__ ms.   -_.______ amp.` | 16, 29 |
| Form fields: `_` runs with `.`; dot runs for scientific; literal text; leading `-` | see (a)4 | 13, 16, 17 |
| `Go` | `Go.` | 13 |
| `Delete all` | `Delete all.` | 17, 18 |
| `Demand` (stored only), indexed variable | `1.1 Demand q(x,y).` | 18 |
| Indexed variable with two indices | `q(x,y)`; prompts `q(1,1) =` … `q(3,4) =` | 18 |
| Line kill: line ending in `*` is ignored | `Form1:*` | 13 |
| Strike-over correction (backspace + overtype) | `( ` overstruck with `,` in step 2.2 | 17 |
| Multiplication `·` | `Type -1.2·1.2.` | 10 |
| Division `/` | `Type 7/2.` | 10 |
| Exponent `*` | `Type 2*8.` → 256 ; `R*2` | 11 |
| Unary minus, negative exponent in parentheses | `2.3*(-.7)` ; `.5·10*(-6)` | 11 ; 16 |
| Fractional exponent | `…*(2/3)` | 11 |
| Absolute value bars | `|9*5-5*7|` | 11 |
| Parentheses and brackets interchangeable | `[.7*5-2.3*(-.7)]/[…]` ; `sqrt[1/(L·C)]` | 11 ; 15 |
| Functions used: `sqrt`, `log` (base e), `exp`, `cos`, `sin`, `ip`, `fp`, `dp`, `xp` | `Type y, ip(y), fp(y), dp(y), xp(y).` | 11, 12, 16 |
| Numbers: leading-point decimals, leading zero accepted on input | `.001` ; `0.000987` | 16 ; 18 |
| Upper- and lower-case letters are distinct variables | `F` and `f` ; `L` vs `m`,`p` | 15, 29 |
| Relation `=` in conditions | `fp(x/40) = 1/40` | 14 |

### Listed in Appendix C "JOSS WORDS AND SYMBOLS" (p. 51 [43]) — full table, verbatim

* **COMMANDS — Direct only:** Cancel, Delete, Go, Form. **Indirect only:** To, Done, Stop, Demand.
  **Both:** Type, Do, Set, Line, Page.
* **MODIFIERS:** `if, and, or` · `for` · `in form`
* **DESIGNATORS:** `step, all steps` · `part, all parts` · `form, all forms` · `all values` · `all` · `size`
* **FUNCTIONS:** sqrt, log, exp, sin, cos, arg, max, min, sgn, ip, fp, dp, xp
* **OPERATIONS:** `+` `-` `·` `/` `*` `| |` `( )` `[ ]`
* **RELATIONS:** `=` `≠` `>` `≥` `<` `≤`
* **PUNCTUATION AND SPECIAL CHARACTERS:** `.` `,` `;` `:` `'` `"` `_` `#` `$` `?`

### Listed or described but with NO example in this document

`Cancel`, `To`, `Done`, `Stop`; `and` / `or`; `Delete` of anything but `all`;
`all steps/parts/forms/values`, `size`; functions `arg`, `max`, `min`, `sgn`; relations other
than `=`; `;`, `'`, `"`, `#`, `$`, `?`. There is no `Type "text"`, no `$`, no `#`, no
`Type size/time/users`, no parenthesised (cancellable) `Do`, and no `*` comment line (only the
line-kill use of a *trailing* `*`). `time` and `users` do not appear at all. Described in prose
only: `Type form x` with a variable form number, typing steps/parts/values individually,
multi-segment ranges, chained relations and `and`/`or` (see (d)); planned but not implemented:
`Punch`, `Reload`, a `sum` function.

---

## (d) Prose statements that specify behaviour (quoted, with page)

Quotes from pp. 10–18 were read from the page images; quotes from Appendix B (pp. 33–50) are
from the OCR text, spot-checked.

**Sign-on, paging, typewriter**
* p. 10: "[User presses POWER button, and JOSS responds by typing:] Press 'IN' for JOSS service.
  [User presses IN button.] Please identify yourself."
* p. 10 footnote: "This line is typed by JOSS at the top of a new page (8½"×11"). One-inch
  margins and similar date-lines are automatically supplied for each page."
* p. 34 [26]: "The system identified itself and asked us to identify ourselves, so that each page
  could be headed with our little commercial on it. … The simplest thing you can do takes 15 key
  strokes: turn on the system, do all initiation steps, and get an answer back — that's to the
  problem 'Type 2+2.'"
* p. 6: "It uses continuous-feed, sprocket-guide, single-ply paper in 8½"×11" sheets. The user's
  input of instructions and data is typed in green, and JOSS responds with output in black."
* p. 5: "As many as eight users at a time"; console is an "IBM Model 868 electric typewriter with
  a slightly modified character set".
* p. 14: "Part 2 — that is, all steps numbered two-point-something — asks for a new page and a
  heading line."

**Arithmetic and numbers**
* p. 10: "Multiplication must be indicated by a centered dot." "We divide by using a slash, …
  and an attempt to divide by zero will result in an error message."
* p. 11: "An exponent is indicated by an asterisk." "Several expressions may be evaluated on one
  line, and a blank line is indicated by an underscore." "A number of functions are available.
  The argument is enclosed in parentheses." "Trigonometric functions are provided, and we notice
  that numbers may be up to nine decimal digits." "Logarithms are to the base e."
* p. 12: "JOSS functions are available to take the integer part of a number, the fraction part,
  the digit part, and the exponent part." (Shown: y = 22026.4658 → ip 22026, fp .4658,
  dp 2.20264658, xp 4.)
* p. 43 [35]: "it's important for all calculations to be done in decimal, so that all 'magic
  numbers' come out accurately. If you ask for a function you know, such as 2^16 or sqrt(2),
  you'll get out the right number, exact to 9 decimal digits. If there is a fractional power,
  JOSS looks at the fraction, and if it turns out to be 1/2, you get the square root routine
  rather than the log and exponential routines. … As an example, the log of e is indeed 1, not
  .999999999 or 1.00000001". (2^16 is printed as a superscript in the report.)
* p. 41 [33]: "The internal representation of JOSS statements is identical to the external
  representation, as is the representation of decimal values."
* p. 45 [37]: JOHNNIAC has "no floating point (which we wouldn't want anyway)".
* No statement of the number *range* (largest/smallest magnitude) appears in this document.

**Stored program, steps, parts**
* p. 12: "To use JOSS as a stored-program computer, we may ask JOSS to store a statement rather
  than execute immediately. This is indicated by prefixing the statement with an identifying
  number." "We will now ask JOSS to execute this step a number of times, varying x from 1, in
  steps of 1, to 100."
* p. 13: "We will change our program step by replacing it with one that calls for typing in a form."
* p. 14: "JOSS will insert step 1.05 before our 'Type' statement, and will request part 2 as a
  subroutine every forty lines." "This time we have asked JOSS to start all over from the beginning."
* p. 43–44 [35–36]: "there are two types of commands. Those that are to be done immediately —
  such as 'Do part so-and-so,' or 'Type y,' etc. — are of course scanned and executed immediately
  and must be well formed with all variables defined at this point. Statements that are to be
  stored internally are not scanned until actual execution time … The only check that is made on
  these is that there is a well-formed statement number preceding the statement, so that there is
  some place in the hierarchy of storage to put in and recall it properly."
* No explicit rule for the allowed range/format of part or step numbers, or of index values, is
  stated in this document (step numbers seen: 1.05, 1.07, 1.1 … 3.2; indices seen: 1–4, loop to 8).

**Interrupt, errors, Go**
* p. 12: "Let's interrupt JOSS by pressing the INTERRUPT button. … JOSS has calculated several
  lines and will stop when they have been printed."
* p. 13: "'Go' tells JOSS to resume calculation at the point of interruption." (Shown: interrupted
  after typing `x = 4` in step 1.1; after the step was replaced, `Go.` re-typed the x = 4 line in
  the new form — i.e. the interrupted step is re-executed from its beginning with the loop
  variable unchanged.)
* p. 44 [36]: "when JOSS is interpreting steps in the internal mode and detects an error
  condition, it responds with an appropriate error message. The system then backs off so that you
  can correct the offending step or steps, change any of the variables, define new variables, put
  in a part if a part is missing, and so on. Then by saying 'Go.', you ask JOSS to try again from
  the beginning of the execution of the statement which caused the error. As you noticed, the
  very first thing we did with a stored program was to start JOSS iterating over the step,
  interrupt, knock the whole step out from underneath the iteration control, put in a new step,
  and then start up again."
* p. 35 [27]: "anything you try to do that is illegal or unrecognizable to the system will result
  in an error message, but always in such a way that you can clean up and go on. There are no
  catastrophic errors at all."
* p. 47 [39]: "after we push the INTERRUPT button and JOSS is typing out, we may get four or five
  lines. This varies considerably." (output already buffered is still typed.)
* Nothing is said about `Cancel`, `Stop`, `Done`, `To`, parenthesised `Do`, or what happens when a
  part is deleted while it is running.

**Typing corrections**
* p. 13: "JOSS does not require us to be perfect typists. A line may be killed by ending it with
  an asterisk, and JOSS will ignore this line."
* p. 17 and footnote: "here we can backspace, strike over the error, and go on"; "the user
  inadvertently struck the left-paren key; the error is rectified by simply backspacing and
  striking-over with the intended character, the comma."
* p. 46 [38]: "there are sixteen line buffers which will take 81 characters each. A line buffer
  assembles a line as it is typed by the user, and takes care of the backspacing, strikeovers,
  and corrections. When the carriage return is pushed, this sets a bit in the control word".
* p. 47–48 [39–40]: "the up/down shift doesn't lock when JOSS takes control, so sometimes it is
  possible to shift before control has been returned, and the shift character is lost."

**Forms**
* p. 13: "we may now use underscores in the form to indicate the position of fixed point numbers,
  and a string of dots to indicate that scientific notation is desired."
* p. 16: "The form may include any additional information we wish to print out with our answers."
* p. 47 [39]: "Field definitions in forms should be made more flexible, too." (a wish, not a feature)

**Type all / Delete all**
* p. 14: "we may ask JOSS for a copy of the program, the forms, and the value of all variables."
* p. 17: "erase it, its forms, and all associated values from JOSS' storage."
* p. 42 [34]: "we can print out steps individually, parts individually, and values individually."

**for-clauses and conditions**
* p. 16: "from t = 0, in steps of d, to 100 times d." p. 17: "for L equals .1, .2 in steps of .2 up to 2."
* p. 42 [34]: "The limits of executing the 'for' statement can be of the form 'expression, in
  steps of expression, to an expression, in steps of another expression, to another expression,'
  and so on as far as you want to go. This 'limit-step-limit-etc.' form may be one item on a
  list, and a list may have many items. Each of the components can be just about as complicated
  an expression as you want to make. All of this is, of course, finalized at the time of execution."
* p. 42 [34]: "the conditional clause doesn't just test the sign of a variable, but tests
  relationships between expressions. For instance, we can say: if a = b = c and y > z or x = y and
  relation R between expression U and expression V. This type of conditional statement may appear
  with any step or command".
* p. 41–42 [33–34]: "anyplace where there is a number in the language, you may substitute a
  variable or an expression. … We could also say 'Type form x,' and if the value of x happened to
  be 3, we would print form 3. We could say, 'Type form i' in a step, 'Do' this step for i = 1 to
  10, and print out forms 1 to 10. … Any subscript may also be an expression, and the expression
  may involve subscripted variables whose subscripts are expressions — almost ad infinitum."
* Loop termination is not described in prose. Evidence from output: `t = 0(d)100·d` with d = .0002
  produced exactly 101 lines (.00 … 20.00 ms) and `Type all` afterwards shows `t = 2·10*(-2)`;
  after `x = 1(1)100`, `x = 100`; after the nested loops, `p = 9`, `L = 2`. So the end value is
  included and the variable is left at the **last value used**, not one step beyond.
  `.1, .2(.2)2` gave 11 values (.1, .2, .4, … 2.0).

**Demand / indexed variables**
* p. 18: "JOSS can operate with indexed variables, and a stored program may call for values of
  variables to be entered during execution." "We enter the value of each variable on the same
  line as it is requested by JOSS."

**Storage and timing**
* p. 44 [36]: "Each JOSS cell holds nine characters. … there is currently a stock of 110 cells for
  each user, in addition to the cells which are needed for variables, control, and so on. This
  represents about a page's worth of typing." p. 49 [41]: "At present, it's 1000 characters".
* p. 45 [37]: "there's enough safe space left to allow you to clean up. … if you are printing in a
  form, you may decide to eliminate the form and go back to JOSS-style output, which is much
  simpler and takes much less storage space."
* p. 45 [37]: no saving of programs: "A user can, of course, as we did here, 'Type all' and retype in."
* p. 40 [32]: "At the end of each interpretation cycle, which is one statement within the machine,
  the supervisor will go and look at the I/O buffering system"; time slot "about two seconds".
* p. 37 [29]: "sentences start with capitals and end with periods, and it's very easy to tell
  English commands from algebraic expressions."

---

## (e) Sessions / figures in order

| File | PDF p. | Kind | Content |
|---|---|---|---|
| page-10 | 10 | retyped | Sign-on messages, date line, `Type 2+2`, +, −, `·`, `/`. |
| page-11 | 11 | retyped | `Zero divisor` error, `*`, multi-expression Type with `_`, big bracket/abs/fractional-power expression (scientific result), sqrt nesting, cos, log. |
| page-12 | 12 | retyped | `Undefined` error, `Set`, ip/fp/dp/xp, first stored step, `Do step 1.1 for x = 1(1)100`, start of unformatted loop output. |
| page-13 | 13 | retyped | Rest of loop output, `Interrupted at step 1.1.`, step replaced, killed line `Form1:*`, Form 1 defined, `Go.`, formatted rows 4–9, second interrupt. |
| page-14 | 14 | retyped | Step 1.05 with if-clause, part 2 (Page + Type form 2), Form 2, `Do part 1`, heading + rows 1–8, `Interrupted at step 1.05.`, step 1.07 `Line if`. |
| example-1 | 22–24 | **facsimile** | Full 100-row sqrt/log/exp table over three JOSS pages, blank line every 5 rows. Best fixture for forms (fixed + scientific fields), Page, Line, if, nested Do. |
| page-15 | 15 | retyped | `Type all` listing of the table program (layout not faithful), then the RLC-circuit program steps 1.1–1.5. |
| page-16 | 16 | retyped | Parameters, `Do part 1` (default Type of F, f, I, d incl. scientific), step 2.1 + Form 3, `Do part 2 for t = 0(d)100·d`. |
| example-2 | 27–28 | **facsimile** | 101 lines `t ms. / i amp.` in Form 3. Best fixture for sign handling and leading-zero suppression in forms, exp/sin accuracy. |
| example-3 | 29 | **facsimile** | `Type all` of the RLC program: steps, Form 3, values. Best fixture for default number layout incl. scientific. |
| page-17 | 17 | retyped | `Delete all`, brightness program (parts 1–3, Forms 1–2), strike-over correction. |
| page-18 | 18 | retyped | Constants, `Do part 1`, `Type all`, `Delete all`, Demand with doubly indexed q(x,y) and nested loops, 8 entered values. |
| example-4 | 31 | **facsimile** | 44-row P/L/M table (4 blocks of 11, blank line before each). Fixture for nested Do with list + range, log/cos, sign in form. |
| example-5 | 32 | **facsimile** | `Type all` of the brightness program: steps, Forms 1–2, values. |

Charts (no typewriter text): A p. 21 (the expression typed on p. 11), B p. 25 and C p. 26 (RLC
problem and formulas: F = 1/(2π)·sqrt(1/(LC)), f = 1/(2π)·sqrt(1/(LC) − R²/(4L²)), I = 2πF²Q/f,
interval = .1/f; evaluate i = I·e^(−Rt/2L)·sin 2πft for 100 intervals from t = 0), D p. 30
(Δm = −2.5 log(cos P/0.707 · 0.64/ℓ₂²)).

---

## (f) Open uncertainties

1. **One printed digit differs from recomputation** (transcribed as printed, confirmed at 400 dpi):
   example-1 x = 72, EXP mantissa `1.8586717`. e^72 = 1.858671745…·10^31; correctly rounded to 9
   digits that is 1.85867175, which would print …718 — so JOSS's own 9-digit exp(72) was evidently
   1.85867174 (one unit low in the 9th digit). Everything else in examples 1, 2 and 4 (245 lines)
   reproduces exactly with "evaluate, round to 9 significant digits, then round half-up to the
   field"; e.g. example-2 row 1.20 ms: −1.40580649… → −1.40580650 → `-1.405807`. An emulator needs
   JOSS's actual exp algorithm to hit that one last digit; intermediate results inside an
   expression were not needed at 9-digit precision to reproduce the tables.
2. **Default `Type` layout for plain `Type x.`** is inferred, not directly shown — see (a)2–3.
   Negative-number sign column, long-label behaviour and positive-exponent scientific form are
   not attested in this document.
3. **The leading `-` in form fields** (`-_.______`, `-_.__`): clearly a hyphen in both facsimiles;
   its exact rule is not explained. Fields without it (Form 1) never had to show a negative value.
4. **example-5 Form 2 middle field** prints as `_ __` (no visible point) in the facsimile; taken
   as a dropout and transcribed `_.__` (flagged in the file).
5. **Scenario is edited**: no `Delete all` between the table program and the RLC program
   (p. 15), although example-3 proves storage was cleared; p. 13 has `exp(x), in form 1.` where
   the p. 15 listing has `exp(x) in form 1.`; p. 16 has a blank where example-3 has `·` before
   `sin`; p. 11 `Type cos (…)` vs echoed `cos(…)`; Chart A has `+` where the typed line (and the
   result) have `-`.
6. **`Error above:` spacing** (two spaces after the colon) is from retyped pages only.
7. **Page layout**: the date line's column, the number of lines per page, top spacing and what a
   mid-output automatic page break looks like are not visible — the facsimiles are cropped
   clippings. Example-2's 101 lines must have crossed a page boundary but none is shown.
8. **Blank-line counts in `Type all`** (one between parts, two before forms, none between forms,
   two before values) come from examples 3 and 5, which look like paste-ups with slightly
   irregular line spacing; the retyped p. 15 listing shows a different arrangement (one blank
   everywhere, including between forms).
9. **Demand line** (p. 18): the gap between `=` and the user's entry is not recoverable; whether
   JOSS types a space after `=` is unknown.
10. **Left margin of examples 1, 2, 4**: column 0 is inferred from the form pictures (the
    clippings show no margin). Columns *within* each line are measured and certain.
11. Appendix B quotations in (d) are from OCR and were only spot-checked against the images.
