# 6. Steps and parts

## A step

A line that begins with a number is not done. It is kept.

```joss
1.1 Type 2+2.
1.2 Type 3+3.
```
```text
1.1 Type 2+2.
1.2 Type 3+3.
```

JOSS typed nothing, because nothing was asked of it yet. Those two lines
are *steps*, and together they are a stored program. `Do` carries it out:

```joss
1.1 Type 2+2.
1.2 Type 3+3.
Do part 1.
```
```text
1.1 Type 2+2.
1.2 Type 3+3.
Do part 1.
     2+2 =       4
     3+3 =       6
```

A line without a number is a *direct* instruction, done at once. A line
with a number is an *indirect* one, done when a `Do` reaches it. The
language is the same for both:

> A striking feature of the system is that the user commands JOSS directly
> in the same language that he uses to define procedures for JOSS to carry
> out indirectly. (P-2922, p. 14)

There is no separate mode for writing programs and no command to enter or
leave one. The number in front is all there is to it.

## Parts

A step number has a whole part and a decimal part. The whole part says
which *part* of the program the step belongs to: steps 1.1, 1.2 and 1.35
are part 1, and step 2.1 is part 2. Within a part the steps are done in
the order of their numbers, whatever order they were typed in.

```joss
2.1 Type "This is part 2.".
1.3 Type "three".
1.1 Type "one".
1.2 Type "two".
Do part 1.
Do part 2.
```
```text
2.1 Type "This is part 2.".
1.3 Type "three".
1.1 Type "one".
1.2 Type "two".
Do part 1.
one
two
three
Do part 2.
This is part 2.
```

`Do part 1.` does the steps of part 1 and stops. It does not run on into
part 2. A part is JOSS's subroutine, and
[the next chapter](07-do-and-for.md) is about what can be done with it.

A single step can be done by itself:

```joss
1.1 Type "one".
1.2 Type "two".
Do step 1.2.
```
```text
1.1 Type "one".
1.2 Type "two".
Do step 1.2.
two
```

## Looking at the program

`Type` shows what is stored: one step, one part, or everything.

```joss
1.1 Set a = 3.
1.2 Set b = 4.
1.3 Do part 2.
2.1 Type sqrt(a*2+b*2).
Type step 1.2.
Type part 1.
Type all steps.
```
```text
1.1 Set a = 3.
1.2 Set b = 4.
1.3 Do part 2.
2.1 Type sqrt(a*2+b*2).
Type step 1.2.
1.2 Set b = 4.
Type part 1.
1.1 Set a = 3.
1.2 Set b = 4.
1.3 Do part 2.
Type all steps.

1.1 Set a = 3.
1.2 Set b = 4.
1.3 Do part 2.

2.1 Type sqrt(a*2+b*2).

```

`Type all steps.` puts a blank line before the listing, after each part
and at the end. (`Type all parts.` does the same thing.)

`Type all.` types everything JOSS is holding for you: the steps, then the
forms ([chapter 9](09-forms.md)), then the values.

```joss
1.1 Set c = sqrt(a*2+b*2).
1.2 Type c.
Set a = 3.
Set b = 4.
Do part 1.
Type all.
```
```text
1.1 Set c = sqrt(a*2+b*2).
1.2 Type c.
Set a = 3.
Set b = 4.
Do part 1.
       c =       5
Type all.

1.1 Set c = sqrt(a*2+b*2).
1.2 Type c.


       a =       3
       b =       4
       c =       5

```

## Changing the program

The step number is how a program is edited.

> The numeric label determines whether an indirect step is an addition, an
> insertion, or a replacement for another step. The step numbers really do
> pay their way. (P-2922, p. 24)

To replace a step, type a new one with the same number. To put a step
between two others, give it a number between theirs: there is always
room, because a step number may have several decimal places.

```joss
1.1 Type "one".
1.2 Type "three".
Do part 1.
1.15 Type "two".
1.2 Type "three, corrected".
Type part 1.
Do part 1.
```
```text
1.1 Type "one".
1.2 Type "three".
Do part 1.
one
three
1.15 Type "two".
1.2 Type "three, corrected".
Type part 1.
1.1 Type "one".
1.15 Type "two".
1.2 Type "three, corrected".
Do part 1.
one
two
three, corrected
```

A step number may be anything from 1 up to nine digits long. Noughts at
the end do not count: 1.50 is step 1.5.

```joss
1.50 Type "a".
1.5 Type "b".
Type all steps.
0.5 Type "c".
```
```text
1.50 Type "a".
1.5 Type "b".
Type all steps.

1.5 Type "b".

0.5 Type "c".
Error above:  Step number must satisfy 1≤step<10*9.
```

## Delete

Steps are taken away one at a time, a part at a time, or all together.

```joss
1.1 Type "one".
1.2 Type "two".
2.1 Type "part two".
3.1 Type "part three".
Delete step 1.2.
Delete part 2.
Type all steps.
Delete all steps.
Type all steps.
Delete step 1.1.
```
```text
1.1 Type "one".
1.2 Type "two".
2.1 Type "part two".
3.1 Type "part three".
Delete step 1.2.
Delete part 2.
Type all steps.

1.1 Type "one".

3.1 Type "part three".

Delete all steps.
Type all steps.

Delete step 1.1.
Error above:  I can't find the required step.
```

`Delete all.` deletes steps, forms and values together and leaves JOSS as
it was at the start of the session. The sessions in the papers end with
it, to leave the console clean for the next person.

These instructions can only be given directly. A program may not delete
its own steps. (It may delete values: `Delete x.` and `Delete all values.`
are allowed as steps.)

## A step is not read until it is done

When a step is typed, JOSS looks at its number and keeps the rest as it
stands. It does not read it. A step that is not JOSS is found out when a
`Do` gets to it, and not before.

```joss
1.1 Type "one".
1.2 Tpye "two".
1.3 Type "three".
Do part 1.
```
```text
1.1 Type "one".
1.2 Tpye "two".
1.3 Type "three".
Do part 1.
one
Error at step 1.2:  Eh?
```

The message now says where: "Error at step 1.2". The mistake is in a
stored step, not in the line above. JOSS has stopped there and is
waiting. What can be done next is the subject of
[chapter 12](12-mistakes-and-go.md), but here is the short version:

```joss
1.1 Type "one".
1.2 Tpye "two".
1.3 Type "three".
Do part 1.
1.2 Type "two".
Go.
```
```text
1.1 Type "one".
1.2 Tpye "two".
1.3 Type "three".
Do part 1.
one
Error at step 1.2:  Eh?
1.2 Type "two".
Go.
two
three
```

The same holds for letters. A step may use a letter that has no value
when the step is typed. It only needs one when the step is done:

```joss
1.1 Type 2·r, 3.14159265·r*2.
Do part 1.
Set r = 10.
Do part 1.
```
```text
1.1 Type 2·r, 3.14159265·r*2.
Do part 1.
Error at step 1.1:  r is undefined.
Set r = 10.
Do part 1.
     2·r =      20
3.14159265·r*2 =  314.159265
```

This is what makes it possible to build a program a line at a time, try
it, and mend it as you go. Shaw noticed that people who had programmed
before were slow to believe it:

> Most of the nearly 200 JOSS users had no prior direct programming
> experience. Those who did betrayed this fact by the style in which they
> first used JOSS. They frequently worked on a program till they felt that
> all eventualities were covered before telling JOSS to do any processing.
> (P-3149, p. 9)

Next: [Do and for](07-do-and-for.md)
