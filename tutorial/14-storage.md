# 14. Storage

## Type size

Each user of JOSS had a small block of JOHNNIAC's store, and everything
the user typed that JOSS had to remember went into it: steps, forms and
values. `Type size.` says how much of it is in use.

```joss
Type size.
Set x = 1.
Type size.
1.1 Set t(i) = i*2.
Type size.
Form 1:
__ ____
Type size.
Do part 1 for i = 0(1)99.
Type size.
```
```text
Type size.
0
Set x = 1.
Type size.
3
1.1 Set t(i) = i*2.
Type size.
8
Form 1:
__ ____
Type size.
12
Do part 1 for i = 0(1)99.
Type size.
415
```

The unit is the JOHNNIAC word, of 40 bits. In this reconstruction a user
has 720 words. The figure is from Shaw's storage map of January 1965,
which leaves that many for the user out of a block of 1024; in August
1964 the whole block was 512 words, and Shaw wrote that a version was
"being prepared to double the present user's block" (P-2922, p. 31). How
many words each thing costs is this reconstruction's estimate, not
JOSS's own arithmetic: a step or a form costs two words and one more for
every five characters, a plain value three, and an element of an array
four.

Whatever the exact numbers, it was not much. Shaw put the limit on a
program at "about three typewritten pages" (P-3149, p. 12), and the
commonest requests from users were "for more scheduled JOSS time, for
more storage space, and for long-term storage of programs" (P-2922,
p. 31). An array of a hundred elements, as above, is more than half of
everything.

## Running out

When there is no room for what a step needs, JOSS says so and stops at
that step.

```joss
1.1 Set s(i) = i*2.
2.1 Set c(i) = i*3.
Do part 1 for i = 0(1)99.
Do part 2 for i = 0(1)99.
Type i.
Type size.
Delete s.
Type size.
Go.
Type c(99).
```
```text
1.1 Set s(i) = i*2.
2.1 Set c(i) = i*3.
Do part 1 for i = 0(1)99.
Do part 2 for i = 0(1)99.
I need more storage space.
Type i.
       i =      73
Type size.
717
Delete s.
Type size.
317
Go.
Type c(99).
   c(99) =  970299
```

The program is suspended, not lost. The squares in `s` were finished
with, so the user deleted them and said `Go.`, and the cubes were
completed from the place where they had stopped.

> Even when JOSS has run out of space in pursuing the task, it stands
> ready to help. The user may ask JOSS to delete portions of the program
> which are no longer essential to getting final answers (such as forms
> or steps no longer needed) and to continue with a Go command, this time
> with additional space for JOSS to work in. (P-2922, p. 27)

## What a Do costs

A program that is running takes room too. Each `Do` that has been started
and not finished has to be remembered: which step it had reached, and, if
it has a `for`, which value. A `Do` inside a `Do` inside a `Do` is three
of them.

Usually this is too little to notice. It matters when a part does
itself.

## Recursion

Nothing stops a step of part 1 from saying `Do part 1.` JOSS begins
part 1 again, and when that is finished it comes back and carries on from
the step after the `Do`.

JOSS has no arguments and no local values: every letter is the same
letter everywhere. A part that does itself must therefore put back
whatever it changes. This is the factorial of n, worked out as n times
the factorial of n-1:

```joss
1.1 Set f = 1 if n ≤ 1.
1.2 Done if n ≤ 1.
1.3 Set n = n-1.
1.4 Do part 1.
1.5 Set n = n+1.
1.6 Set f = f·n.

Set n = 10.
Do part 1.
Type n, f.
```
```text
1.1 Set f = 1 if n ≤ 1.
1.2 Done if n ≤ 1.
1.3 Set n = n-1.
1.4 Do part 1.
1.5 Set n = n+1.
1.6 Set f = f·n.

Set n = 10.
Do part 1.
Type n, f.
       n =      10
       f =       3.6288·10*6
```

Step 1.3 counts n down on the way in and step 1.5 counts it up again on
the way out, so that when step 1.6 is reached n is what it was. JOSS
remembers where each of the ten `Do`s had got to; the program has to
remember everything else.

The Towers of Hanoi are done the same way. To move n discs from peg a to
peg b, move n-1 to the spare peg c, move one disc, and move the n-1 from
c to b. The pegs are exchanged before each inner move and exchanged back
after it.

```joss
1.1 Done if n = 0.
1.2 Set n = n-1.
1.3 Do part 2.
1.4 Do part 1.
1.5 Do part 2.
1.6 Type a, b in form 1.
1.7 Do part 3.
1.8 Do part 1.
1.9 Do part 3.
1.95 Set n = n+1.

2.1 Set t = b.
2.2 Set b = c.
2.3 Set c = t.

3.1 Set t = a.
3.2 Set a = c.
3.3 Set c = t.

Form 1:
Move a disc from peg _ to peg _

Set a = 1.
Set b = 3.
Set c = 2.
Set n = 3.
Do part 1.
```
```text
1.1 Done if n = 0.
1.2 Set n = n-1.
1.3 Do part 2.
1.4 Do part 1.
1.5 Do part 2.
1.6 Type a, b in form 1.
1.7 Do part 3.
1.8 Do part 1.
1.9 Do part 3.
1.95 Set n = n+1.

2.1 Set t = b.
2.2 Set b = c.
2.3 Set c = t.

3.1 Set t = a.
3.2 Set a = c.
3.3 Set c = t.

Form 1:
Move a disc from peg _ to peg _

Set a = 1.
Set b = 3.
Set c = 2.
Set n = 3.
Do part 1.
Move a disc from peg 1 to peg 3
Move a disc from peg 1 to peg 2
Move a disc from peg 3 to peg 2
Move a disc from peg 1 to peg 3
Move a disc from peg 2 to peg 1
Move a disc from peg 2 to peg 3
Move a disc from peg 1 to peg 3
```

## Recursion without end

A part that does itself and never stops uses a little more room at every
turn, and comes to the end of it.

```joss
1.1 Set n = n+1.
1.2 Do part 1.
Set n = 0.
Do part 1.
Type n.
Type size.
Cancel.
Type size.
```
```text
1.1 Set n = n+1.
1.2 Do part 1.
Set n = 0.
Do part 1.
I need more storage space.
Type n.
       n =      59
Type size.
720
Cancel.
Type size.
12
```

The value of n says how deep it went. The suspended program is holding
nearly all the room there is, and `Cancel.` gives it back.

So recursion in JOSS is for shallow work. A loop that only goes round
should be written with `for` or with `To`, which cost nothing however
long they run:

```joss
1.1 Set n = n+1.
1.2 To step 1.1 if n < 5000.
Set n = 0.
Do part 1.
Type n.
Type size.
```
```text
1.1 Set n = n+1.
1.2 To step 1.1 if n < 5000.
Set n = 0.
Do part 1.
Type n.
       n =    5000
Type size.
15
```

## Nothing is kept

When the user switched the console off, everything went. JOSS on
JOHNNIAC had nowhere to keep a program from one session to the next:
Shaw lists "no provision for filing programs for use in a later session"
among its three serious limitations (P-3149, p. 11). A program that was
wanted again had to be typed again, and `Type all.` at least gave a clean
copy to type it from.

A session file does the same service here, and `joss file.joss` types it
in for you.

## The time and the company

Two more things can be asked of JOSS itself.

```joss
Type time.
Type users.
```
```text
Type time.
1000
Type users.
1
```

`Type time.` gives the time of day as four digits: 1000 is ten o'clock,
which is when JOSS service began each morning. `Type users.` gives the
number of consoles in use. When Shaw typed it on 28 April 1965 the answer
was 6 (P-3131, p. 4). Here there is one console and the clock is not
running, so the answers are always these.

Next: [Worked examples](15-worked-examples.md)
