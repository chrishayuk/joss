# P-3131 — Shaw, "JOSS: Examples of the Use of an Experimental On-Line Computing Service", April 1965

Transcripts: `page-03.txt` … `page-10.txt` (PDF page numbers; printed page numbers are -2- … -9-).
PDF pages 1, 2, 11 are title, introduction and references (no sessions).

## Conventions used in the transcript files

- `U: ` user input (green in the original), `J: ` JOSS output (black). Text starts in the 4th character; typewriter column 0 is the 4th character.
- `J:` / `U:` alone = blank typewriter line. Blank paper has no colour, so attribution is inferred:
  a blank is written `U:` only when it lies between two user lines; otherwise `J:`. JOSS-certain blanks are
  the ones produced by `Line.` steps and those inside/around `Type all …` listings; the single blank after
  each `Type form 1.` output / before a following user line is not certain.
- `U+ text` = user input typed on the SAME typewriter line as the preceding `J:` record, starting in the column
  right after the J text (Demand prompts; the J text ends with one space, so the reply starts at col 11).
- Lines starting `#` are comments (`# ??` = uncertainty about the line immediately above).
- First line of each file is a `#` provenance comment.
- The RAND report page number ("-2-" etc., upper right, 3–4 lines below the heading) is not JOSS output and is omitted.
- Characters: `·` multiplication dot, `*` exponent, `≥`, `|`, `_` form underscore, `"` quotes.

## How columns were measured

Pages 3–9 are 300 dpi colour scans (pitch 30.1 px = 10 cpi, line pitch 49.85 px = 6 lpi); page 10 is 400 dpi bilevel.
Green and black were separated by colour, each layer was put on its own character grid (per-line phase
tracking), and every transcribed line was machine-checked against the ink occupancy of each character cell
(all lines pass; only a few flagged cells from wide-glyph spill).

IMPORTANT caveat: the black (JOSS) and green (user) images are mis-registered against each other in the
reproduction by a fraction of a character that varies by page (black is +0.4 … +0.95 cell right of green on
printed pages 2–7, −0.1 cell on page 8; vertically 5–11 px). The absolute J column origin was therefore fixed
per page from JOSS lines that are flush left (page heading, `Eh?`, `Error …`, step listings from `Type all`).
With that origin everything is self-consistent (form output lands exactly in the fields of user-typed forms;
JOSS-typed copies of forms coincide with user-typed forms). Relative columns inside any J line (e.g. distance
from `=` to the number) do not depend on this assumption. P-3149 Fig. 1 (single-layer photo) independently
confirms col 0 for step listings and the `x =` layout.

## (a) Default `Type` layout (JOSS as of 28 Apr 1965) — columns 0-based

Rule that fits every line:

    label right-justified in cols 0–7, space (col 8), "=" in col 9, space (col 10),
    then the number: units digit of the integer part in col 17, decimal point in col 18.

- A minus sign floats: it is printed immediately left of the first digit (or of the point).
- No leading zero: fractions print as `.456` with the point in col 18.
- Integers print without point; nothing after the units digit.
- 9 significant digits, trailing zeros suppressed (`1.5`, `2.30258509`, `.198669331`, `10.0788675`,
  `.0992174964` — the last has a non-significant 0 after the point, then 9 significant digits).
- Long labels: if the label is longer than 8 characters it starts at col 0 and pushes ` = ` right; the number
  stays with its units digit in col 17 as long as there is room, otherwise it follows `= ` after one more
  blank (the sign position), i.e. `label = ` + sign-slot + digits.

Evidence (pNN = `page-NN.txt`, i.e. PDF page number; measured columns):

| line | label cols | `=` col | number cols |
|---|---|---|---|
| p03 `     2+2 =       4` | 5–7 | 9 | 17 |
| p03 `       x =       3` | 7 | 9 | 17 |
| p03 `     x/2 =       1.5` | 5–7 | 9 | 17–19 (point 18) |
| p03 ` log(10) =       2.30258509` | 1–7 | 9 | 17–26 |
| p03 ` sin(.2) =        .198669331` | 1–7 | 9 | 18–27 (point 18) |
| p03 `arg(3,3) =        .785398163` | 0–7 | 9 | 18–27 |
| p03 `sqrt(144) =     12` (9-char label) | 0–8 | 10 | 16–17 |
| p03 `sgn(-3.5) =     -1` | 0–8 | 10 | `-` 16, `1` 17 |
| p03 `4·arg(3,3) =     3.14159265` (10) | 0–9 | 11 | 17–26 |
| p03 `1.23456·10*2 =  123.456` (12) | 0–11 | 13 | 16–22 (units 18) |
| p03 `max(6·x, 20, x*3) =  27` (17) | 0–16 | 18 | 21–22 |
| p03 `[(|x-5|·3+4)·2-15]·3+10 =  25` (23) | 0–22 | 24 | 27–28 |
| p03 `       y =     123.456` | 7 | 9 | 15–21 (units 17) |
| p03 `   fp(y) =        .456` | 3–7 | 9 | 18–21 |
| p05 `       h =        .1` | 7 | 9 | 18–19 |
| p05 `       x =      10.0788675` | 7 | 9 | 16–25 |
| p05 `       y =        .0992174964` | 7 | 9 | 18–28 |
| p05 `       X =      10` | 7 | 9 | 16–17 |

Not observed in this paper: default-format scientific notation, negative fractions, numbers ≥ 10^9.
(Compare P-2922, Aug 1964: same layout but one column narrower — no blank after `=`; see that NOTES.md.)

`Type all values.` / the values part of `Type all.`: one blank line, then one `name = value` line per
variable in the default layout, then one blank line. Order seen: `x, y` (p03) and `h, p, q, x, y, X, Y` (p05),
i.e. lower-case letters alphabetically, then upper-case.

### Formatted output (`Type … in form n`)

- A field is a run of `_` with at most one `.`; text between fields is copied literally.
- Numbers are right-justified in the field with the decimal point on the form's point; value is ROUNDED to the
  field's decimals (`2.30259` for log 10 = 2.302585…, `1.414214`, `3.605551`, `-.000092`).
- Sign floats immediately left of the first digit or point (`-3.627613`, ` -.326314`).
- No leading zero before the point (`.500000`, `.00000`, `-.005521`); zero prints `.000000`.
- Integer fields (`__`, `____`) print integers right-justified (`a =  4`, `a = 12`, `1296`).
- A field of n dots is scientific notation: n = 11 gives blank/sign, `d.ddddd`, blank, two-digit exponent
  (`2.71828 00`, `4.85165 08`) — mantissa cols 40–46, exponent cols 48–49 for dots in cols 39–49.
- Measured forms and outputs (cols):
  - p04 `a = __   b = __   c = ___.______` → fields 4–5, 13–14, 22–31 (point 25); `a =  1   b =  1   c =   1.414214`.
  - p05 `__   _._____` → fields 0–1, 5–11 (point 6); ` 1    .00000`, `10   2.30259`.
  - p06 `x = __.______   y = __.______` → fields 4–12 (point 6), 20–28 (point 22).
  - p07/p08 `__._____  __._____  __._____  __._____` → fields 0–7, 10–17, 20–27, 30–37; row ` 3.58200  -.67000   .87300  1.05500`.
  - p08 `ATOM = ____   sqrt(ATOM) = __` → fields 7–10, 27–28.
  - p09/p10 `__   ___   _____   _._____   _._____   ...........` → fields 0–1, 5–7, 11–15, 19–25, 29–35, 39–49.
- `Type form 1.` prints just the form line (no `Form 1:` caption). `Type all.` prints `Form 1:` + the form line.

### Listings and spacing (line-exact)

- `Type all steps.` → blank, steps (flush left, blank line between parts), blank.
- `Type all.` → blank, steps (blank between parts), 2 blanks, each form as `Form n:` + form line with 1 blank
  between forms, 2 blanks, values, (1 blank after the values on p05; on p09 with no values there are 2 blanks
  after the last form before the next user line).
- `Type step 1.4.` → the step, no blank lines.
- Demand prompt: name right-justified so that `=` is in col 9 (`  a(1,1) = `), one blank, reply typed from col 11.
- `Type "Root".` prints `Root` at col 0. `Type users.` prints `6` at col 0.
- Page heading typed in black at the top of every sheet: `hhmm m-dd-yy III` at col 0 (e.g. `1230 4-28-65 JCS`),
  followed by 5 blank lines; text resumes on the 7th line. (Whether anything precedes the heading on the sheet
  is not visible.) After step `Page.` the output continues on a new sheet with a new heading (p09 → p10).
  Session ran 1230–1341.

## (b) JOSS messages seen (verbatim; note TWO spaces after the colon)

| text | where |
|---|---|
| `Eh?` | p03, after `Type 2+2+.` |
| `Error above:  x is undefined.` | p03, after `Type x.` |
| `Error at step 1.4:  e is undefined.` | p06, during `Do part 1 for d=-.05.` |

## (c) Statement forms / verbs / functions / syntax seen

- Direct: `Type expr[, expr…].`, `Type … in form 1.`, `Type all values.`, `Type all steps.`, `Type all.`,
  `Type form 1.`, `Type step 1.4.`, `Type users.`, `Set x=3.`, `Do part 2 for a=1(1)4.`, `Do part 1 for X=1(h)10.`,
  `Do part 2 for x=3.5, 4.` (value list), `Do part 2 for x=.5(.5)3.`, `Do part 1 for d=-.05.` (single value),
  `Do part 2 for x=x-.000001.`, `Do step 1 for x=32(1)99.`, `Go.` (resume after error), `Delete all values.`,
  `Delete all.`, `Delete step 2.2.`, `Form 1:` followed by the form on the next line.
- Stored steps: `1.1 Set …`, `1.2 Type a,b,c in form 1 if fp(c)=0.`, `2.1 Do part 1 for b=1(1)a.`, `2.2 Line.`,
  `1.4 To part 1 if |y|≥e.`, `1.5 Type "Root".`, `3.1 Done if i=n.`, `11.1 Demand a(i,j).`, `2.1 Page.`,
  `2.5 Type form 2.`, `1.2 Line if fp(x/5) = 1/5.`, `1.1 Do part 2 if fp(x/20) = 1/20.`,
  integer step number `1 Type x*2, x in form 1 if … = x.` (step "1", run with `Do step 1 for …`).
  Retyping a step number replaces the step (p04 `1.2 …`).
- Functions: `sqrt log exp sin cos arg(a,b) sgn max(…) min(…) ip fp dp xp`; absolute value `|x|`; brackets `[ ]`
  interchangeable with `( )`; operators `+ - · / *`(power); relations `=`, `≥`; numbers like `.5`, `1.23456·10*2`.
- Variables: single letters, case-sensitive (`x`/`X`, `y`/`Y`, `n`/`N`), doubly indexed `a(i,j)`.
- Comment lines: `*` in col 0 (JOSS ignores the line). `*` at the end of a line kills it (`Do part 12 for *`).
- Backspace + strikeover editing (p05, p06, p07; see comments in those files).
- Spaces optional around `=` and after commas (`Set x=3.`, `fp(x/20) = 1/20`).

## (d) Uncertain readings

- p06 last output line `x =  3.126757   y =  -.000015`: lower half clipped; digits read from the top halves.
- p05 user-typed form line and p04 user-typed form line: underscores print low and are overprinted by the
  next line; taken from measured cells and the clear JOSS-typed copies.
- Strikeovers (p05 `log(10)`, p06 `|y|`, p07 `output`): transcribed as the evidently intended text; the
  struck-over character is described in a comment.
- Blank-line attribution (U vs J) and the J column origin: see caveats above.
- p10 (bilevel scan): colours not distinguishable; whole page is JOSS output by context.

## (e) Prose statements specifying behaviour (the paper has only one page of prose)

PDF p.2 (printed -1-):
- "…examples of the use of JOSS (Johnniac Open-Shop System) for the solution of small numerical problems.
  This experimental on-line time-shared computing service is used daily by staff members of The RAND
  Corporation. The service has been available at ten remote consoles since January 1964."
- "Since JOSS ignores input lines beginning with an asterisk, we use this device to interpose comments in the
  examples. On the original copy, output is in black, input is in green."

Statements made by the user inside the sessions (as `*` comments):
- PDF p.5: "Notice above that JOSS allows backspacing and strikeovers."
- PDF p.7: "An asterisk at the end can kill the line... I forgot something:"
- PDF p.7: "JOSS can help with the input:" (introducing `Demand`).
- PDF p.4: "Modification to type only integral c's:" (a conditional `Type … if fp(c)=0`).

No statements about number range, index limits, page length or `$`/`#` occur in this paper
(see P-3149 Fig. 3 transcript for `#`, `$`, `*`).
