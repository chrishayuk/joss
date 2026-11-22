# Audit of RAND internal JOSS material (Bitsavers scans)

Purpose: an independent evidence stream on the externally visible behaviour of JOSS-I
(JOHNNIAC, 1964-65). Page numbers are PDF page numbers. Each section separates what
was **read with confidence** from what is **uncertain**.

How the message texts were obtained: the listings store each message as octal data
words, five 8-bit characters per 40-bit word. I decoded those words with the
listing's own character table (region A), so lower/upper case, `≤ < ≥ |` and spacing
come from the data itself. The upper-case comments printed beside each message were
used only as a cross-check (the line printer shows `≤ < ≥` as `-`). For the Jan-65
listing the octal was machine-read from 400-dpi renders and checked against the
comments; for the undated listing (ruled paper defeats OCR) I read the octal by eye.

Transcription files written:

- `program-notes-and-tests/transcripts/` — `page-01.txt`, `page-02.txt`, `page-03.txt`, `page-04.txt`, `page-05.txt`, `page-06-09.txt`, `page-11.txt`, `page-13-14.txt`, `README.txt`
- `listing-1965/transcripts/messages.txt`
- `listing-undated/transcripts/messages.txt`

---

## 1. `program-notes-and-tests/JOSS_Program_Notes_and_Tests.pdf` (43 pp.)

### What it is

A loose bundle of J. C. Shaw's working papers, not one document, and **it contains no
JOSS console sessions** (no user/JOSS dialogue anywhere), so no `U:`/`J:` transcripts
could be made.

| Pages | Content |
|---|---|
| 1-3 | Typed one-line-per-command summary of the JOSS language with handwritten amendments. Undated. The typed layer is the older language (`coef`, `sfx`; no Demand); the handwriting brings it to the Jan-65 language (`dp`, `xp`, Demand, Punch, Read deck). |
| 4 | Typed "JOSS Regions" (program region letters). |
| 5 | Typed "JOSS 412 Storage Assignment" (core map). |
| 6-9 | Typed list of subroutines `= 0` to `= 80`, drum routines D0-D8, G5, G6. |
| 10 | Printed "List of JOHNNIAC Operations — June 1, 1963" (D(L)-3058). |
| 11 | Test strike of a new typewriter ball: the whole JOSS character set. Handwritten notes "Cliff! It came in. Mal" and a reply initialled CLB. |
| 12 | Blank or near-blank (not examined closely). |
| 13 | Handwritten, "JCS 11-19-64": note on range reduction in routine F1 (log). |
| 14 | Handwritten, "JCS 10-10-64": "Error msg construction". |
| 15-43 | Octal core dumps with handwritten labels: test routines for the multiple-typewriter communication system (the "Tests" of the title). Page 18 is a handwritten index. Not JOSS; not transcribed. |

Dates found: 1 June 1963 (order-code chart), 10-10-64 and 11-19-64 (Shaw notes). Author
of the notes: J. C. Shaw ("JCS"); C. L. Baker ("CLB", "Cliff") appears on p. 11.

### Read with confidence (pp. 1-3, typed text)

Command inventory, exactly as classified on p. 1:

- **Direct only:** `Cancel.` `Go.` `Form 1:` `Delete step 1.1.` `Delete part 1.` `Delete form 1.` `Delete all steps.` `Delete all parts.` `Delete all forms.` `Delete all.`; handwritten additions `Punch.`, `Read deck 3173.`, `x=3` (short Set).
- **Indirect only:** `To step 3.7.` `To part 3.` `Done.` `Stop.`; handwritten `Demand x.`
- **Both:** `Line.` `Page.` `Set x=y+1.` `Do part 1.` (hw: `Do step 1.1.`) `Delete a, b, c.` (hw: `Delete all values.`, moved here from "direct only") `Type step 1.1.` `Type part 1.` `Type form 1.` `Type all steps.` `Type all parts.` `Type all forms.` `Type all values.` `Type "ok".` `Type size.` `Type a,_,b, c in form 1.`; handwritten `Type a, b, V(3), V, A(i,j), A, _.` and `Type all.`
- A typed `Type 2+2, x, y+1.` is struck out by pen (reason unknown; the Jan-65 code does accept expression lists).

Functions (p. 2): `sqrt(x) log(x) exp(x) sin(x) cos(x) arg(x,y) ip(x) fp(x) sgn(x) max(a,b,...,z) min(a,b,...,z)`; typed `coef(x)` and `sfx(x)` struck and replaced by hand with `dp(x)` and `xp(x)`.

Rules stated in prose (p. 2-3), verbatim in the transcripts:

- "The conditional clause may be used at the end of any legitimate step." Example `if a<b or c=d and e≤x≤f.` "All six numerical relations may be used. The "and" and "or" are evaluated from left to right." (So: chained relations allowed; no and/or precedence.)
- Brackets and parentheses interchangeable; absolute-value bars group in expressions but may not enclose subscripts or arguments.
- "The dollar sign may be used to refer to the line number on the page".
- "An asterisk at either end of the input line kills the line unless it is a form."
- Operators `x+y x-y x·y x/y x*y`.
- "52 letters for identifiers. Any letter may be singly or doubly subscripted. No declarations. A new assignment overrides a previous assignment even to the extent of changing dimensions. Subscripts must be integers from 0 to 99."
- Task states: "A task may be completed or stopped or interrupted or errored. The last three changes to "green" are flagged by messages. Only one suspended task is allowed and that may be continued by Go or cancelled by Cancel or Do."
- "Eight slots on the drum for users. 1024 words for each user which covers tables, steps, forms and values."
- Forms: underscores = fixed-point field; "A string of periods" = scientific field ("only the coefficient and scale factor exponent will appear"); "In both cases numbers will be rounded to fit."
- Numbers: "Nine decimal coefficient with sign and two decimal scale factor exponent with sign. All constraints are in decimal ... there are no special representations for "integer", "boolean", etc."
- Handwritten: `for x = 1(1)10(5)100, 7.3, 8.49, 15(.01)15.2.` (range syntax; several ranges and single values in one `for`).

Character set (p. 11), key pairs lower/upper:
`1' 2" 3# 4$ 5≤ 6≥ 7< 8> 9( 0) -_ +*` / `qQ … pP ·|` / `aA … lL ;: =≠` / `zZ … mM ,[ .] /?`.
Baker's note: with the elite ball "you now can get a full 80 character input line and margins".

Shaw's 10-10-64 note (p. 14): error line budget "Error during step ____:__" = 5+1+6+1+4+1+10+1+2 = 31 characters; 72 − 31 = "41 characters for error msg. exclusive of CR+EOM." and "This is the major reason for keeping 1≤step<10^9 so that we wouldn't have to copy wild expressions into msg and take up to 3 lines." Below it he hand-encodes "I need more storage space." — i.e. the personal wording was being put in around 10 October 1964.

### Uncertain

- `Read deck 3173.` — the deck number is handwritten and could be `3/73`.
- "JOSS 412" on p. 5 (and "JOSS 666" on the Jan-65 listing) — I read these as version/assembly tags but nothing explains them.
- Pages 4-9 and the table on p. 5 were transcribed from OCR of clean typescript and only spot-checked; individual numbers on p. 5 are not re-verified.
- Handwritten values `7.3`, `8.49` in the for-clause example; the struck-out word under it ("Condition values"?).

---

## 2. `listing-1965/JOSS_666_Listing_Jan65.pdf` (219 pp.)

### (a) What it is

The assembly listing of JOSS itself for JOHNNIAC, headed by hand "JOSS 666 — JCS 1-9-65"
(p. 5), i.e. J. C. Shaw, 9 January 1965. Symbolic "Style E" JOHNNIAC assembler: each
line has a card ID, a location, two half-word instructions, and an upper-case comment.
This is the production JOSS-I with the personal ("I have…") messages.

Structure (PDF pages):

| Pages | Content |
|---|---|
| 1 | Blank "JOHNNIAC-Style E" coding form |
| 2 | List of JOHNNIAC Operations, June 1, 1963 |
| 3-4 | Handwritten list of routines `=0`…`=80` (same as typed list in the notes) |
| 5 | Handwritten storage map "JOSS 666, JCS 1-9-65" |
| 6 | "JOSS Regions" with handwritten annotations |
| 7-9 | **A** — character table (code → class, value, properties) |
| 10-12 | **D0** error routine and error-message framing texts |
| 13-17 | **D1, D2** Delete extensions (`all …`, specific values) |
| 18-23 | **D3, D4** Type `all …`, Type all values |
| 24-25 | **D5** octal in/out (maintenance) |
| 26-29 | **D6** Type values (default layout) |
| 30-33 | **D7** Type values in a form |
| 34-36 | **D8** Type step/part/form/size/users/time/"text" |
| 37-42 | **E** error directory and message texts |
| 43-60 | **F0-F12** functions sqrt, log, exp, sin, cos, ip, fp, xp, dp, sgn, arg, max, min |
| 61-82 | **G0-G14** verbs: Set, Do, To, Done, Cancel, Delete, Type, Line, Page, Read, Punch, Form, Stop, Go, Demand |
| 83-106 | Data regions **H, I, J, K, L, N, P, Q, R, S, T, U0-U7, V, W, Z** (W = keyword table, p. 105) |
| 107-129 | **X0-X15** executive: logging, queue service, signal service/input line analysis (X2), task selection, interpret verb (X4, verb table p. 116), advance, fetch, ascend/iterate (X7), console signals and interrupts (X10), sign-on (X11), transmit |
| 130-213 | **=0 … =79** subroutines (expression evaluator =20/=21 pp. 150-152, time line =22 p. 153, condition =23 p. 154, function-name table =24 p. 155, arithmetic =31-=35 pp. 162-168, range tests =40-=46 pp. 173-179) |
| 214-219 | Region origin table, loaders, initializer, drum utilities |

The routine-to-page map was derived from OCR of the card IDs and spot-checked on about
40 pages; individual boundaries may be off by a page.

### (b) Messages — read with confidence

Full list in `listing-1965/transcripts/messages.txt`. Error directory on p. 37, texts pp. 38-42:

| # | Text |
|---|---|
| E0 | `I need more storage space.` |
| E1 | `I have an overflow.` |
| E2 | ` is undefined.` (appended to the variable name, e.g. `x is undefined.`; indices are included) |
| E3 | `I have a zero divisor.` |
| E4 | `Index value must be integer and 0≤index≤99.` |
| E5 | `Please limit numbers to 9 significant digits.` |
| E6 | `Eh?` |
| E7 | `You haven't told me to do anything yet.` |
| E8 | `I have too many values for the form.` |
| E9 | `Don't give this command indirectly.` |
| E10 | `Don't give this command directly.` |
| E11 | `Please limit step labels to 9 digits.` |
| E12 | `Form number must be integer and 0<form<10*9.` |
| E13 | `Step number must satisfy 1≤step<10*9.` |
| E14 | `I have a negative argument for sqrt.` |
| E15 | `Field definition has fewer than 7 periods.` |
| E16 | `I can't find the required step.` |
| E17 | `I can't find the required part.` |
| E18 | `Part number must be integer and 0<part<10*9.` |
| E19 | `I can't find the required form.` |
| E20 | `I have an argument ≤ 0 for log.` |
| E21 | `I have a negative base to a fractional power.` |
| E22 | `Please keep |x|<100 for sin(x) or cos(x).` |
| E23 | `I have zero to a negative power.` |
| E24 | `I have too few values for the form.` |
| E25 | `I need individual values for a form.` |
| E26 | `I can't find the part for iteration.` |
| E27 | `I can't find the step for iteration.` |
| E28 | `Illegal set of values for iteration.` |
| E29 | unused |
| E30 | `Your fields run together in the form.` |
| E31 | `I can't express value in your form.` |

Framing pieces (D0, pp. 10-12): `Error `, `during `, `at `, `step `, `above`, `:  ` (colon and **two** spaces), and `It's a mess.  Let's start over.` (two spaces after "mess.").

Other messages:

- `Stopped by step ` + number + `.` (G12, p. 79)
- `I'm at step ` + number + `.` (X10, pp. 123-124) — typed when the user interrupts a running stored-program task
- `Revoked.` (X10) — typed when the user interrupts a direct command
- `Just one to four letters please.` / `Initials please:  ` (X10, p. 124)
- `JOSS at your service.` / `Initials please:  ` (X11, p. 125)
- `  You are now number ` n ` in the queue.` (X11, p. 126)
- `Call 233 for latest JOSS information.` (X10, p. 122)
- `Please wind up your work and turn off as soon as possible.` (X10, p. 122)
- `Form` (heading for Type all forms, p. 19); `·10*` (number output, pp. 23, 29)

Count: 31 error texts + "It's a mess…" + 8 others = **40**, matching Shaw's "stock of forty canned messages".

How an error line is assembled (D0, pp. 10-11, from the code comments and jumps):

- E0 (storage) and a **direct** E6 skip the prefix: the line is just the message (`Eh?`).
- Otherwise `Error ` + location + `:  ` + message, where location is `above` for a direct command, `at step N` for a stored step, and `during step N` when the trouble arose in the iteration/ascend logic (X7) of a `Do … for`. Trouble in a top-level (direct) `Do … for` gives `above`.
- If the failing step can no longer be found, the routine types `It's a mess.  Let's start over.` and erases the whole control structure.
- E2 builds `x is undefined.` / `x(i) …` / `x(i,j) …` using the grouper style saved in J20.

### (c) Keywords — read with confidence

- **Verbs** (X4 table, p. 116; six-bit packed, first letter capital): `Set Do To Done Cancel Delete Type Line Page Read Punch Form Stop Go Demand` — 15 verbs, in that order (= G0…G14). A line that starts with no valid verb is tried as a short Set (`x=3`). There is no `Let`, `Quit`, `Reset`, `Use`, `File`, `Recall`, `Discard`.
- **Words** (W table, p. 105): `all time for form forms if in part parts step steps users values size or and` (16 words).
- **Type objects** (G6 p. 71, D8 p. 34): `all` (+ `steps`/`parts`/`forms`/`values`, or nothing), `step`, `part`, `form`, `size`, `users`, `time`, a quoted string, or a list of expressions/letters; `_` in the list gives a blank line; `in form n` suffix.
- **Functions** (=24 table, p. 155, in this order): `sqrt log exp sin cos ip fp xp dp sgn arg max min` — 13, nothing else.
- Not present: `not`, `tv`, `sum`, `prod`, `first`, `times`, `timer`, `formula`, `item`, `sparse`, `deck` (the word `deck` is gone from the W table although `Read`/`Punch` verbs remain).

### (d) Limits and numeric rules

Read with confidence:

- 9 significant digits on input: more gives E5; leading and trailing zeros beyond nine are tolerated (=20, p. 150).
- Index: integer, 0…99 (E4). Part and form numbers: integer, 0 < n < 10^9. Step: 1 ≤ step < 10^9, label at most 9 digits (E11, E13; X2 p. 113 "ERROR IF 10 OR MORE DIGITS").
- sqrt of a negative, log of x ≤ 0, |x| ≥ 100 for sin/cos, 0 to a negative power, negative base to a non-integer power are errors (E14, E20-E23). x*0 = 1 for any x including 0 (=35, p. 166 "DOES B=0 … RESULT IS ONE").
- `Do … for` ranges: zero increment, or an increment whose sign disagrees with the limits, is E28; equal limits are accepted (G1, p. 64). On stepping, when the new value equals or exceeds the limit the limit itself is used as the last value (X7, pp. 119-120).
- Scientific field in a form needs at least 7 periods (E15); adjacent fields are E30; a value that will not fit is E31; value count must match fields (E8, E24).
- Page: constant J2 = 55, "LINE NUMBER FOR EJECT" (p. 85). Each new page starts with a "time line" (date, time, initials) followed by "SIX CR'S AND EOM" (=22, p. 153).
- Default `Type` of a value (D6, pp. 26-29): the label is right-justified in 8 columns ("JUMP IF EXPRESSION IS 8 OR MORE CHARA" — longer labels are not padded), then space, `=`, space, then padding to a fixed sign column, optional `-`, digits. Scientific form is `d.dddddddd·10*n`, negative exponent in parentheses `·10*(-n)`; trailing zeros and an empty fraction are suppressed. If the value will not fit after a long label it goes on the next line.
- Input line buffer 81 characters (R block 82 words). Storage: handwritten map p. 5 gives the user block P…R plus "Available Space 720" words; drum loader uses 1024-word blocks.

Uncertain (my reading of code, not stated in comments):

- Fixed vs scientific threshold: D6 tests exponent+3 < 0 and exponent−6 ≥ 0 (constants I3, I9), which gives fixed notation for 10^-3 ≤ |x| < 10^6 and scientific otherwise. This matches the documented .001…1,000,000 rule, but I did not trace the exponent convention instruction by instruction.
- Exact columns: sign column constant is `S 17` and line-fit test `S 50` (p. 29). If S0 is column 1, that puts `=` in column 10, the sign in column 18 and the decimal point in column 20. Treat as a hypothesis to check against printed sessions in P-2922 / RM-4162.
- Whether the visible page is 54 lines: the counter ejects at 55, but I did not establish whether the count starts at 0 or 1 or how the six-CR heading is counted.
- Exponent range: two decimal digits with sign per the notes (10^-99…10^99); the listing shows overflow → E1 and underflow → 0 in exp and power (pp. 46, 167) but I did not find the bound as an explicit constant.

### (e) Go, Cancel, Stop, interrupts, errors

Read with confidence (comments quoted):

- **Stop** (G12, p. 79): indirect only; sets a "stopped" flag, types `Stopped by step N.` and returns control to the user.
- **Go** (G13, p. 80): direct only. "ERROR IF NO OPEN TASK" → E7. If the stopped flag is set: "RESET 'STOP' FLAG / JUMP TO ADVANCE" — resumes at the step **after** the Stop. Otherwise (after an error or interrupt): "FIND CURRENT STEP / JUMP IF FOUND, ELSE ADVANCE / SET P16 AND FETCH" — **re-executes the step that failed or was interrupted**; if that step has since been deleted it continues with the next one. If the trouble was in iteration (X7) it re-enters the ascend logic.
- **Cancel** (G4, p. 68): direct only; "ERASE ONE LEVEL OF STRUCTURE / JUMP IF MORE" — discards the whole suspended task. No message.
- **Do** while a task is suspended (G1, p. 62): "JUMP IF NO SUSPENDED TASK / CANCEL SUSPENDED TASK" — a direct Do silently cancels the old task. (The older listing instead refused with a message, see §3.) A handwritten margin note on p. 62 concerns a correction here and is only partly legible.
- **Delete all / all steps / all parts** also drop a suspended task (D1, p. 14).
- **Interrupt** (X10, p. 123): during a stored-program task types `I'm at step N.`; during a direct command types `Revoked.`; an interrupt during output is noted and honoured at the next line ("FAKE AN 'RI'"). The task stays suspended and Go resumes it.
- **Errors**: after the message all expression stacks are cleaned and control returns to the user; the control structure (the suspended task) is kept, except in the "It's a mess" case.
- **Done** (G3, p. 67) and **To** (G2, p. 66) are indirect only; **Form** (G11, p. 78) is direct only and "ERROR IF CONDITION ATTACHED"; **Demand** (G14, p. 81) "ERROR IF DIRECT", types the variable name and ` = ` and waits; the reply is evaluated as an expression, final period optional.
- Conditions on direct commands are allowed in this version (X4, p. 116 evaluates `if` for any command; the old "Direct commands must be unconditional" message is gone). Short Set is direct only (indirect gives E6).
- Input line handling (X2, p. 112): empty line → nothing; line ending with `*` → ignored; first non-space a digit → step label; a letter → direct command.

Uncertain: whether a leading `*` also kills the line (the notes say "either end"; I only confirmed the trailing test in the comments).

---

## 3. `listing-undated/JOSS_Listing.pdf` (55 pp.)

### What it is

An earlier assembly of the same program on ruled line-printer paper, no title page and
no date. Same region structure in one continuous run: A (pp. 1-3), E (3-7), F (7-12),
G (13-21), H-P data (21-24), U/V/W tables (24-27), X executive (27-34), `=` subroutines
(34-55). It is older than Jan-65: impersonal messages, verbs `Erase`/`Return`, no
Demand, functions `sfx`/`cf`/`amp`. Since Shaw was encoding "I need more storage
space." on 10-10-64, this listing is probably from before October 1964. That date is
my inference.

### Read with confidence

- **Verbs** (X4, p. 31): `Set Do To Done Cancel Erase Type Return Page Read Punch Form Stop Go Delete` — both `Erase` and `Delete`; `Return` where Jan-65 has `Line`; no `Demand`.
- **Functions** (=24, p. 42): `sqrt log exp sin cos ip fp sfx cf sgn amp max min` (`sfx`, `cf`, `amp` later became `xp`, `dp`, `arg`).
- **Words** (W, p. 27): `deck for form forms if in part parts step steps to size`, plus one or two entries I could not read (W0, W12; probably `all` and `values`).
- **Messages** (pp. 3-7), full list in `listing-undated/transcripts/messages.txt`:
  `Insufficient storage space.` · `Overflow.` · `Undefined value.` · `Attempting division by zero.` · `Index value must be integer and 0≤index≤99.` · `Numerals must be less than ten digits long.` · `Malformed.` · `Limit must be integer and |limit|<10*9.` · `Please "cancel" or "go" before another "do".` · `This command must not be given indirectly.` · `This command must not be given directly.` · `Deck number must be integer and 0<deck<10*9.` · `Form number must be integer and 0<form<10*9.` · `Step number must satisfy 1≤step<10*9.` · `Attempting sqrt of a negative number.` · `Direct commands must be unconditional.` · `The step called for doesn't exist.` · `The part called for doesn't exist.` · `The form called for doesn't exist.` · `Part number must be integer and 0<part<10*9.` · `Attempting log of a nonpositive number.` · `Attempting negative base to noninteger power.` · `Attempting sin or cos of magnitude ≥100.` · `Attempting zero to a negative power.`
  Framing: `Error in line above:  ` and `Error in step N:  `. Sign-on comment: `PLEASE TYPE YOUR INITIALS.`
- 24 error numbers (E0-E23) against 32 in Jan-65: no form-fitting, iteration or "Eh?" messages yet. "Malformed." is the ancestor of "Eh?".
- J2 "LINE NUMBER FOR EJECT" = 55 here too (p. 22).

### Uncertain

- The "another" message: the octal as I read it spells `anotther`. Either a typo in the original data or one misread digit.
- Five messages (E4, E12, E13, E17, E18, E19 in part) are given from the printer comment plus matching octal patterns rather than a full decode; case and `≤ <` are inferred from the identical Jan-65 words.
- I did not study this listing's Go/Cancel code; behaviour differences beyond the "Please cancel or go before another do" refusal are not established.

---

## 4. Lower-priority documents (skimmed via OCR)

### `implementation-1966/` P-3486 (Nov 1966)

About the PDP-6 system. JOSS-I facts mentioned: work began November 1960 (OCR garbled),
"partially operational in early 1963 and fully operational with eight consoles in January
1964", on JOHNNIAC's "4000 word memory"; PDP-6 JOSS in service February 1966.
**Uncertain**: the start date — the OCR reads "11965" and I did not check the page image.

### `typewriter-system-1962/` D-10507 (Sep 1962)

Hardware of the multiple-typewriter system: 16 line buffers of 81 characters; 7-bit
characters; output at 10 characters/second until an end-of-message code; stations come
on in the "red" state; control states red/green; an interrupt signal (RI) may be sent
during output; carriage return, tab and page are the terminating keys. Consistent with
the 81-character line and the RI handling seen in the listing. Not read beyond the grep.

### `rubrics-1967/` P-3560 (C. L. Baker, March 1967, 6 pp.)

**Not a message list.** It is a five-page bulleted list of design principles for
JOSS-like systems ("JOSS steps are quantized", "Interpretation is distinct from
execution", "Input to JOSS is typed in green, output in black", true results rounded
to nine digits, and so on). No messages are tabulated, so nothing was transcribed into
`rubrics-1967/transcripts/`. Its two example messages are JOSS-II style
(`Error at step 5.1: x = ???`).

### JOSS-II-only features (RM-5058 Jul 1966, RM-5220 Aug 1967 appendix A, P-3560)

Do **not** use any of these for JOSS-I:

- `Let` and formulas: `Let f(x)=…`, `Type formula f`, `Delete formula f`, `Type/Delete all formulas`, `Type f(3)`.
- Files: `Use file n (ident).`, `File … as item n (code).`, `Recall item n (code).`, `Discard item n (code).`, `Type item-list.`
- Conditional expressions `(x<0: 0; 0≤x<1: x*2; 1)`.
- Propositions as values: `true`/`false`, `not`, `tv(P)`, `conj`, `disj`, `Set x=P`.
- Iterative functions: `sum`, `prod`, `first`, and the range form `max[i=1(1)n: …]`, `min[…]`.
- `Do part n, k times.` / `Do step n, k times.`
- Parenthetical commands: `(Cancel.)`, `(Do part 1.)`.
- `Quit.`, `Reset timer.`, `Type timer.`, `Let S be sparse.`
- `Demand x as "text".`
- Combined commands: several objects in one `Type` or `Delete` (`Delete x, form 1, all parts.`).
- Messages: `x = ???`, `Error in formula f: …`, `Error at step 1 (in formula i): …`, `Please use a legitimate number`, single space after the colon in `Error at step 1.2: I can't…`.
- Much larger number range and storage.

Carried over unchanged from JOSS-I (seen in both): `Eh?`, `I have a negative argument for sqrt.`, `I have an overflow.`, `I can't express value in your form.`, `Don't give this command directly.`, `Illegal set of values for iteration.`, `I'm at step N.`, `Stopped by step N.`

**Uncertain**: the JOSS-II list comes from OCR of a dense appendix page (RM-5220 p. 43) and a keyword search of RM-5058; it is not exhaustive, and I did not find a single printed JOSS-II message table in these three documents.

---

## 5. Open questions worth settling against printed sessions

1. Exact column positions of `=` and the decimal point in default `Type` output (§2d).
2. Two spaces after the colon in `Error at step N:  …` — clear in the Jan-65 data and in Shaw's character count, but worth confirming on a printed page.
3. Page length (ejects at line count 55) and the layout of the time line.
4. Whether a leading `*` kills an input line.
5. "anotther" in the undated listing.
