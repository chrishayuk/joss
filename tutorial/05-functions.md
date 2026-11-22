# 5. Functions

JOSS has thirteen functions. Their names are in small letters, and the
argument goes in brackets, round or square, straight after the name.

| | |
|---|---|
| `sqrt(x)` | square root |
| `log(x)` | natural logarithm |
| `exp(x)` | e to the power x |
| `sin(x)` `cos(x)` | sine and cosine, x in radians |
| `arg(x,y)` | the angle of the point [x,y] |
| `ip(x)` | integer part |
| `fp(x)` | fraction part |
| `dp(x)` | digit part |
| `xp(x)` | exponent part |
| `sgn(x)` | sign: -1, 0 or 1 |
| `max(a,b,...)` | the largest |
| `min(a,b,...)` | the smallest |

There are no others, and you cannot define your own. A piece of
calculation that is wanted in several places is made a part of a stored
program ([chapter 6](06-steps-and-parts.md)).

## Roots, logarithms and exponentials

```joss
Type sqrt(144), sqrt(2), sqrt[sqrt(16)].
Type log(10), exp(1), exp(log(7)).
```
```text
Type sqrt(144), sqrt(2), sqrt[sqrt(16)].
sqrt(144) =     12
 sqrt(2) =       1.41421356
sqrt[sqrt(16)] =  2
Type log(10), exp(1), exp(log(7)).
 log(10) =       2.30258509
  exp(1) =       2.71828183
exp(log(7)) =    7.00000001
```

The last line is a reminder that each function gives nine digits and no
more: the logarithm of 7 is rounded, and the exponential of the rounded
number is not quite 7.

`log` is the natural logarithm. For a logarithm to base ten, divide by
`log(10)`:

```joss
Type log(1000)/log(10).
```
```text
Type log(1000)/log(10).
log(1000)/log(10) =  3
```

Shaw took trouble over the cases a user would notice: "Care has been
taken in a basic set of elementary functions to hit certain "magic" values
on the nose and to provide reasonably full significance of results"
(P-2922, p. 25).

```joss
Type sqrt(49), log(1), exp(0), sin(0), cos(0), 4*.5, 10*(-3).
```
```text
Type sqrt(49), log(1), exp(0), sin(0), cos(0), 4*.5, 10*(-3).
sqrt(49) =       7
  log(1) =       0
  exp(0) =       1
  sin(0) =       0
  cos(0) =       1
    4*.5 =       2
 10*(-3) =        .001
```

Each has a kind of argument it will not take, and a message that says so:

```joss
Type sqrt(-1).
Type log(0).
```
```text
Type sqrt(-1).
Error above:  I have a negative argument for sqrt.
Type log(0).
Error above:  I have an argument ≤ 0 for log.
```

## Sines and cosines

Angles are in radians. There is no tangent: divide.

```joss
Set p = 3.14159265.
Type sin(p/6), cos(p/3), sin(p/4)/cos(p/4).
Type sin(.5)*2+cos(.5)*2.
Type sin(100).
```
```text
Set p = 3.14159265.
Type sin(p/6), cos(p/3), sin(p/4)/cos(p/4).
sin(p/6) =        .499999999
cos(p/3) =        .500000001
sin(p/4)/cos(p/4) =  1
Type sin(.5)*2+cos(.5)*2.
sin(.5)*2+cos(.5)*2 =  1
Type sin(100).
Error above:  Please keep |x|<100 for sin(x) or cos(x).
```

The sine of π/6 is a half. The `p` here is π to nine digits, which is not
quite π, and the answer says so in its last place.

Beyond 100 radians nine digits are not enough to say where in the circle
the angle falls, and JOSS declines.

## arg

`arg(x,y)` is the angle, between -π and π, of the line from the origin to
the point [x,y]. It is the inverse tangent of y/x, with the quadrant right
and no trouble when x is nought.

```joss
Type arg(1,0), arg(0,1), arg(-1,0), arg(0,-1).
Type arg(1,1), 4·arg(1,1).
```
```text
Type arg(1,0), arg(0,1), arg(-1,0), arg(0,-1).
arg(1,0) =       0
arg(0,1) =       1.57079633
arg(-1,0) =      3.14159265
arg(0,-1) =     -1.57079633
Type arg(1,1), 4·arg(1,1).
arg(1,1) =        .785398163
4·arg(1,1) =     3.14159265
```

JOSS has no name for π. `4·arg(1,1)` is the usual way to get it, and
`arg(-1,0)` is shorter.

The other inverse functions come from `arg`. The angle whose sine is s is
`arg(sqrt(1-s*2), s)`:

```joss
Set s = .5.
Type arg(sqrt(1-s*2), s), 3.14159265/6.
```
```text
Set s = .5.
Type arg(sqrt(1-s*2), s), 3.14159265/6.
arg(sqrt(1-s*2), s) =  .523598775
3.14159265/6 =    .523598775
```

## Taking a number apart

`ip` and `fp` are the two sides of the decimal point. `dp` and `xp` are
the two halves of scientific notation: the digits, as a number from 1 up
to 10, and the power of ten.

```joss
Set y = 123.456.
Type y, ip(y), fp(y), dp(y), xp(y).
Type ip(y)+fp(y), dp(y)·10*xp(y).
```
```text
Set y = 123.456.
Type y, ip(y), fp(y), dp(y), xp(y).
       y =     123.456
   ip(y) =     123
   fp(y) =        .456
   dp(y) =       1.23456
   xp(y) =       2
Type ip(y)+fp(y), dp(y)·10*xp(y).
ip(y)+fp(y) =  123.456
dp(y)·10*xp(y) =  123.456
```

All four keep the sign of the number:

```joss
Type ip(-3.7), fp(-3.7), dp(-3.7), xp(-3.7).
Type dp(.00625), xp(.00625).
```
```text
Type ip(-3.7), fp(-3.7), dp(-3.7), xp(-3.7).
ip(-3.7) =      -3
fp(-3.7) =       -.7
dp(-3.7) =      -3.7
xp(-3.7) =       0
Type dp(.00625), xp(.00625).
dp(.00625) =     6.25
xp(.00625) =    -3
```

`ip` and `fp` are used more than any other function, because between them
they answer most questions about whole numbers. JOSS has no remainder and
no test for "is a whole number", and does not need them:

```joss
Set n = 17.
Type fp(n/2), fp(n/17).
Type n - 5·ip(n/5).
Type ip(n/10), n - 10·ip(n/10).
Type ip(2.5+.5), ip(3.49+.5).
```
```text
Set n = 17.
Type fp(n/2), fp(n/17).
 fp(n/2) =        .5
fp(n/17) =       0
Type n - 5·ip(n/5).
n - 5·ip(n/5) =  2
Type ip(n/10), n - 10·ip(n/10).
ip(n/10) =       1
n - 10·ip(n/10) =  7
Type ip(2.5+.5), ip(3.49+.5).
ip(2.5+.5) =     3
ip(3.49+.5) =    3
```

`fp(n/d)` is nought exactly when d divides n. `n - d·ip(n/d)` is the
remainder. `ip(n/10)` drops the last digit of a whole number and the
remainder is that digit. `ip(x+.5)` rounds a positive number to the
nearest whole one. Because the arithmetic is decimal, `fp(x/5) = 1/5` and
tests like it are exact: they are in the original programs, and in
[chapter 8](08-conditions.md).

## sgn, max and min

```joss
Type sgn(-3.5), sgn(0), sgn(3.5).
Set x = 3.
Type max(6·x, 20, x*3), min(6·x, 20, x*3).
Type max(x, 0), min(x, 0).
```
```text
Type sgn(-3.5), sgn(0), sgn(3.5).
sgn(-3.5) =     -1
  sgn(0) =       0
sgn(3.5) =       1
Set x = 3.
Type max(6·x, 20, x*3), min(6·x, 20, x*3).
max(6·x, 20, x*3) =  27
min(6·x, 20, x*3) =  18
Type max(x, 0), min(x, 0).
max(x, 0) =      3
min(x, 0) =      0
```

`max` and `min` take two arguments or more.

## What "Eh?" covers

A function with the wrong number of arguments, a name that is not one of
the thirteen, or a name without its brackets is not JOSS.

```joss
Type sqrt(1,2).
Type max(3).
Type tan(1).
Type sqrt 4.
Type Sqrt(4).
```
```text
Type sqrt(1,2).
Eh?
Type max(3).
Eh?
Type tan(1).
Eh?
Type sqrt 4.
Eh?
Type Sqrt(4).
Eh?
```

Next: [Steps and parts](06-steps-and-parts.md)
