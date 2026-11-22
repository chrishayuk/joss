# P-3149 — Shaw, "JOSS: Experience with an Experimental Computing Service for Users at Remote Typewriter Consoles", May 1965

Transcripts: `fig-1.txt` (PDF p.6), `fig-2.txt` (PDF p.8), `fig-3.txt` (PDF p.11, the one-page summary).
Only the figures were transcribed (prose not re-read, as instructed).

## Conventions

- `U: ` user input, `J: ` JOSS output; column 0 = 4th character. Figures 1 and 2 have no margin labels; U/J was
  assigned from print weight (JOSS output is bolder) together with content — unambiguous for every line.
- `U:` / `J:` alone = blank line. In fig-1 the blanks inside/around the `Type all steps.` listing are `J:`;
  the blanks between the user's stored parts and the one before `Type all steps.` are written `U:` (the last
  could be either).
- `fig-3.txt` is not a session; each line is prefixed `T: `. Its columns are approximate (see below).
- `#` lines are comments.

## Measurement

400 dpi bilevel renders. Figures 1 and 2 are reduced: pitch 30.35 px (≈ 76 % of 10 cpi), line pitch 50.1 px.
Each line was placed on a character grid and the transcription machine-checked cell by cell (all pass apart
from form underscores, which print low and bleed into the following `Go.` line). User and JOSS text share one
column origin here (single-layer photograph), so columns are absolute. This figure is the independent
confirmation that JOSS step listings and messages start in col 0.

## (a) Default `Type` layout (May 1965)

Fig. 1, 15 lines `       x =       2` … `       x =      47`:
`x` col 7, `=` col 9, units digit col 17 (two-digit values cols 16–17).
→ label right-justified to col 7, ` = `, number with units digit in col 17. Same as P-3131 (Apr 1965);
one column wider than P-2922 (Aug 1964, units col 16).

### Forms (Fig. 2, measured from glyph runs)

    a =___   b =___   c =___   roots:    .____    .____      fields 3–5, 12–14, 21–23, 37–41, 46–50
    a =___   b =___   c =___   roots: ___.________.____      fields …, 34–45(!), 46–50 → "fields run together"
    a =___   b =___   c =___   roots: ___.____ ___.____      fields …, 34–41 (point 37), 43–50 (point 46)
    a =  1   b = -1   c = -6   roots:   3.0000  -2.0000
    a =  1   b = -1   c = -1   roots:   1.6180   -.6180

- Fields may abut the preceding text (`=___`).
- Integer fields: right-justified, floating minus (` -1` in a 3-wide field).
- A field with no integer positions (`.____`) cannot hold 3.0 → `I can't express value in your form.`
- Two fields with no blank between them → `Your fields run together in the form.`
- Fewer expressions than fields → `I have too few values for the form.`
- Missing form → `I can't find the required form.`
- No leading zero: `-.6180` (minus in col 45, point col 46).

### Listing

`Type all steps.` → blank line, steps flush left with a blank line between parts, blank line.

## (b) JOSS messages seen in Fig. 2 (verbatim; two spaces after the colon; in order of appearance)

| text | provoked by |
|---|---|
| `Eh?` | `Do part1.1.` (missing space) |
| `Error above:  Part number must be integer and 0<part<10*9.` | `Do part 1.1.` |
| `Error at step 1.1:  a is undefined.` | `Do part 1.` |
| `Error at step 1.1:  I have a zero divisor.` | `Go.` after `Set a = 0.` |
| `Error at step 1.1:  I have a negative argument for sqrt.` | `Go.` after `Set a = -1.` |
| `Error at step 1.3:  I can't find the required form.` | `Go.` after `Set a = 1.` |
| `Error at step 1.3:  I have too few values for the form.` | `Go.` (form has 5 fields, step types 2 values) |
| `Error at step 1.3:  I can't express value in your form.` | `Go.` (fields `.____`) |
| `Error at step 1.3:  Your fields run together in the form.` | `Go.` |
| `Error above:  Illegal set of values for iteration.` | `Do part 1 for c = -1(1)-4.` |
| `I need more storage space.` | `Do step 2.1.` with `2.1 Do step 2.1 for i = 1(1)100.` (recursion) — no `Error …:` prefix |
| `It's a mess.  Let's start over.` | `Go.` after `Delete step 2.1.` (two spaces after "mess.") |
| `Error above:  You haven't told me to do anything yet.` | `Go.` with nothing to resume |
| `Please wind up your work and turn off as soon as possible.` | operator/system broadcast, unsolicited |

Behavioural facts visible in Fig. 2: `Go.` resumes at the failed step after the user fixes the cause (the step
is re-executed, e.g. step 1.1 re-run three times); retyping `Form 1:` or a step replaces it; after an error in
an indirect step the partial `Do` is still pending for `Go.`; after "It's a mess" nothing is pending.
`Do part 1 for c = -1(-1)-4.` iterates c = -1,-2,-3,-4 (negative step). Roots computed correctly
(`1.6180`, `-.6180` = (1±√5)/2 rounded to 4 places).

## (c) Statement forms / verbs / functions / syntax seen

Fig. 1: `*` comment line; `1.1 Do part 2 for d = 2(1)ip[sqrt(x)] if x > 3.` (Do with for-range AND if-clause;
`ip[…]` with brackets as function parentheses); `1.2 Type x if x ≠ 0.`; `2.1 Set x = 0 if fp(x/d) = 0.`;
`Do part 1 for x = 2(1)50.`; `Type all steps.`; `Delete all.` Relations `>`, `≠`, `=`.

Fig. 2: `Set x(1) = [-b+sqrt(b*2-4·a·c)] / [2·a].` (indexed targets, brackets for grouping, spaces around `/`);
`Type x(1), x(2) in form 1.`; `Set b = -1.`; `Do part 1.`; `Go.`; `Form 1:`; `Do part 1 for c = -1(-1)-4.`;
`Do part 1 for c = -5.`; `2.1 Do step 2.1 for i = 1(1)100.`; `Do step 2.1.`; `Delete step 2.1.`; `Delete all.`

Fig. 3 (the 1/12/65 one-page summary; complete inventory — see `fig-3.txt`):
- DIRECT or INDIRECT: `Set x=a.` · `Do step 1.1.` · `Do step 1.1 for x = a, b, c(d)e.` · `Do part 1.` ·
  `Do part 1 for x = a(b)c(d)e, f, g.` · `Type a,b,c,_.` · `Type a,b in form 2.` · `Type "ABCDE".` ·
  `Type step 1.1.` · `Type part 1.` · `Type form 2.` · `Type all steps.` · `Type all parts.` · `Type all forms.` ·
  `Type all values.` · `Type all.` · `Type size.` · `Type time.` · `Type users.` · `Delete x,y.` ·
  `Delete all values.` · `Line.` · `Page.`
- INDIRECT (only): `1.1 To step 3.1.` · `1.1 To part 3.` · `1.1 Done.` · `1.1 Stop.` · `1.1 Demand x.`
- DIRECT (only): `Cancel.` · `Delete step 1.1.` · `Delete part 1.` · `Delete form 2.` · `Delete all steps.` ·
  `Delete all parts.` · `Delete all forms.` · `Delete all.` · `Go.` · `Form 2:` + form line
  (`dist. = ..........  accel. = __.___`) · `x=a` (short Set — typed without `Set` and without final period).
- RELATIONS: `= ≠ ≤ ≥ < >`. OPERATIONS: `+ - · / * () [] ||`. CONDITIONS: `if a<b<c and d=e or f≠g`
  (chained relations, `and`, `or`).
- FUNCTIONS: `sqrt(a)` (square root), `log(a)` (natural logarithm), `exp(a)`, `sin(a)`, `cos(a)`,
  `arg(a,b)` (argument of point [a,b]), `ip(a)` (integer part), `fp(a)` (fraction part), `dp(a)` (digit part),
  `xp(a)` (exponent part), `sgn(a)` (sign), `max(a,b)`, `min(a,b,c)`.
- PUNCTUATION and SPECIAL CHARACTERS: `. , ; : ' " _ # $ ?`

## (d) Uncertain readings

- fig-3: `dist. = ..........` — 10 dots counted (9 possible); form field `__.___` (2 + 3).
- fig-3: `___._____ indicates a field…` — 3 underscores, point, 5 underscores (underscores merge; from width).
- fig-3 column positions are approximate: the sheet is a typed original that is slightly skewed and the two
  columns were normalised to cols 2 and 38 (headings at 0 and 36); vertical line positions are exact.
- fig-1: the blank line before `Type all steps.` could be user or JOSS.

## (e) Behaviour statements appearing in the figures (Fig. 3, PDF p.11, verbatim)

- "___._____ indicates a field for a number in a form."
- "......... indicates scientific notation in a form."
- "# is the strike-out symbol."
- "$ carries the value of the current line number."
- "* at the beginning or end kills an instruction line."
- "Brackets may be used above in place of parentheses."
- "Indexed letters (e.g. v(a), w[a,b]) may be used above in place of x, y."
- "Arbitrary expressions ( e.g. 3·[sin(2·p+3)-q]+r ) may be used above in place of a, b, c, ... ."
- Fig. 2 message (PDF p.8): "Part number must be integer and 0<part<10*9."

Prose of the paper not re-read (out of scope per the request).
