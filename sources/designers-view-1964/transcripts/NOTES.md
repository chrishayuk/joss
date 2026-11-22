# P-2922 — Shaw, "JOSS: A Designer's View of an Experimental On-Line Computing System", August 1964

Transcripts: `fig-3a.txt` … `fig-3i.txt` (Figures 3a–3i, PDF pages 16–24, printed pages -15- … -23-).
Only the figures were transcribed (prose not re-read, as instructed).

## Conventions

- `U: ` / `J: ` as labelled in the figure margins (caption on 3a: "U - Denotes inputs of the JOSS user; normally
  typed in green. J - Denotes outputs from JOSS; normally typed in black."). Column 0 = 4th character.
- `U:` / `J:` alone = blank typewriter line. The margin labels do not cover blank lines. Blanks are written `J:`
  where JOSS evidently produced them (`Line`, the `__` item in a Type list, blanks in/around `Type all …`
  listings and `Type all values`), otherwise `U:` (e.g. the blank separating two exchanges in 3a–3c, 3e
  before `Delete part 2.`): those could equally be editorial or JOSS; unknown.
- `U+ text` = user reply typed on the same typewriter line as the preceding `J:` prompt (3h, margin label "J/U:").
- `#` lines are comments; first line of each file is a provenance comment.

## Measurement

400 dpi bilevel renders; pitch 40.3 px (10 cpi), 66.7 px line pitch (6 lpi). The figures are reproduced
full size. Every line was put on a character grid with its own phase and each transcribed line was
machine-checked cell by cell against ink occupancy (all pass). User and JOSS lines share the same column
origin to within 0.1–0.2 cell in these figures (they overprint vertically in places — different colours on the
original — but are not shifted horizontally), so J columns are absolute here.

## (a) Default `Type` layout — August 1964 version (columns 0-based)

    label right-justified in cols 0–7, space (col 8), "=" in col 9,
    then the number with the units digit of its integer part in col 16, decimal point in col 17.

This is ONE COLUMN NARROWER than in the April/May 1965 papers (units col 17, point col 18): in 1964 there is
no blank after `=` before the number field. Equivalent statement: `label =` followed by sign-slot + digits,
right-aligned to units col 16; when the label is long the number follows `=` after exactly one position
(the sign slot): `sin(.5)*2+cos(.5)*2 = 1`, `1.23456·10*2 = 123.456`, `[(|x-5|·3+4)·2-15]·3+10 = 25`
(1965 prints two blanks there).

Other rules (same as 1965): floating minus sign directly left of the first digit (`sgn(-3.5) =    -1`: `-` col 15);
no leading zero (`.69314718`, point col 17); integers without point; 9 significant digits with trailing zeros
dropped (`7.3890561`, `20.0855369`, `21.2132034`, `1.5`); labels longer than 8 start at col 0 and push `=`
right while the number keeps units col 16 if it fits.

Scientific notation in default format: `  exp(x) =      2.68811714·10*43` — mantissa units digit in col 16,
point col 17, 9 significant digits, then `·10*43` immediately (cols 16–31). Only this one example.

Evidence (measured columns):

| line | label | `=` | number |
|---|---|---|---|
| 3a `     2+2 =      4` | 5–7 | 9 | 16 (glyph centres: `=` 1166 px, `4` 1449 px → 7.02 cells apart) |
| 3a `     x/2 =      1.5` | 5–7 | 9 | 16–18 |
| 3a `[(|x-5|·3+4)·2-15]·3+10 = 25` | 0–22 | 24 | 26–27 |
| 3b ` sqrt(3) =      1.73205081` | 1–7 | 9 | 16–25 |
| 3b `  log(2) =       .69314718` | 2–7 | 9 | 17–25 |
| 3b `sin(.5)*2+cos(.5)*2 = 1` | 0–18 | 20 | 22 |
| 3b `arg(-1,0) =     3.14159265` (9-char label) | 0–8 | 10 | 16–25 |
| 3b `4·arg(3,3) =    3.14159265` (10) | 0–9 | 11 | 16–25 |
| 3c `       y =    123.456` | 7 | 9 | 14–20 |
| 3c `   fp(y) =       .456` | 3–7 | 9 | 17–20 |
| 3c `1.23456·10*2 = 123.456` (12) | 0–11 | 13 | 15–21 |
| 3c `sgn(-3.5) =    -1` | 0–8 | 10 | 15–16 |
| 3c `max(1,2,3) =    3` (10) | 0–9 | 11 | 16 |
| 3d `       c =      4.24264069` | 7 | 9 | 16–25 |
| 3e `       c =     21.2132034` | 7 | 9 | 15–24 |
| 3f `       x =    100` | 7 | 9 | 14–16 |
| 3f `  exp(x) =      2.68811714·10*43` | 2–7 | 9 | 16–31 |
| 3h `       s =   1924` | 7 | 9 | 13–16 |
| 3h `    b(1) =       .282224532` | 4–7 | 9 | 17–26 |

### Forms (measured)

- 3d/3e `a = __   b = __   c = __.___` → fields 4–5, 13–14, 22–27 (point 24); rows `a =  1   b =  1   c =  1.414`,
  `a = 12   b =  5   c = 13.000`. Values rounded to the field (3.606, 4.243).
- 3g `_.__   _.___   _.____   .........` → fields 0–3, 7–11, 15–20, dots 24–32. Row `8.50   2.915   2.1401    4.915 03`:
  for a 9-dot field: col 24 blank (sign), mantissa `d.ddd` cols 25–29, blank col 30, 2-digit exponent cols 31–32.
- 3i Form 61 `  t      Integral` (pure text form, typed out by `Type form 61.`);
  Form 62 `_.__     _.____` → fields 0–3 (point 1), 9–14 (point 10); rows ` .00      .0000`, `4.00      .5000`
  (zero prints `.00`, no leading zero).

### Listing / spacing behaviour seen (line-exact in the figures)

- `Type all steps.` → blank, steps with a blank line between parts, blank.
- `Type all forms.` → blank, `Form 1:`, form line, blank.
- `Type all.` → blank, steps (blank between parts), 2 blanks, `Form 1:` + form line, 2 blanks, values in default layout.
- `Type all values.` → blank, values, blank.
- `Type part 2.` → the steps only, no blank lines (3e). `Type b.` for an indexed letter lists all elements
  `b(1)…b(4)`, no blank lines (3h).
- `__` item in a Type list (`Type x, sqrt(x), log(x), exp(x), __.`) → a blank line after the values (3f).
- `Line` → blank line (3e conditional `2.15 Line if fp(c)=0.`; 3i `6.2 Line.`).
- Demand prompt `    b(1) = ` (`=` in col 9, one blank), reply typed from col 11 (3h).

## (b) JOSS messages seen (verbatim, two spaces after the colon)

| text | where |
|---|---|
| `Error above:  Negative argument for sqrt.` | 3b, after `Type sqrt(-1).` |
| `Error above:  No such part.` | 3e, after `Type part 2.` (part deleted) |

(Note the 1965 wording for the sqrt case inside a step is `I have a negative argument for sqrt.` — P-3149 Fig. 2.)

## (c) Statement forms / verbs / functions / syntax seen

- `Type` with expression lists; `Type … in form n.`; `Type all steps.`, `Type all forms.`, `Type all.`,
  `Type all values.`, `Type part 2.`, `Type form 61.` (as step), `Type s.`, `Type b.` (whole indexed letter),
  blank item `__` (3f).
- `Set x=3.` / `Set y = 123.456.` (spaces optional), indexed target `5.1 Set b(i) = b(i)/s.`
- `Do part 1 for a=1(1)3.`, `Do part 6.` (no for), `Do step 3.1 for x=1,2,3,100.`,
  `Do step 3.1 for x = 8.50(.01)8.54(.02)8.60, 9, 9.5.` (chained ranges + list), `Do part 7 for t=0(h)4.`,
  `Do part 8 for x = t+h·(.5-k), t+h·(.5+k).`, `Do step 4.2 for i=1(1)4.`
- `Delete all.`, `Delete part 2.`
- `Line.`, `Line if fp(c)=0.`, `Demand b(i).`, conditional `Type … if fp(t/.5)=0.`
- `Form n:` then the form on the next line; form numbers up to at least 62; step numbers with two decimals (`2.15`)
  inserted between 2.1 and 2.2.
- Functions: `sqrt log exp sin cos arg(a,b) ip fp dp xp sgn max min`; `|…|`; `[ ]`; `+ - · / *`; `1.23456·10*2`.

## (d) Uncertain readings

- 3f: the blank item after `exp(x),` is a 2-cell-wide underscore → transcribed `__.` (single `_` not excluded by
  the typography alone, but the measured width is 2.1 cells).
- 3f: a final blank line after the x=100 block is presumably there but hidden by the figure frame; not transcribed.
- Blank-line attribution U vs J (see Conventions).
- Trailing blank lines at the end of figures cannot be seen and are not transcribed.

## (e) Prose

Not re-read (out of scope per the request).
