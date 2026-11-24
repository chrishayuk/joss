# JOSS examples

Each `name.joss` is a session: the lines a user types at the console, one
to a line. Each `name.out` is the page that session produces, the user's
lines and JOSS's answers together, exactly as

```bash
target/release/joss examples/name.joss
```

prints it. Where a program asks for numbers with `Demand`, the answers
are the lines that follow the `Do` in the session file. A line beginning
with `!` is not typed: it is a key pressed or a setting
(`! interrupt after 4 lines`, `! joss 1964`).

`cargo test` replays every session and compares it with its `.out` file
(`tests/examples.rs`).

## From the papers of 1964 and 1965

The user's lines are as they stand in `transcripts/`, which were typed in
from the scans of the original pages. The first line of each file, a
note, says which paper and which page. The sessions of 1964 begin with
`! joss 1964` so that JOSS answers in the words and columns of that year.

| Session | From | What it is |
|---|---|---|
| `1964-film-calculator` | RM-4162-PR, the film of March 1964 | JOSS as a desk calculator: arithmetic, functions, the first error messages |
| `1964-film-table` | RM-4162-PR, examples 1 and 2 | A table of roots, logarithms and exponentials, interrupted three times and improved each time |
| `1964-film-rlc-circuit` | RM-4162-PR, example 3 | The damped current in a circuit with resistance, inductance and capacitance |
| `1964-film-brightness` | RM-4162-PR, examples 4 and 5 | A table of differences in brightness for a list of angles and distances, three parts deep |
| `1964-elements` | P-2922, figures 3a to 3c | The elements of the language, a line at a time |
| `1964-hypotenuse` | P-2922, figures 3d and 3e | A stored program, listed, changed and deleted |
| `1964-step-and-form` | P-2922, figures 3f and 3g | One step done for a list of values, without a form and with one |
| `1964-demand` | P-2922, figure 3h | `Demand` fills a vector, which is then scaled to add up to 1 |
| `1964-normal-integral` | P-2922, figure 3i | The normal integral by Gauss's two-point rule |
| `1965-elements` | P-3131, page 2 | The elements of the language again, in the JOSS of 1965 |
| `1965-hypotenuse` | P-3131, page 3 | The hypotenuse, then changed in two lines to find Pythagorean triples |
| `1965-gauss-integration` | P-3131, page 4 | The integral of 1/x by Gauss's two-point rule, against `log` |
| `1965-root-finding` | P-3131, page 5 | Two roots of exp(x) = 20·log(x) by the secant rule, with a `Go` on the way |
| `1965-matrix-inversion` | P-3131, pages 6 and 7 | A 4 by 4 matrix inverted in place, and inverted back |
| `1965-atom` | P-3131, page 7 | C. L. Baker's one-step solution of the ATOM puzzle |
| `1965-table` | P-3131, pages 8 and 9 | A formatted table of elementary functions with a heading |
| `1965-primes` | P-3149, figure 1 | The prime numbers up to 50 in three steps |
| `1965-quadratic-with-mistakes` | P-3149, figure 2 | A quadratic equation and thirteen messages: every mistake put right with `Go` |

## Written for this reconstruction

Idiomatic JOSS-I: nothing in them is later than January 1965.

| Session | What it shows |
|---|---|
| `hello` | `Type "..."` |
| `desk-calculator` | Direct arithmetic: the five operations, brackets and bars |
| `tenths` | Decimal arithmetic: .1 added ten times is exactly 1 |
| `compound-interest` | A power in a form, and a range with a step of 5 |
| `factorial` | A running product, and what happens at the tenth digit |
| `factorial-recursive` | A part that does itself |
| `fibonacci` | Three letters and a form |
| `gcd` | Euclid's algorithm: a loop made with `To` |
| `prime-factors` | Trial division with `To` in both directions |
| `primes-sieve` | The sieve of Eratosthenes on an indexed letter |
| `perfect-numbers` | Two loops and a sum of divisors |
| `collatz` | The 3n+1 sequence: `To`, `Done` and a count |
| `digits` | Taking a number apart with `ip`, and with `dp` and `xp` |
| `binary` | A number in binary, digits kept in an indexed letter |
| `leap-years` | `and` and `or`, taken from left to right |
| `day-of-week` | Zeller's rule, with capital and small letters as different names |
| `easter` | The date of Easter; a part used as a remainder function |
| `newton-sqrt` | Newton's method for a square root |
| `bisection` | A root by halving the interval, with `sgn` |
| `simpson` | Simpson's rule; the same part done three times with different weights |
| `pi-series` | Pi by Machin's formula |
| `e-series` | e as a sum of reciprocal factorials |
| `horner` | A polynomial with its coefficients in an indexed letter |
| `tpk` | Knuth and Trabb Pardo's TPK (1976): `Demand`, the formula written out, and TOO LARGE in a form |
| `quadratic-roots` | Real or complex roots: `To part` chooses the ending |
| `pascal-row` | A row of Pascal's triangle |
| `multiplication-table` | A form with nine fields |
| `temperature-table` | A table with a heading typed by `Type form` |
| `loan` | Paying off a loan month by month |
| `line-numbers` | `$`, the number of the line the typewriter is on |
| `mean-and-deviation` | `Demand`: JOSS asks for the data |
| `bubble-sort` | Sorting an indexed letter |
| `hanoi` | The Towers of Hanoi by recursion, exchanging letters on the way in and out |
| `done` | `Done` leaves a part early |
| `stop-and-go` | `Stop` as a breakpoint: look, delete the step, `Go` |
| `error-and-go` | A wrong value found by JOSS and put right without starting again |
| `interrupt-and-go` | The interrupt key: change the form in the middle of a table and `Go` |
| `storage` | Recursion without end, "I need more storage space." and `Cancel` |
