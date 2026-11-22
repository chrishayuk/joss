# 17. Reference

This is JOSS-I as it stood in January 1965, the language of the one-page
summary that RAND gave its users (P-3149, figure 3), as this
reconstruction accepts it.

## A line

| A line that | is |
|---|---|
| begins with a verb | a direct instruction, done at once |
| begins with a number | a step, kept until a `Do` reaches it |
| is `Form n:` | the heading of a form; the next line is the form |
| is `x = a` | a direct `Set` without the word |
| begins or ends with `*` | ignored |
| is blank | ignored |

An instruction is a verb, what the verb needs, an optional `if` and
condition, and a full stop. One instruction to a line, one line to an
instruction.

A step number is from 1 up to (but not including) 10*9, with at most nine
digits. Its whole part is the number of the part it belongs to. A step
with a number already in use replaces the old one.

## Verbs

Direct or indirect:

| | |
|---|---|
| `Set x = a.` | give x the value a |
| `Do step 1.1.` | do one step |
| `Do part 1.` | do the steps of part 1 in order |
| `Do part 1 for x = a(b)c, d, e.` | do it for each value of x |
| `Type a, b, _.` | type values, one to a line; `_` is a blank line |
| `Type a, b in form 2.` | type values in the blanks of a form |
| `Type "text".` | type the text |
| `Type step 1.1.` `Type part 1.` `Type form 2.` | type what is stored |
| `Type all steps.` `Type all parts.` `Type all forms.` `Type all values.` | type all of one kind |
| `Type all.` | type steps, forms and values |
| `Type size.` | words of storage in use |
| `Type time.` | the time of day |
| `Type users.` | the number of consoles in use |
| `Delete x, y.` | delete values (a letter, or one element) |
| `Delete all values.` | delete every value |
| `Line.` | type a blank line |
| `Page.` | go to the top of a new page |

Direct only:

| | |
|---|---|
| `Go.` | take up the suspended program |
| `Cancel.` | give up the suspended program |
| `Delete step 1.1.` `Delete part 1.` `Delete form 2.` | delete what is stored |
| `Delete all steps.` `Delete all parts.` `Delete all forms.` | delete all of one kind |
| `Delete all.` | delete steps, forms and values |
| `Form 2:` | the next line is form 2 |
| `x = a` | set x, without the word `Set` |

Indirect only:

| | |
|---|---|
| `1.1 To step 3.1.` | go on from step 3.1 |
| `1.1 To part 3.` | go on from the first step of part 3 |
| `1.1 Done.` | leave this part |
| `1.1 Stop.` | suspend the program and return to the user |
| `1.1 Demand x.` | type `x = ` and wait for the user to give a value |

The words of the language are `Set Do Type Delete Line Page To Done Stop
Demand Cancel Go Form`, `step part form all steps parts forms values size
time users`, and `if for in and or`. Verbs begin with a capital letter.
Everything else is in small letters.

## for

    for x = a, b, c
    for x = a(b)c
    for x = a(b)c(d)e, f, g

A list of values and ranges. `a(b)c` runs from a in steps of b to c, and
c is always the last value. b may not be nought and must lead from a
towards c. All the expressions are worked out once, when the `Do` is
met. Leave a space before the bracket if what comes before it ends in a
letter.

## Conditions

    if a < b
    if a ≤ b < c
    if a = b and c ≠ d or e > f

Six relations: `=` `≠` `<` `>` `≤` `≥`. Relations may be chained. `and`
and `or` are taken from left to right, with no precedence and no
brackets, and every expression in the condition is worked out.

## Expressions

| | |
|---|---|
| numbers | digits with an optional point: `3`, `.5`, `0.5`, `12.75`; at most nine significant digits |
| letters | `a` to `z` and `A` to `Z`: 52 names |
| indexed letters | `v(i)`, `w[i,j]`; each index a whole number from 0 to 99 |
| `$` | the number of the line the typewriter is on, 1 to 54 |
| `+` `-` | plus, minus |
| `·` `/` | times, divided by |
| `*` | to the power |
| `( )` `[ ]` | brackets, interchangeable in pairs |
| `\|a\|` | absolute value |

Powers are done first, then `·` and `/`, then `+` and `-`; operators of
the same rank from left to right. A sign in front belongs to the first
term: `-3*2` is -9.

Numbers are held as nine decimal digits and a power of ten from -99 to
99. Results are rounded to nine digits; a result too large is an error
and one too small is nought.

## Functions

| | |
|---|---|
| `sqrt(a)` | square root; a must not be negative |
| `log(a)` | natural logarithm; a must be positive |
| `exp(a)` | e to the power a |
| `sin(a)` `cos(a)` | a in radians, \|a\| < 100 |
| `arg(a,b)` | angle of the point [a,b], from -π to π |
| `ip(a)` | integer part |
| `fp(a)` | fraction part |
| `dp(a)` | digit part: 1.23456 for 123.456 |
| `xp(a)` | exponent part: 2 for 123.456 |
| `sgn(a)` | -1, 0 or 1 |
| `max(a,b,...)` `min(a,b,...)` | two arguments or more |

## Forms

    Form 1:
    dist. = ___.__ miles   speed = -__._   energy = ..........

| | |
|---|---|
| `____` | a field for a whole number |
| `___.__` | a field with a point and decimals |
| `-__.__` | the same, with a place marked for the sign |
| `.......` | scientific notation: digits, then the power of ten; seven periods or more |

Values are rounded to fit and set to the right. Anything else in the form
is typed as it stands. `Type form 1.` types the form itself. The number
after `Form` may be an expression, `Form n:`, as long as its value is a
whole number from 1 to 999999999: the listing of January 1965 works it out
like any other.

When no form is given, a value is typed on a line of its own, labelled
with its expression; in fixed point if it is nought or its size is from
.001 up to 1,000,000, otherwise as `d.ddd·10*n`.

## Characters

| JOSS | | type |
|---|---|---|
| `·` | times | `@` |
| `≠` | not equal | `!=` |
| `≤` | at most | `<=` |
| `≥` | at least | `>=` |
| `*` | to the power | |
| `#` | strike-out: a space | |
| `$` | the line number | |
| `_` | a blank line in a `Type` list; a field in a form | |
| `"` | round text to be typed | |

## What JOSS says

To a line that is not JOSS:

    Eh?

To a line that cannot be done, `Error above:  ` and one of the messages
below. To a step that cannot be done, `Error at step 1.1:  ` and the
message. When a step or part being repeated by a `for` has gone,
`Error during step 1.1:  `.

Arithmetic:

| | |
|---|---|
| `I have a zero divisor.` | `1/0` |
| `I have an overflow.` | a result beyond 10*99 |
| `I have a negative argument for sqrt.` | `sqrt(-1)` |
| `I have an argument ≤ 0 for log.` | `log(0)` |
| `I have a negative base to a fractional power.` | `(-8)*.5` |
| `I have zero to a negative power.` | `0*(-1)` |
| `Please keep \|x\|<100 for sin(x) or cos(x).` | `sin(100)` |
| `I have arguments of zero for arg.` | `arg(0,0)` (supplied: see chapter 16) |

Values and numbers:

| | |
|---|---|
| `x is undefined.` | a letter or element with no value |
| `Index value must be integer and 0≤index≤99.` | `v(100)`, `v(1.5)` |
| `Please limit numbers to 9 significant digits.` | `3.141592653` |

Steps, parts and forms:

| | |
|---|---|
| `I can't find the required step.` | `Do step 9.9.` |
| `I can't find the required part.` | `Do part 9.` |
| `I can't find the required form.` | `Type x in form 9.` |
| `Step number must satisfy 1≤step<10*9.` | `0.5 Type x.` |
| `Part number must be integer and 0<part<10*9.` | `Do part 1.1.` |
| `Form number must be integer and 0<form<10*9.` | `Form 0:` |
| `Please limit step labels to 9 digits.` | `1234567890.1 Type x.` |

Typing in a form:

| | |
|---|---|
| `I have too few values for the form.` | fewer values than fields |
| `I have too many values for the form.` | more values than fields |
| `I can't express value in your form.` | a number too wide for its field |
| `Your fields run together in the form.` | two fields with nothing between |
| `Field definition has fewer than 7 periods.` | `......` |
| `I need individual values for a form.` | a whole array named in the list |

Doing things:

| | |
|---|---|
| `Illegal set of values for iteration.` | `for x = 5(1)1`, `for x = 1(0)5` |
| `I can't find the part for iteration.` | the part of a `Do ... for` was deleted |
| `I can't find the step for iteration.` | the step of a `Do ... for` was deleted |
| `Don't give this command directly.` | `To`, `Done`, `Stop` or `Demand` without a step number |
| `Don't give this command indirectly.` | `Go`, `Cancel`, a `Delete` of steps, parts or forms, as a step |
| `You haven't told me to do anything yet.` | `Go.` with nothing suspended |

Typed without "Error":

| | |
|---|---|
| `I need more storage space.` | the user's 720 words are full |
| `Stopped by step 1.2.` | a `Stop` was reached |
| `I'm at step 1.2.` | the interrupt key was pressed |
| `It's a mess.  Let's start over.` | the suspended program cannot be found; it is given up |

After any of these but the last, and after any "Error at step", the
program is suspended and `Go.` takes it up again.

Every message in these tables is typed by the session below.

```joss
Type 1/0.
Type 10*99·10.
Type sqrt(-1).
Type log(0).
Type (-8)*.5.
Type 0*(-1).
Type sin(100).
Type arg(0,0).
Type x.
Type v(100).
Type 3.141592653.
Do step 9.9.
Do part 9.
Type 1 in form 9.
0.5 Type x.
Do part 1.1.
Form 0:
1234567890.1 Type x.
Form 1:
__ and __
Type 1 in form 1.
Type 1, 2, 3 in form 1.
Type 100, 2 in form 1.
Form 2:
__.__.__
Type 1, 2 in form 2.
Form 3:
x = ......
Type 1 in form 3.
Set v(1) = 1.
Type v, v in form 1.
Stop.
1.1 Go.
Do part 1 for x = 5(1)1.
Do part 1.
Cancel.
Go.
1.1 Do part 2 for i = 1(1)3.
2.1 Stop.
Do part 1.
Delete part 2.
Go.
1.1 Do step 2.1 for i = 1(1)3.
2.1 Stop.
Do part 1.
Delete step 2.1.
Go.
3.1 Type i.
! interrupt after 2 lines
Do part 3 for i = 1(1)9.
4.1 Do step 4.1 for j = 1(1)9.
Do part 4.
Delete step 4.1.
Go.
```
```text
Type 1/0.
Error above:  I have a zero divisor.
Type 10*99·10.
Error above:  I have an overflow.
Type sqrt(-1).
Error above:  I have a negative argument for sqrt.
Type log(0).
Error above:  I have an argument ≤ 0 for log.
Type (-8)*.5.
Error above:  I have a negative base to a fractional power.
Type 0*(-1).
Error above:  I have zero to a negative power.
Type sin(100).
Error above:  Please keep |x|<100 for sin(x) or cos(x).
Type arg(0,0).
Error above:  I have arguments of zero for arg.
Type x.
Error above:  x is undefined.
Type v(100).
Error above:  Index value must be integer and 0≤index≤99.
Type 3.141592653.
Error above:  Please limit numbers to 9 significant digits.
Do step 9.9.
Error above:  I can't find the required step.
Do part 9.
Error above:  I can't find the required part.
Type 1 in form 9.
Error above:  I can't find the required form.
0.5 Type x.
Error above:  Step number must satisfy 1≤step<10*9.
Do part 1.1.
Error above:  Part number must be integer and 0<part<10*9.
Form 0:
Error above:  Form number must be integer and 0<form<10*9.
1234567890.1 Type x.
Error above:  Please limit step labels to 9 digits.
Form 1:
__ and __
Type 1 in form 1.
Error above:  I have too few values for the form.
Type 1, 2, 3 in form 1.
Error above:  I have too many values for the form.
Type 100, 2 in form 1.
Error above:  I can't express value in your form.
Form 2:
__.__.__
Type 1, 2 in form 2.
Error above:  Your fields run together in the form.
Form 3:
x = ......
Type 1 in form 3.
Error above:  Field definition has fewer than 7 periods.
Set v(1) = 1.
Type v, v in form 1.
Error above:  I need individual values for a form.
Stop.
Error above:  Don't give this command directly.
1.1 Go.
Do part 1 for x = 5(1)1.
Error above:  Illegal set of values for iteration.
Do part 1.
Error at step 1.1:  Don't give this command indirectly.
Cancel.
Go.
Error above:  You haven't told me to do anything yet.
1.1 Do part 2 for i = 1(1)3.
2.1 Stop.
Do part 1.
Stopped by step 2.1.
Delete part 2.
Go.
Error during step 1.1:  I can't find the part for iteration.
1.1 Do step 2.1 for i = 1(1)3.
2.1 Stop.
Do part 1.
Stopped by step 2.1.
Delete step 2.1.
Go.
Error during step 1.1:  I can't find the step for iteration.
3.1 Type i.
Do part 3 for i = 1(1)9.
       i =       1
       i =       2
I'm at step 3.1.
4.1 Do step 4.1 for j = 1(1)9.
Do part 4.
I need more storage space.
Delete step 4.1.
Go.
It's a mess.  Let's start over.
```

## The command line

    joss                    a console: type a line, JOSS answers
    joss file               replay a session file and print the page
    joss --marked file      the same, with U: J: and D: in front of each line
    joss --quiet file       print only what JOSS typed
    joss --colour file      print the user's lines in green
    joss --year 1964 file   the wording and columns of 1964
    joss --patience N file  press the interrupt key after N steps
    joss --check file...    read without running; report what is not JOSS
    joss --verify file...   replay transcripts of the original pages and compare

In a session file, a line beginning with `!` is a key pressed or a
setting, not a line typed:

    ! interrupt after N lines    press INTERRUPT after N lines of the next answer
    ! joss 1964                  the JOSS of 1964 from here on (or ! joss 1965)
    ! users N                    what Type users. is to answer
    ! time N                     what Type time. is to answer

With no file name and input that is not a terminal, `joss` reads the
session from standard input: `echo 'Type 2+2.' | joss`.
