# Independent review

## Who reviewed, and what was checked

The repository was reviewed by OpenAI Codex CLI (codex-cli 0.166.0) on 22
November 2026. It is an independent second model, not a person. It checked
the code against the sources, ran everything, and looked for claims that go
beyond the evidence. It changed no code.

It ran the build, the tests, the transcript checker, the WebAssembly build
and tests, `cargo fmt`, `cargo clippy` and the WASI comparison, all of which
passed. The VS Code extension's dependencies could not be installed in its
sandbox (no network), so it could not run the extension's tests; they have
since been run here and pass.

It checked twenty places in the transcriptions against the scans, character
by character (RM-4162 pp. 11, 13, 16 and 32; P-2922 pp. 17 and 21; P-3149
p. 8; P-3131 p. 6; Shaw's notes pp. 2 and 13; the undated listing pp. 3 and
5), and it read the January 1965 listing's control flow for the power
routine, `arg`, `Form`, `Read` and `Punch`.

Each finding below was checked again here against the code, by running it,
and against the scans, before anything was changed. Page numbers are PDF
pages.

## Spot-check counts

Of the twenty places the reviewer checked:

- 13 agree with the scan exactly;
- 6 differ from the scan in a way the repository had already marked or
  noted (the comma, the multiplication dot, the space after `cos`, the
  restored decimal point, `Functiona:` and `numeriaal`);
- 1 (`Type 2*8.` on RM-4162 p. 11) was reported as an unmarked change. It
  is not: see finding 7.

So no transcription error was found in those twenty places. The README's
description of Shaw's note on p. 13 was wrong, though the transcription of
it was right (finding 9).

## The findings, and what was done

### 1. Large integral powers return zero or crash: accepted, fixed

Confirmed: `(10*99)*(10*17)` typed 0 in a release build and panicked in a
debug build (the exponent product overflowed an `i64`);
`(10*(-99))*(10*17)` reported an overflow; `(-1)*(10*18)` reported a
negative base to a fractional power, because a whole number too large for
an `i64` was taken to be fractional.

Fixed in 6d13833 by rewriting `Num::pow` to follow routine =35 of the
listing, which tests the fraction part separately and handles a power of
ten by its exponent. Tests: `powers_follow_the_listing` in `src/num.rs`,
and `tests/sessions/listing-arithmetic.txt`.

### 2. Whole powers use a different algorithm from the original: accepted, fixed

Confirmed on pp. 166-167: cards =035E816-852 do a whole power below 30 by
repeated squaring, each product rounded, and a negative power starts from
a rounded 1/a. The reconstruction multiplied a times itself n-1 times.

Fixed in 6d13833. `1.23456789*13` now types `15.4772825`, the reviewer's
hand trace of the listing; correctly rounded it is 15.4772823. Test:
`powers_follow_the_listing`.

### 3. `Form` rejects expressions the original evaluates: accepted, fixed

Confirmed on p. 78: card G011E020 calls the expression evaluator, "EVALUATE
FORM NUMBER", then G011E030 "VALIDATE". Fixed in 3588de6: `Form n:` and
`Form 1+1:` are evaluated and checked; a bad value gives "Form number must
be integer and 0<form<10*9."; an undefined letter gives the usual message.
Tests: the parser test in `src/parse.rs` and
`tests/sessions/listing-arithmetic.txt`.

### 4. The listing resolves `arg(0,0)`: accepted, fixed

Confirmed on pp. 56-57: card F010E110 tests y = 0 first and sets the angle
to zero; the quadrant step leaves a zero angle alone when the sign of x is
not negative, and a packed zero has no sign. Fixed in 6d13833:
`arg(0,0)` is 0, and the supplied message "I have arguments of zero for
arg." is withdrawn from the code, the tutorial and the README. Test:
`arg_of_the_origin_is_zero`.

### 5. "Read and Punch … added late in 1964" is contradicted: accepted

Confirmed: routines G9 and G10 (pp. 76-77) are each one card that jumps
to "Eh?", and P-3149 (p. 13; printed p. 12) says a punched card file was
rejected. The names are in the verb table and were written by hand into
Shaw's summary of the language, and the executive has timing constants
for reading and punching, so the verbs were planned; they did not work in
January 1965. The README and chapter 16 now say so (b63068d).

### 6. The numerical differences were attributed to original bugs too certainly: accepted, and resolved

The review was right that the pages alone did not show what JOSS held
inside, and that `exp(72)` correctly rounds straight to the printed eight
digits. Rather than only narrowing the claims, the listing's own routines
were followed: F1 (log, pp. 44-45), F2 (exp, pp. 46-47) and =35 (powers),
in `src/routines.rs` and `Num::pow` (6d13833). The results:

- F1 gives log(2) = .693147180, typed `.69314718` as printed. F1 works the
  log out in binary to about twelve places; for a log below 1 it takes the
  nine digits with `M N 9` (a product kept whole, not rounded), cutting the
  rest off (cards F001E660-E670). For a log of 1 or more it rounds
  (`MR`, card F001E450).
- F2 gives e^72 = 1.85867174·10*31, typed `1.8586717` in the film's table.
  It works from x·log10(e) with an eleven-digit constant (.43429448190).
- With =35, F1 and F2 together, the Chart A expression ends `…357` as
  printed.
- F2 also gives `exp(100)` = 2.68811714·10*43, as P-2922 fig. 3f prints.

All 767 lines of the five transcripts now match, and the `~:` lines that
recorded the three differences are gone. The handwritten note "Check it"
was therefore not about the printed log 2: see 9.

How much this shows: it is the reconstruction's reading of the routines,
not a run of the original program. The routines were transcribed from the
page images, run in a small instruction-level emulator written for this
check (not kept in the repository), and then written in Rust; the Rust
agrees with the emulator on 2,920 random arguments. Both rest on the same
reading. What the listing does not settle is assumed and listed at the
head of `src/routines.rs`: that a product leaves a positive low half in MQ
(which card F001E540 needs), that a quotient is cut off towards zero, and
what the tables N, I, J11 and K21 hold (their names and the region listing
say). The result for log 2 does not depend on these: the computed value is
.69314718056 to about eleven places, and cutting it off at nine digits
gives .693147180 whichever way the last bits fall.

### 7. An unmarked period after `Type 2*8`: disputed

RM-4162 p. 11 prints `Type 2*8.` with the full stop. It is there in the
reduced scan `docs/scans/rm-4162-1964/page-11.jpg`, in the 200 dpi page
image, and in a 400 dpi render of the PDF made for this check. Nothing was
changed. The review's wider point, that the README's count of changes to
the film's text was incomplete, was right for another reason: the form's
restored decimal point (example 5, p. 32) was marked in its source
transcript but not in the combined one. It is now marked there as change
(5).

### 8. The correction claims need classifying: accepted

Every editorial change is now listed in the README with its kind (slip,
conflict, interpretation, ours), and the film transcript labels each of its
five changes where it is made (b63068d). The classifications follow the
review's table: the comma and the dot are conflicts; `Delete all.`, the
space after `cos` and the restored point are interpretations, and the
`Delete all.` note now says that this exact line is not on any page; the
Chart A sign is a conflict; `Functiona:` and `numeriaal` are slips; the
undated listing's `OVERLFOW` and `anotther` are slips that are kept as
they are. The three numerical cases are no longer corrections at all (6).

### 9. Other overstatements: accepted

- "686 … column for column": the README now says exactly what is compared
  with what (edited transcripts; trailing spaces ignored; retyped lines
  with spaces left out; page throws and operators' messages not compared)
  and that this is a regression test against the transcripts, not an
  independent check of the scans. The count is now 689 of 767.
- The 1964 fixed-point cutoff: labelled an assumption in `src/format.rs`
  and the README. The pages show `.2` plain and `.02` with an exponent and
  nothing between.
- Shaw's note: it does not end "Check it." Checked against the scan
  (`docs/scans/program-notes/page-13.jpg`): it reads "Check it. Looks like
  coefficient between 1 and sqrt(2) gets doubled several times." The
  README quoted only the first two words. Following F1 shows what the note
  is about: cards F001E080-E120 double the coefficient until it is above
  (√2/2)·2^3·10^8, and a coefficient from 1 to √2 (times 10^8) is doubled
  three times. The range reduction works; the cut-off ninth digit is a
  different part of the routine, and the note does not mention it.
- "One message in a hundred": removed from chapter 12.
- "The first conversational computing language": `Cargo.toml` now says
  "an early conversational computing language".

### 10. Transcription spot check

See the counts above.

### 11. Build and test results

Reproduced here, after the changes:

| Command | Result |
|---|---|
| `cargo build --release` | passes |
| `cargo test --release` and `cargo test` | pass |
| `cargo test -p joss-lsp` | 18 passed |
| `target/release/joss --verify transcripts/*.txt` | 767 of 767 lines match, 689 column for column; no differences |
| `node web/build.mjs && node web/test.mjs` | 55 examples, 5 original sessions, 55 files through the language server, 0 failed |
| `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo test --target wasm32-wasip1`, `scripts/wasi-check.sh` | pass; 56 compared, 0 differ |
| `editors/vscode`: `npm install`, `npm run build`, `npm test`, `npm run test:vscode` | 15 passed; the VS Code run passes |

## What is still open

- `sin`, `cos` and `arg` still use the Rust standard library, rounded to
  nine digits. Their routines (F3, F4, F10) are in the listing and have not
  been followed; no page shows a value they would change.
- The JOHNNIAC details assumed in `src/routines.rs` are assumptions.
- No person has checked the transcriptions against the scans.
