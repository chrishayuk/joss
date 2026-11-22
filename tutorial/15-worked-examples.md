# 15. Worked examples

"The use of the JOSS system is best taught by example", Shaw wrote
(P-3149, p. 9), and in April 1965 he sat down at a console and typed
eight pages of them. They were printed as P-3131, in two colours as they
came off the typewriter. This chapter goes through those pages. Each session is as he
typed it; each is in `examples/` under a name beginning `1965-`.

The prime numbers of P-3149 are in [chapter 8](08-conditions.md), the
page of mistakes in [chapter 12](12-mistakes-and-go.md) and the matrix
inversion in [chapter 13](13-arrays.md).

## The hypotenuse, and Pythagorean triples

Page 3. A program in two parts and a form, listed, run, and then changed
in two lines to do something else.

```joss
*  Stored program for computing the hypotenuse:

1.1 Set c=sqrt(a*2+b*2).
1.2 Type a,b,c in form 1.
Form 1:
a = __   b = __   c = ___.______
2.1 Do part 1 for b=1(1)a.
2.2 Line.
Type all steps.
Type form 1.

Do part 2 for a=1(1)4.
*  Modification to type only integral c's:

Delete step 2.2.
1.2 Type a,b,c in form 1 if fp(c)=0.
Do part 2 for a=1(1)25.
```
```text
*  Stored program for computing the hypotenuse:

1.1 Set c=sqrt(a*2+b*2).
1.2 Type a,b,c in form 1.
Form 1:
a = __   b = __   c = ___.______
2.1 Do part 1 for b=1(1)a.
2.2 Line.
Type all steps.

1.1 Set c=sqrt(a*2+b*2).
1.2 Type a,b,c in form 1.

2.1 Do part 1 for b=1(1)a.
2.2 Line.

Type form 1.
a = __   b = __   c = ___.______

Do part 2 for a=1(1)4.
a =  1   b =  1   c =   1.414214

a =  2   b =  1   c =   2.236068
a =  2   b =  2   c =   2.828427

a =  3   b =  1   c =   3.162278
a =  3   b =  2   c =   3.605551
a =  3   b =  3   c =   4.242641

a =  4   b =  1   c =   4.123106
a =  4   b =  2   c =   4.472136
a =  4   b =  3   c =   5.000000
a =  4   b =  4   c =   5.656854

*  Modification to type only integral c's:

Delete step 2.2.
1.2 Type a,b,c in form 1 if fp(c)=0.
Do part 2 for a=1(1)25.
a =  4   b =  3   c =   5.000000
a =  8   b =  6   c =  10.000000
a = 12   b =  5   c =  13.000000
a = 12   b =  9   c =  15.000000
a = 15   b =  8   c =  17.000000
a = 16   b = 12   c =  20.000000
a = 20   b = 15   c =  25.000000
a = 21   b = 20   c =  29.000000
a = 24   b =  7   c =  25.000000
a = 24   b = 10   c =  26.000000
a = 24   b = 18   c =  30.000000
```

Part 1 works out one hypotenuse and types it. Part 2 does part 1 for
every b up to a, and then a blank line. The direct `Do` does part 2 for
each a. That is three loops deep, and only the innermost has any
arithmetic in it.

The modification is the interesting part. A table of every hypotenuse
becomes a search for right-angled triangles with whole-number sides by
adding `if fp(c)=0` to one step and deleting the step that spaced the
table. Nothing was retyped that did not change. The square root of a
perfect square is exact in JOSS, so the test can be trusted.

## Integration by Gauss's two-point rule

Page 4. The integral of 1/x from 1 to X, which is log X, for X from 1 to
10.

```joss
*  Integration of 1/x by Gauss two-point rule:

1.1 Type X,Y in form 1 if fp(X)=0.
1.2 Do part 2 for x=X+p·h, X+q·h.
2.1 Set y=1/x.
2.2 Set Y=Y+.5·h·y.
Form 1:
__   _._____
Set p=.5-.5/sqrt(3).
Set q=.5+.5/sqrt(3).
Set Y=0.
Set h=.1.
Do part 1 for X=1(h)10.
Type 10, log(10) in form 1.
Type all.
```
```text
*  Integration of 1/x by Gauss two-point rule:

1.1 Type X,Y in form 1 if fp(X)=0.
1.2 Do part 2 for x=X+p·h, X+q·h.
2.1 Set y=1/x.
2.2 Set Y=Y+.5·h·y.
Form 1:
__   _._____
Set p=.5-.5/sqrt(3).
Set q=.5+.5/sqrt(3).
Set Y=0.
Set h=.1.
Do part 1 for X=1(h)10.
 1    .00000
 2    .69315
 3   1.09861
 4   1.38629
 5   1.60944
 6   1.79176
 7   1.94591
 8   2.07944
 9   2.19722
10   2.30259
Type 10, log(10) in form 1.
10   2.30259
Type all.

1.1 Type X,Y in form 1 if fp(X)=0.
1.2 Do part 2 for x=X+p·h, X+q·h.

2.1 Set y=1/x.
2.2 Set Y=Y+.5·h·y.


Form 1:
__   _._____


       h =        .1
       p =        .211324866
       q =        .788675134
       x =      10.0788675
       y =        .0992174964
       X =      10
       Y =       2.31253534

```

The rule takes two points in each strip of width h, at the fractions p
and q of the way across, and gives each half the weight of the strip.
Step 1.2 does part 2 at those two points: a `for` with a list of two
expressions. Part 2 is the function being integrated, in step 2.1, and
the running sum, in step 2.2. To integrate something else, change step
2.1.

X goes from 1 to 10 in steps of a tenth. Step 1.1 types a line only when
X is a whole number, and step 1.2 then adds the strip that begins at X.
`fp(X)=0` is safe as a test because a tenth is exact: after ten steps X
is 2, not 1.9999999. The last
direct `Type` puts the true value of log 10 under the table in the same
form, and it agrees to all five places.

Capital and small letters are used here as a mathematician would use
them: x and y for the point, X and Y for the limit and the integral.

`Type all.` at the end shows the state the program left behind. The last
strip ran from 10 to 10.1, so Y has gone one strip past the last line
typed.

## Finding a root

Page 5. A root of exp(x) = 20·log(x) by the secant rule, with the
function in a part of its own.

```joss
*  Root finding:

1.1 Set w=y.
1.2 Do part 2 for x=x+d.
1.3 Set d=d·y/(w-y).
1.4 To part 1 if |y|≥e.
1.5 Type "Root".

2.1 Set y=exp(x)-20·log(x).
2.2 Type x,y in form 1.

Form 1:
x = __.______   y = __.______

Type step 1.4.
Do part 2 for x=.5(.5)3.
Do part 2 for x=3.5, 4.
Do part 2 for x=1.2.
Do part 1 for d=-.05.
Set e=.00005.
Go.
Do part 2 for x=3.1.
Do part 1 for d=.1.
Do part 2 for x=x-.000001.
```
```text
*  Root finding:

1.1 Set w=y.
1.2 Do part 2 for x=x+d.
1.3 Set d=d·y/(w-y).
1.4 To part 1 if |y|≥e.
1.5 Type "Root".

2.1 Set y=exp(x)-20·log(x).
2.2 Type x,y in form 1.

Form 1:
x = __.______   y = __.______

Type step 1.4.
1.4 To part 1 if |y|≥e.
Do part 2 for x=.5(.5)3.
x =   .500000   y = 15.511665
x =  1.000000   y =  2.718282
x =  1.500000   y = -3.627613
x =  2.000000   y = -6.473888
x =  2.500000   y = -6.143321
x =  3.000000   y = -1.886709
Do part 2 for x=3.5, 4.
x =  3.500000   y =  8.060193
x =  4.000000   y = 26.872263
Do part 2 for x=1.2.
x =  1.200000   y =  -.326314
Do part 1 for d=-.05.
x =  1.150000   y =   .362954
Error at step 1.4:  e is undefined.
Set e=.00005.
Go.
x =  1.176329   y =  -.005521
x =  1.175934   y =  -.000092
x =  1.175928   y =   .000000
Root
Do part 2 for x=3.1.
x =  3.100000   y =  -.430091
Do part 1 for d=.1.
x =  3.200000   y =  1.269514
x =  3.125305   y =  -.023794
x =  3.126680   y =  -.001278
x =  3.126758   y =   .000001
Root
Do part 2 for x=x-.000001.
x =  3.126757   y =  -.000015
```

This page shows how JOSS was really used, and no description of the
language would lead you to it.

Part 2 is the function: it works out y from x and types both. Before
running anything else Shaw uses it by hand. `Do part 2 for x=.5(.5)3.`
tabulates the function, and the sign of y changes between 1 and 1.5 and
again between 3 and 3.5. Two more values confirm the second. So there are
two roots, and he knows roughly where.

Part 1 is the iteration. It remembers the last y as w, moves x by d, and
works out a new d from the two values of y. Step 1.4 goes back to the
top of the part while y is still too big. `Do part 2 for x=x+d.` is worth
a look: a `for` with a single value, used to give x a new value and call
the function in one step.

He starts it near the first root, `Do part 1 for d=-.05.`, and it stops
at once: he has not said how small is small enough. He sets e and types
`Go.`, and the iteration carries on from step 1.4 to the root. The second
root needs no repairs. The last line checks it by moving x a millionth
and watching y change sign.

## ATOM

Page 7. A puzzle from an advertisement, and a solution in one step by
C. L. Baker.

```joss
*  From Litton Industries' "Problematical Recreations" we get:
*  Problem:  Find four digits A, T, O and M such that
*            sqrt(ATOM) = A+TO+M.
*  C. L. Baker's solution:

Form 1:
ATOM = ____   sqrt(ATOM) = __
1 Type x*2, x in form 1 if x*2-9·ip(x*2/10)-99·ip(x*2/1000) = x.
Do step 1 for x=32(1)99.
```
```text
*  From Litton Industries' "Problematical Recreations" we get:
*  Problem:  Find four digits A, T, O and M such that
*            sqrt(ATOM) = A+TO+M.
*  C. L. Baker's solution:

Form 1:
ATOM = ____   sqrt(ATOM) = __
1 Type x*2, x in form 1 if x*2-9·ip(x*2/10)-99·ip(x*2/1000) = x.
Do step 1 for x=32(1)99.
ATOM = 1296   sqrt(ATOM) = 36
ATOM = 6724   sqrt(ATOM) = 82
```

ATOM is a four-digit number that is a perfect square, so its root x lies
between 32 and 99, and the program tries them all. Write n for x*2. Then
`ip(n/1000)` is the digit A and `ip(n/10)` is the number ATO. Now

    A + TO + M = n - 999·A - 9·TO = n - 9·ATO - 99·A

because ATO is 100·A + TO. So the condition in step 1 says that A+TO+M
equals x, and it says it without ever taking the number apart.

The step is numbered `1`, with no decimal part. It is the only step of
part 1, and `Do step 1` does it. There are two answers: 1+29+6 is 36, and
6+72+4 is 82.

## A formatted table

Page 8. A table of elementary functions with a heading, spaced in fives,
on a page of its own.

```joss
*  Production of a formatted table:

1.1 Do part 2 if fp(x/20) = 1/20.
1.2 Line if fp(x/5) = 1/5.
1.3 Type x, x*2, x*3, sqrt(x), log(x), exp(x) in form 1.

2.1 Page.
2.2 Type "Table of elementary functions:".
2.3 Line.
2.4 Line.
2.5 Type form 2.

Form 1:
__   ___   _____   _._____   _._____   ...........

Form 2:
 x   x*2    x*3    sqrt(x)    log(x)      exp(x)

Do part 1 for x=1(1)20.
```
```text
*  Production of a formatted table:

1.1 Do part 2 if fp(x/20) = 1/20.
1.2 Line if fp(x/5) = 1/5.
1.3 Type x, x*2, x*3, sqrt(x), log(x), exp(x) in form 1.

2.1 Page.
2.2 Type "Table of elementary functions:".
2.3 Line.
2.4 Line.
2.5 Type form 2.

Form 1:
__   ___   _____   _._____   _._____   ...........

Form 2:
 x   x*2    x*3    sqrt(x)    log(x)      exp(x)

Do part 1 for x=1(1)20.
Table of elementary functions:


 x   x*2    x*3    sqrt(x)    log(x)      exp(x)

 1     1       1   1.00000    .00000    2.71828 00
 2     4       8   1.41421    .69315    7.38906 00
 3     9      27   1.73205   1.09861    2.00855 01
 4    16      64   2.00000   1.38629    5.45982 01
 5    25     125   2.23607   1.60944    1.48413 02

 6    36     216   2.44949   1.79176    4.03429 02
 7    49     343   2.64575   1.94591    1.09663 03
 8    64     512   2.82843   2.07944    2.98096 03
 9    81     729   3.00000   2.19722    8.10308 03
10   100    1000   3.16228   2.30259    2.20265 04

11   121    1331   3.31662   2.39790    5.98741 04
12   144    1728   3.46410   2.48491    1.62755 05
13   169    2197   3.60555   2.56495    4.42413 05
14   196    2744   3.74166   2.63906    1.20260 06
15   225    3375   3.87298   2.70805    3.26902 06

16   256    4096   4.00000   2.77259    8.88611 06
17   289    4913   4.12311   2.83321    2.41550 07
18   324    5832   4.24264   2.89037    6.56600 07
19   361    6859   4.35890   2.94444    1.78482 08
20   400    8000   4.47214   2.99573    4.85165 08
```

Part 2 is the top of a page: a new sheet, a title, two blank lines and
the column headings, which are a form with no fields in it. Step 1.1 does
part 2 before lines 1, 21, 41 and so on; step 1.2 puts a blank line
before lines 1, 6, 11, 16. The tests are `fp(x/20) = 1/20` and
`fp(x/5) = 1/5`: x/5 has the fraction 1/5 when x is one more than a
multiple of 5. They can be relied on because both sides are exact
decimals.

The exponentials would not fit a fixed field, growing as they do from 2
to 485 million, so the last field is a row of periods. The result is a
page that could go straight into a report, which was the point: "the hard
copy produced is excellent--quite acceptable for reports without further
transcription and chance for error" (P-2922, p. 9).

## The examples folder

`examples/` has these sessions and more, 55 in all, each with the page it
produces in a `.out` file. `examples/README.md` lists them. Those whose
names begin with a year are from the papers. The others were written for
this reconstruction to show the language at work on familiar problems:
Fibonacci numbers, Euclid's algorithm, a sieve, Newton's square root,
Simpson's rule, a bubble sort, the date of Easter. All of them are
replayed by the test suite (`tests/examples.rs`).

Next: [1964 and 1965](16-1964-and-1965.md)
