# 12. Mistakes and Go

Everything so far could be said of other languages. This chapter is about
what could not, in 1964: what happens when something goes wrong.

On most systems of the day a program that failed was finished. The user
got a dump or a diagnostic, went away, mended the cards and tried again
from the beginning. JOSS stops where the trouble is, says what it is,
keeps everything exactly as it was, and waits to be told what to do.

> If the user specifies a process imperfectly, JOSS will call it to his
> attention with an error message and stand by to help with the changes.
> Frequently the user can direct JOSS to make the correction and continue
> without having to start over at the beginning of the program.
> (P-3149, p. 5)

## Three ways of saying it

**Eh?** The line just typed is not JOSS. It has no other meaning and no
other detail ([chapter 2](02-a-first-conversation.md)).

**Error above:** The line just typed is JOSS, but it cannot be done. A
message follows that says why.

**Error at step n:** A step of the stored program cannot be done. JOSS has
stopped at that step.

```joss
Type 2+.
Type 2/0.
1.1 Type 2/0.
Do part 1.
```
```text
Type 2+.
Eh?
Type 2/0.
Error above:  I have a zero divisor.
1.1 Type 2/0.
Do part 1.
Error at step 1.1:  I have a zero divisor.
```

There are about thirty messages that can follow "Error", and
[chapter 17](17-reference.md) lists them all. They are sentences. In
1964 they were impersonal ("Zero divisor."). By 1965 JOSS spoke for
itself:

> After a year's experience with neutral, impersonal messages, we
> introduced personal pronouns and some informal diction (Fig. 2). This
> was done not facetiously, but rather to reinforce the model. No
> objections to the more personal messages have been voiced.
> (P-3149, p. 6)

## Nothing is half done

When a step fails, it has not happened. JOSS works out the whole step
before it changes anything, so an error leaves every value, step and form
as it was before the step began. Shaw called this the hard part of
writing JOSS:

> It was essential that no irreversible change be made in the user's
> block until the interpretation of a step could guarantee that the
> execution would proceed to completion without an error. (P-2922, p. 30)

That is why the next thing works.

## Go

After "Error at step n", the program is not over. It is waiting at step n.
`Go.` tells JOSS to try that step again and carry on.

In between, the user puts right whatever was wrong. Here it is a value:

```joss
1.1 Set s = s + sqrt[x(i)].
1.2 Type i, x(i), s in form 1.
Form 1:
__   x = ___   sum of roots so far ___.____

Set x(1) = 4.
Set x(2) = 9.
Set x(3) = -16.
Set x(4) = 25.
Set s = 0.
Do part 1 for i = 1(1)4.
Type i, x(i).
Set x(3) = 16.
Go.
```
```text
1.1 Set s = s + sqrt[x(i)].
1.2 Type i, x(i), s in form 1.
Form 1:
__   x = ___   sum of roots so far ___.____

Set x(1) = 4.
Set x(2) = 9.
Set x(3) = -16.
Set x(4) = 25.
Set s = 0.
Do part 1 for i = 1(1)4.
 1   x =   4   sum of roots so far   2.0000
 2   x =   9   sum of roots so far   5.0000
Error at step 1.1:  I have a negative argument for sqrt.
Type i, x(i).
       i =       3
    x(i) =     -16
Set x(3) = 16.
Go.
 3   x =  16   sum of roots so far   9.0000
 4   x =  25   sum of roots so far  14.0000
```

The table was two lines long when JOSS met the square root of a negative
number. The user asked where it had got to, saw the mistyped value,
corrected it and said `Go.` The table went on from its third line. The
first two were not typed again and the sum was not started again.

Here it is a step that is wrong:

```joss
1.1 Set y = 100/(x-3).
1.2 Type x, y in form 1.
Form 1:
x = _   y = ____.__
Do part 1 for x = 1(1)5.
1.1 Set y = 100/(x-3.5).
Go.
```
```text
1.1 Set y = 100/(x-3).
1.2 Type x, y in form 1.
Form 1:
x = _   y = ____.__
Do part 1 for x = 1(1)5.
x = 1   y =  -50.00
x = 2   y = -100.00
Error at step 1.1:  I have a zero divisor.
1.1 Set y = 100/(x-3.5).
Go.
x = 3   y = -200.00
x = 4   y =  200.00
x = 5   y =   66.67
```

And here it is something missing altogether. The program was started
before its form had been written:

```joss
1.1 Type n, n*2 in form 7.
Do part 1 for n = 1(1)3.
Form 7:
The square of _ is _.
Go.
```
```text
1.1 Type n, n*2 in form 7.
Do part 1 for n = 1(1)3.
Error at step 1.1:  I can't find the required form.
Form 7:
The square of _ is _.
Go.
The square of 1 is 1.
The square of 2 is 4.
The square of 3 is 9.
```

This is the way JOSS was meant to be used. There is no need to have
everything ready before starting, because JOSS will stop and ask for
whatever turns out to be missing.

## While JOSS waits

A program that has stopped for an error, a `Stop` or the interrupt key is
*suspended*. While it is, the user may give any direct instruction and
type any step or form. Only three things end the wait:

- `Go.` takes the program up where it stopped.
- `Cancel.` gives it up.
- A new `Do`, given directly, gives it up and starts something else.

Only one program can be waiting. If there is none, `Go.` has nothing to
go on with:

```joss
1.1 Type 1/x.
Set x = 0.
Do part 1.
Cancel.
Go.
```
```text
1.1 Type 1/x.
Set x = 0.
Do part 1.
Error at step 1.1:  I have a zero divisor.
Cancel.
Go.
Error above:  You haven't told me to do anything yet.
```

Starting again from the beginning needs no special instruction: type the
`Do` again.

```joss
1.1 Type x, 1/x.
Do part 1 for x = 2, 1, 0, -1.
Do part 1 for x = 2, 1, .5.
```
```text
1.1 Type x, 1/x.
Do part 1 for x = 2, 1, 0, -1.
       x =       2
     1/x =        .5
       x =       1
     1/x =       1
Error at step 1.1:  I have a zero divisor.
Do part 1 for x = 2, 1, .5.
       x =       2
     1/x =        .5
       x =       1
     1/x =       1
       x =        .5
     1/x =       2
```

## The interrupt key

The console had a key that broke into whatever JOSS was doing. JOSS
finished the line it was typing, said which step it had reached, and
handed the typewriter back with the program suspended.

A session file cannot press a key, so it has a line for it. A line that
begins with `!` is not typed at the console:

    ! interrupt after 4 lines

means that the interrupt key is pressed once JOSS has typed four lines of
its answer to the next line.

```joss
1.1 Type x, x*3 in form 1.
Form 1:
__ ______
! interrupt after 4 lines
Do part 1 for x = 1(1)12.
Form 1:
The cube of __ is ____.
Go.
```
```text
1.1 Type x, x*3 in form 1.
Form 1:
__ ______
Do part 1 for x = 1(1)12.
 1      1
 2      8
 3     27
 4     64
I'm at step 1.1.
Form 1:
The cube of __ is ____.
Go.
The cube of  5 is  125.
The cube of  6 is  216.
The cube of  7 is  343.
The cube of  8 is  512.
The cube of  9 is  729.
The cube of 10 is 1000.
The cube of 11 is 1331.
The cube of 12 is 1728.
```

The user saw four lines of a table, did not like the look of it, pressed
the key, wrote a better form and said `Go.` The table went on from 5.

The key is also the way out of a program that will not stop of its own
accord. (At the console of `joss`, Ctrl-C ends the session; to try the
interrupt key, use a session file or `joss --patience N`, which presses
it after N steps.)

In the film of 1964 the same thing is done to a table of a hundred lines:
it is interrupted three times, to change the layout, to add a heading,
and to space the lines in fives (`examples/1964-film-table.joss`).

## Shaw's nine mistakes

Figure 2 of P-3149 is a page typed at a console to show what JOSS says.
Shaw sets out to solve a quadratic equation and gets nearly everything
wrong on the way. Here is the whole page (it is
`examples/1965-quadratic-with-mistakes.joss`).

```joss
1.1 Set x(1) = [-b+sqrt(b*2-4·a·c)] / [2·a].
1.2 Set x(2) = [-b-sqrt(b*2-4·a·c)] / [2·a].
1.3 Type x(1), x(2) in form 1.
Set b = -1.
Set c = -6.
Do part1.1.
Do part 1.1.
Do part 1.
Set a = 0.
Go.
Set a = -1.
Go.
Set a = 1.
Go.
Form 1:
a =___   b =___   c =___   roots:    .____    .____
Go.
1.3 Type a, b, c, x(1), x(2) in form 1.
Go.
Form 1:
a =___   b =___   c =___   roots: ___.________.____
Go.
Form 1:
a =___   b =___   c =___   roots: ___.____ ___.____
Go.
Do part 1 for c = -1(1)-4.
Do part 1 for c = -1(-1)-4.
2.1 Do step 2.1 for i = 1(1)100.
Do step 2.1.
Delete step 2.1.
Go.
Go.
Do part 1 for c = -5.
Delete all.
```
```text
1.1 Set x(1) = [-b+sqrt(b*2-4·a·c)] / [2·a].
1.2 Set x(2) = [-b-sqrt(b*2-4·a·c)] / [2·a].
1.3 Type x(1), x(2) in form 1.
Set b = -1.
Set c = -6.
Do part1.1.
Eh?
Do part 1.1.
Error above:  Part number must be integer and 0<part<10*9.
Do part 1.
Error at step 1.1:  a is undefined.
Set a = 0.
Go.
Error at step 1.1:  I have a zero divisor.
Set a = -1.
Go.
Error at step 1.1:  I have a negative argument for sqrt.
Set a = 1.
Go.
Error at step 1.3:  I can't find the required form.
Form 1:
a =___   b =___   c =___   roots:    .____    .____
Go.
Error at step 1.3:  I have too few values for the form.
1.3 Type a, b, c, x(1), x(2) in form 1.
Go.
Error at step 1.3:  I can't express value in your form.
Form 1:
a =___   b =___   c =___   roots: ___.________.____
Go.
Error at step 1.3:  Your fields run together in the form.
Form 1:
a =___   b =___   c =___   roots: ___.____ ___.____
Go.
a =  1   b = -1   c = -6   roots:   3.0000  -2.0000
Do part 1 for c = -1(1)-4.
Error above:  Illegal set of values for iteration.
Do part 1 for c = -1(-1)-4.
a =  1   b = -1   c = -1   roots:   1.6180   -.6180
a =  1   b = -1   c = -2   roots:   2.0000  -1.0000
a =  1   b = -1   c = -3   roots:   2.3028  -1.3028
a =  1   b = -1   c = -4   roots:   2.5616  -1.5616
2.1 Do step 2.1 for i = 1(1)100.
Do step 2.1.
I need more storage space.
Delete step 2.1.
Go.
It's a mess.  Let's start over.
Go.
Error above:  You haven't told me to do anything yet.
Do part 1 for c = -5.
a =  1   b = -1   c = -5   roots:   2.7913  -1.7913
Delete all.
```

Line by line:

1. `Do part1.1.` A word must stand clear of what follows it. **Eh?**
2. `Do part 1.1.` That is JOSS, but there are no parts with fractions:
   1.1 is a step. **Error above.**
3. `Do part 1.` The program starts and stops at once, because a has no
   value. From here on the program is suspended at a step, and every
   repair is followed by `Go.`
4. `Set a = 0.` Now step 1.1 divides by 2·a.
5. `Set a = -1.` Now b*2-4·a·c is -23.
6. `Set a = 1.` Step 1.1 is done at last, then 1.2, and step 1.3 wants a
   form that does not exist.
7. The form is given, with five fields. Step 1.3 has two values for it.
8. Step 1.3 is replaced. Now the values are right, but the fields for the
   roots have no room before the point, and a root is 3.
9. The form is given again with room, but the two fields touch.
10. The form is given a third time, and the answer comes out: the roots
    of x*2 - x - 6 are 3 and -2.

Seven times JOSS stopped at a step, and seven times `Go.` took it up
again at that step. Steps 1.1 and 1.2 were done once each.

Then a table of roots for other values of c. The first range goes the
wrong way: -1 in steps of +1 will never reach -4.

## It's a mess

The last mistake in the figure is of another kind.

    2.1 Do step 2.1 for i = 1(1)100.
    Do step 2.1.

Step 2.1 does step 2.1, which does step 2.1. Each `Do` needs a little
room to remember where it was, and after some dozens of them JOSS has
none left ([chapter 14](14-storage.md)):

    I need more storage space.

This is not an "Error". The program is suspended as before, and the usual
remedy is to delete something that is not needed and say `Go.` But Shaw
deletes step 2.1 itself, the step that every one of those `Do`s was in
the middle of, and then says `Go.` JOSS looks for the step to go on with
and there is nothing there:

    It's a mess.  Let's start over.

It gives up the suspended program, and only that. The steps of part 1,
the form and the values are untouched, as the last `Do` in the figure
shows. The second `Go.` finds nothing waiting.

That message is the nearest JOSS comes to a crash, and it is still a
sentence, still tells the user what to do next, and still loses nothing
the user typed.

## Error during step

A few messages begin "Error during step" rather than "Error at step".
It means the trouble is not in doing a step but in going round a `for`: the
step or part being repeated has been deleted while the program was
suspended.

```joss
1.1 Do part 2 for i = 1(1)3.
2.1 Type i.
2.2 Stop if i = 1.
Do part 1.
Delete part 2.
Go.
2.1 Type i·100.
Go.
```
```text
1.1 Do part 2 for i = 1(1)3.
2.1 Type i.
2.2 Stop if i = 1.
Do part 1.
       i =       1
Stopped by step 2.2.
Delete part 2.
Go.
Error during step 1.1:  I can't find the part for iteration.
2.1 Type i·100.
Go.
   i·100 =     200
   i·100 =     300
```

JOSS reports it against the step that holds the `Do`, and waits. Give
part 2 some steps again and `Go.` carries on with the next value of i.

Next: [Arrays](13-arrays.md)
