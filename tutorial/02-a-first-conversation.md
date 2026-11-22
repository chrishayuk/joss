# 2. A first conversation

## Type

This is the first thing typed in the JOSS film of 1964. Shaw's figures of
1964 and his pages of examples of 1965 begin with it too:

```joss
Type 2+2.
```
```text
Type 2+2.
     2+2 =       4
```

The user asked JOSS to type the value of 2+2, and JOSS did. It labelled
the answer with the expression it was asked for, so that the page can be
read afterwards without looking back.

`Type` is a verb. An instruction to JOSS is a sentence in the imperative:
it begins with a verb, which begins with a capital letter, and it ends
with a full stop. JOSS is strict about both.

```joss
type 2+2.
Type 2+2
Type 2+2.
```
```text
type 2+2.
Eh?
Type 2+2
Eh?
Type 2+2.
     2+2 =       4
```

## Eh?

`Eh?` is what JOSS says to any line it cannot read. It does not say what
is wrong with the line or where. Shaw thought that was the honest thing to
do:

> All these errors are reported by the very brief message "Eh?". Thus, the
> user is forced to read his step to find the error, rather than possibly
> being misled by a message unrelated to the actual error. (P-2922, p. 27)

Nothing is lost by a mistake. The line did nothing; type it again.

## Several things at once

`Type` takes a list, with commas between the items. Each value goes on a
line of its own.

```joss
Type 7+2, 7-2, 7/2.
```
```text
Type 7+2, 7-2, 7/2.
     7+2 =       9
     7-2 =       5
     7/2 =       3.5
```

JOSS lines up the equals signs and the decimal points. An underscore in
the list is a blank line:

```joss
Type 1+1, _, 2+2.
```
```text
Type 1+1, _, 2+2.
     1+1 =       2

     2+2 =       4
```

If any one of the values cannot be worked out, none of them is typed:

```joss
Type 1+1, 1/0, 2+2.
```
```text
Type 1+1, 1/0, 2+2.
Error above:  I have a zero divisor.
```

"Error above" means that the mistake is in the line just typed. JOSS's
other messages are the subject of [chapter 12](12-mistakes-and-go.md).

## Words

Words between double quotes are typed as they stand.

```joss
Type "The answer is on the next line.".
Type 6·7.
```
```text
Type "The answer is on the next line.".
The answer is on the next line.
Type 6·7.
     6·7 =      42
```

The full stop after the closing quote ends the instruction. The one before
it is part of the text. `Type` takes one piece of text or a list of
values, not a mixture: to put words and numbers on the same line you need
a form ([chapter 9](09-forms.md)).

## Spaces

Spaces may be put between the parts of an instruction or left out, as you
please. The label JOSS gives an answer is the expression exactly as you
typed it, spaces and all.

```joss
Type 2+2.
Type 2 + 2.
Type   2+   2  .
```
```text
Type 2+2.
     2+2 =       4
Type 2 + 2.
   2 + 2 =       4
Type   2+   2  .
  2+   2 =       4
```

A space cannot go inside a word or a number, and a word must stand clear
of what follows it:

```joss
Ty pe 2+2.
Type2+2.
```
```text
Ty pe 2+2.
Eh?
Type2+2.
Eh?
```

## Notes, and lines to be ignored

A line with an asterisk at the beginning or at the end is ignored. JOSS
says nothing and does nothing.

```joss
* The area of a circle of radius 3:
Type 3.14159265·3*2.
Type 3.14159265·3·2. *
```
```text
* The area of a circle of radius 3:
Type 3.14159265·3*2.
3.14159265·3*2 =  28.2743339
Type 3.14159265·3·2. *
```

An asterisk at the beginning makes a note. An asterisk at the end is how a
line was cancelled: on a typewriter there is no way to take back what has
been typed, so the user who saw a mistake half way along a line typed `*`
and pressed the carriage return. Shaw does it on page 6 of P-3131:

```joss
Do part 12 for *
*  An asterisk at the end can kill the line... I forgot something:
```
```text
Do part 12 for *
*  An asterisk at the end can kill the line... I forgot something:
```

The other repair was the strike-out sign, `#`. The typewriter could
backspace, and a character struck over another replaced it; `#` struck
over a character replaced it with a space. In a file there is no
backspacing, so here `#` is simply a space:

```joss
Type#2+2.
```
```text
Type#2+2.
     2+2 =       4
```

A line of nothing at all is also allowed, and the pages in the papers use
it freely to set things apart.

## Direct and indirect

Everything in this chapter was a *direct* instruction: JOSS did it as soon
as the line was finished. The other kind of line begins with a number.
JOSS keeps such a line and does nothing until it is told to:

```joss
1.1 Type 2+2.
Do step 1.1.
```
```text
1.1 Type 2+2.
Do step 1.1.
     2+2 =       4
```

That is a stored program of one step. Stored programs begin in
[chapter 6](06-steps-and-parts.md). Until then everything is direct, and
JOSS is, as the film put it, "a printing desk calculator" (RM-4162-PR,
p. 2).

Next: [Arithmetic](03-arithmetic.md)
