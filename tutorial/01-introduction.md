# 1. Introduction

JOSS is the JOHNNIAC Open-Shop System. It was written by J. C. Shaw at the
RAND Corporation in Santa Monica, and it ran on JOHNNIAC, a computer that
RAND had built for itself in 1950-53. An early version was in limited use
from May 1963. From January 1964 the full system was in daily use: ten
typewriters around the building, up to eight of them served at once.

JOSS was not a way of getting at the computer. It was a way of getting
sums done. Shaw wrote that it was designed

> to give the individual scientist or engineer an easy, direct way of
> solving his small numerical problems without a large investment in
> learning to use an operating system, a compiler, and debugging tools, or
> in explaining his problems to a professional computer programmer and in
> checking the latter's results. (P-2922, p. 2)

The user sat at an electric typewriter and typed a line. JOSS answered on
the same sheet of paper, and then it was the user's turn again. The user's
lines came out in green and JOSS's in black. A line could be an order to
be carried out at once, or a step of a program to be kept for later, and
the two were written in the same language. When something went wrong JOSS
said so in a sentence, kept everything as it was, and waited. The user
put the mistake right and typed `Go.`

That is the whole idea. Shaw called JOSS a "computing aide", and asked
his users to think of it as one.

The machine was small even for its day: 4096 words of core store and a
drum of 12,288 words. Shaw listed what it lacked:

> JOHNNIAC remains a very primitive machine with no indexing, no indirect
> addressing, no floating point, no error checking, no memory protect, no
> interrupts, no channels, no compare, no zero test, a miserable format of
> two single-address instructions per word, and a 50-μs add time.
> (P-2922, p. 6)

It did not matter. By May 1965 there were nearly 200 users, most of whom
had never programmed before, and one of them wrote:

> JOSS is becoming essential to our output like paper, or coffee. [...] We
> can hardly live with JOSS, but we can't live without it. We're hooked.
> (P-3149, p. 14)

## Which JOSS this is

There were two languages called JOSS. This tutorial is about the first,
the one on JOHNNIAC, which ran from 1963 until a new JOSS on a PDP-6
computer took over in February 1966. The first is now called JOSS-I and
the second JOSS-II. JOSS-II was a larger language, and most of what has
been written about JOSS describes that one. Nothing of JOSS-II is used here.
[Chapter 16](16-1964-and-1965.md) lists what the later language added, so
that you can tell which one a description is talking about.

The interpreter used in this tutorial was written sixty years later from
RAND's own papers: Shaw's two papers of 1964 and 1965, his page of
examples of April 1965, C. L. Baker's scenario of the JOSS film of March
1964, and the assembly listing of JOSS dated 9 January 1965, from which
the wording of the messages was taken. The pages of typewriter output
printed in those papers are replayed by the test suite, and what this
JOSS types is compared with them column for column.

The papers are referred to by their RAND numbers:

| | |
|---|---|
| P-2922 | J. C. Shaw, "JOSS: A Designer's View of an Experimental On-Line Computing System", August 1964 |
| RM-4162-PR | C. L. Baker, "JOSS: Scenario of a Filmed Report", June 1964 |
| P-3131 | J. C. Shaw, "JOSS: Examples of the Use of an Experimental On-Line Computing Service", April 1965 |
| P-3149 | J. C. Shaw, "JOSS: Experience with an Experimental Computing Service for Users at Remote Typewriter Consoles", May 1965 |

## Running JOSS

Build the interpreter once:

```bash
cargo build --release
```

With no file name, `joss` is a console. Type a line and press Enter, and
JOSS answers:

```bash
target/release/joss
```

A session can also be kept in a file, one typed line to a line. This is
`examples/hello.joss`:

```joss
* The first program. JOSS types what stands between the quotes.
Type "Hello, world".
```
```text
* The first program. JOSS types what stands between the quotes.
Type "Hello, world".
Hello, world
```

Give `joss` the file name and it replays the lines and prints the page as
it would have looked: the user's lines and JOSS's answers, in the order
they were typed.

```bash
target/release/joss examples/hello.joss
```

Three options change what is printed:

```bash
target/release/joss --marked examples/hello.joss   # U: and J: in front of each line
target/release/joss --quiet examples/hello.joss    # only what JOSS typed
target/release/joss --check examples/hello.joss    # read the file without running it
```

`--check` reports lines that are not JOSS, and a few other things that can
be seen without running anything. Its remarks are an editor's, not JOSS's.

## How to read this tutorial

Every session is shown as two blocks. The first is what the user types: it
is a session file, and you can save it and run it. The second is the page
that `joss` prints for it.

```joss
Type 2+2.
Type 2+3.
```
```text
Type 2+2.
     2+2 =       4
Type 2+3.
     2+3 =       5
```

The lines of the second block that are not in the first are the ones JOSS
typed. On the paper they would have been in black, and the user's in
green.

Every pair of blocks like this in the tutorial is run by the test suite
(`tests/tutorial.rs`), and the second block is what the interpreter
really printed. A block with no page under it is a fragment, shown to
make a point, and is not run.

## The typewriter's characters

The typewriters were IBM model 868s with a keyboard chosen for JOSS:
"brackets and the symbols for arithmetic operations and numerical
relations replaced the less frequently used special characters" (P-3149,
p. 3). Four of those characters are not on a modern keyboard. Each has a
stand-in that `joss` accepts:

| JOSS | meaning | you can type |
|------|---------|--------------|
| `·`  | times | `@` |
| `≠`  | is not equal to | `!=` |
| `≤`  | is at most | `<=` |
| `≥`  | is at least | `>=` |

So `Type 6@7.` is `Type 6·7.` JOSS itself always types the real character.
This tutorial uses the real characters throughout.

One character must be learnt at once, because everyone gets it wrong the
first time:

> **`*` is not times.** It is "to the power of". `2*3` is 8. Times is the
> centred dot: `2·3` is 6.

Next: [A first conversation](02-a-first-conversation.md)
