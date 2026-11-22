# 3. Arithmetic

## The five operations

| | |
|---|---|
| `a+b` | a plus b |
| `a-b` | a minus b |
| `a·b` | a times b |
| `a/b` | a divided by b |
| `a*b` | a to the power b |

```joss
Type 7+2, 7-2, 7·2, 7/2, 7*2.
```
```text
Type 7+2, 7-2, 7·2, 7/2, 7*2.
     7+2 =       9
     7-2 =       5
     7·2 =      14
     7/2 =       3.5
     7*2 =      49
```

Multiplication is always written. `2x` and `2(3)` are not JOSS: "Eh?".

The dot is typed as `@` on a modern keyboard (chapter 1). The asterisk was
a special one on the JOSS typewriter: "a five-pointed, upward-pointing,
slightly elevated asterisk in upper case (all of which contribute proper
associations for exponentiation)" (P-2922, p. 11).

## Which is done first

Powers are done first, then multiplications and divisions, then additions
and subtractions. Operations of the same rank are done from left to
right.

```joss
Type 2+3·4, 2·3*2, 12/2/3, 12/2·3, 2*3*2, 10-4-3.
```
```text
Type 2+3·4, 2·3*2, 12/2/3, 12/2·3, 2*3*2, 10-4-3.
   2+3·4 =      14
   2·3*2 =      18
  12/2/3 =       2
  12/2·3 =      18
   2*3*2 =      64
  10-4-3 =       3
```

`2*3*2` is 8 squared, not 2 to the 9th: powers too go from left to right.

A minus sign in front applies to the whole of the first term, so that
`-3*2` is what it would be on paper:

```joss
Type -3*2, (-3)*2, 2*(-1), 10*(-2).
```
```text
Type -3*2, (-3)*2, 2*(-1), 10*(-2).
    -3*2 =      -9
  (-3)*2 =       9
  2*(-1) =        .5
 10*(-2) =        .01
```

## Brackets

Round brackets and square brackets mean the same thing. They must match in
kind. Having two sorts makes a long expression easier to read, and that
is why the square ones were put on the keyboard: "otherwise, such
expressions tend to become cluttered with parentheses" (P-2922, p. 11).

Two upright bars take the absolute value of what is between them.

```joss
Type (2+3)·4, [2+3]·4.
Type |3-10|, |-2|·|-3|.
Type [(|3-5|·3+4)·2-15]·3+10.
Type (2+3].
```
```text
Type (2+3)·4, [2+3]·4.
 (2+3)·4 =      20
 [2+3]·4 =      20
Type |3-10|, |-2|·|-3|.
  |3-10| =       7
|-2|·|-3| =      6
Type [(|3-5|·3+4)·2-15]·3+10.
[(|3-5|·3+4)·2-15]·3+10 =  25
Type (2+3].
Eh?
```

When the expression is too long for its label to be set to the right, the
label starts at the margin and the number follows it.

## Numbers

A number is written with digits and, if it needs one, a decimal point.
There is no nought before the point when JOSS types a fraction, and you
need not type one. There is no way of writing an exponent in a numeral:
a large or small number is written as a product with a power of ten.

```joss
Type .5, 0.5, 3., 6.02·10*23, 1.6·10*(-19).
```
```text
Type .5, 0.5, 3., 6.02·10*23, 1.6·10*(-19).
Eh?
```

That was a mistake worth making. `3.` is not a number: a point belongs to
a number only when a digit follows it, because the point after a number
is the full stop that ends the sentence. JOSS read `Type .5, 0.5, 3.` as a
complete instruction and could make nothing of the rest.

```joss
Type .5, 0.5, 3, 6.02·10*23, 1.6·10*(-19).
```
```text
Type .5, 0.5, 3, 6.02·10*23, 1.6·10*(-19).
      .5 =        .5
     0.5 =        .5
       3 =       3
6.02·10*23 =     6.02·10*23
1.6·10*(-19) =   1.6·10*(-19)
```

## How JOSS types a number

"JOSS types in fixed-point form if the magnitude of the result is between
.001 and 1,000,000; otherwise, JOSS types in scientific notation"
(P-3149, p. 8). Scientific notation is written as you would type it
yourself, with a dot and a power of ten, and a negative power in
brackets.

```joss
Type 999999, 1000000, 123456·1000.
Type .001, .000999, 1/8000.
Type -1/3, 0, 100/3.
```
```text
Type 999999, 1000000, 123456·1000.
  999999 =  999999
 1000000 =       1·10*6
123456·1000 =    1.23456·10*8
Type .001, .000999, 1/8000.
    .001 =        .001
 .000999 =       9.99·10*(-4)
  1/8000 =       1.25·10*(-4)
Type -1/3, 0, 100/3.
    -1/3 =       -.333333333
       0 =       0
   100/3 =      33.3333333
```

Noughts at the end of a fraction are never typed, and neither is a decimal
point with nothing after it.

## Nine decimal digits

JOSS keeps nine significant digits of every number, and they are decimal
digits.

> JOSS represents all numbers internally in scientific notation--nine
> decimal digits of significance and a base-ten scale factor with an
> integer exponent in the range -99 through +99. [...] Addition,
> subtraction, multiplication, division, and square root are carried out
> by JOSS to give true results rounded to nine significant decimal digits
> (P-2922, p. 25)

JOHNNIAC was a binary machine with no floating point at all. Shaw wrote
decimal arithmetic for it in software so that the user would never meet
the binary one. The reward is that numbers behave as they do on paper. A
tenth is exactly a tenth:

```joss
Type .1+.1+.1+.1+.1+.1+.1+.1+.1+.1.
Type .1+.2-.3.
Type 1.15·100.
```
```text
Type .1+.1+.1+.1+.1+.1+.1+.1+.1+.1.
.1+.1+.1+.1+.1+.1+.1+.1+.1+.1 =  1
Type .1+.2-.3.
.1+.2-.3 =       0
Type 1.15·100.
1.15·100 =     115
```

"The user may ask JOSS to add .1 ten times and the result is indeed 1.
Exact arithmetic is performed so long as results do not exceed nine
decimal digits. Longer results are rounded in the conventional way"
(P-3149, p. 8). In binary floating point, which most machines have used
since, none of those three sums comes out exact.

A third is not a decimal fraction, so nine digits of it are all there
are:

```joss
Type 1/3, 2/3, 1/3+1/3+1/3.
Type 123456789+1, 123456789·10+4, 123456789·10+5.
```
```text
Type 1/3, 2/3, 1/3+1/3+1/3.
     1/3 =        .333333333
     2/3 =        .666666667
1/3+1/3+1/3 =     .999999999
Type 123456789+1, 123456789·10+4, 123456789·10+5.
123456789+1 =    1.2345679·10*8
123456789·10+4 =  1.23456789·10*9
123456789·10+5 =  1.2345679·10*9
```

The last two lines show the rounding. A tenth digit of 4 is dropped. A
tenth digit of 5 rounds upwards, to 1234567900, and the nought on the end
of 1.23456790 is not typed.

## Too big, too small, too long

The power of ten may be anything from -99 to 99. A result too big for
that is an error. A result too small is taken as nought, without comment.

```joss
Type 10*99, 10*(-99).
Type 10*99·10.
Type 10*(-99)/10.
```
```text
Type 10*99, 10*(-99).
   10*99 =       1·10*99
10*(-99) =       1·10*(-99)
Type 10*99·10.
Error above:  I have an overflow.
Type 10*(-99)/10.
10*(-99)/10 =    0
```

You may not type a number with more than nine significant digits. Noughts
in front and noughts at the end do not count.

```joss
Type 3.141592653.
Type 3.14159265.
Type 12345678900000.
```
```text
Type 3.141592653.
Error above:  Please limit numbers to 9 significant digits.
Type 3.14159265.
3.14159265 =     3.14159265
Type 12345678900000.
12345678900000 =  1.23456789·10*13
```

## Powers

Any number may be raised to any power, with three exceptions that JOSS
names for you.

```joss
Type 2*10, 2*.5, 2*(-2), 8*(1/3), (-2)*3, 0*0.
Type (-8)*(1/3).
Type 0*(-1).
Type 1/0.
```
```text
Type 2*10, 2*.5, 2*(-2), 8*(1/3), (-2)*3, 0*0.
    2*10 =    1024
    2*.5 =       1.41421356
  2*(-2) =        .25
 8*(1/3) =       2
  (-2)*3 =      -8
     0*0 =       1
Type (-8)*(1/3).
Error above:  I have a negative base to a fractional power.
Type 0*(-1).
Error above:  I have zero to a negative power.
Type 1/0.
Error above:  I have a zero divisor.
```

Next: [Letters](04-letters.md)
