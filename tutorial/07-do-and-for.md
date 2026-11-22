# 7. Do and for

## for

A `Do` may be followed by `for`, a letter, and the values the letter is to
take. The step or part is done once for each.

```joss
1.1 Type x*2.
Do part 1 for x = 1, 2, 3.
```
```text
1.1 Type x*2.
Do part 1 for x = 1, 2, 3.
     x*2 =       1
     x*2 =       4
     x*2 =       9
```

The values may be any expressions. They are worked out when the `Do` is
met.

```joss
1.1 Type x, sqrt(x), _.
Set a = 5.
Do part 1 for x = a, a+1, 10·a.
```
```text
1.1 Type x, sqrt(x), _.
Set a = 5.
Do part 1 for x = a, a+1, 10·a.
       x =       5
 sqrt(x) =       2.23606798

       x =       6
 sqrt(x) =       2.44948974

       x =      50
 sqrt(x) =       7.07106781

```

## Ranges

A run of evenly spaced values is written `a(b)c`: from a, in steps of b,
to c. The step goes in the brackets.

```joss
1.1 Type x in form 1.
Form 1:
___.__
Do part 1 for x = 1(1)5.
Do part 1 for x = 0(.25)1.
Do part 1 for x = 10(-2)4.
```
```text
1.1 Type x in form 1.
Form 1:
___.__
Do part 1 for x = 1(1)5.
  1.00
  2.00
  3.00
  4.00
  5.00
Do part 1 for x = 0(.25)1.
   .00
   .25
   .50
   .75
  1.00
Do part 1 for x = 10(-2)4.
 10.00
  8.00
  6.00
  4.00
```

(`Form 1` is there to make the output short: it types the number alone,
without `x =`. Forms are [chapter 9](09-forms.md).)

A range may go on to change its step, `a(b)c(d)e`, and ranges and single
values may be mixed, with commas between. This range is the one in
figure 3g of P-2922:

```joss
1.1 Type x in form 1.
Form 1:
___.__
Do part 1 for x = 8.50(.01)8.54(.02)8.60, 9, 9.5.
```
```text
1.1 Type x in form 1.
Form 1:
___.__
Do part 1 for x = 8.50(.01)8.54(.02)8.60, 9, 9.5.
  8.50
  8.51
  8.52
  8.53
  8.54
  8.56
  8.58
  8.60
  9.00
  9.50
```

Because JOSS counts in decimal, a step of .01 is exactly a hundredth and
the run lands exactly on 8.54. A hundred steps of .01 from 0 arrive at 1
and stop there:

```joss
1.1 Set n = n+1.
Set n = 0.
Do part 1 for x = 0(.01)1.
Type n, x.
```
```text
1.1 Set n = n+1.
Set n = 0.
Do part 1 for x = 0(.01)1.
Type n, x.
       n =     101
       x =       1
```

## The limit is always the last value

When the next step would go past the limit, the limit itself is used as
the last value. A range always ends on its limit, whether or not the step
divides the distance.

```joss
1.1 Type x in form 1.
Form 1:
___.__
Do part 1 for x = 1(2)6.
Do part 1 for x = 0(.3)1.
```
```text
1.1 Type x in form 1.
Form 1:
___.__
Do part 1 for x = 1(2)6.
  1.00
  3.00
  5.00
  6.00
Do part 1 for x = 0(.3)1.
   .00
   .30
   .60
   .90
  1.00
```

The first run is 1, 3, 5 and then 6. If you are counting on every value
being a multiple of the step, make the limit one.

The step must lead towards the limit. A step of nought, or one that
points the wrong way, is refused before anything is done:

```joss
1.1 Type x.
Do part 1 for x = 5(1)1.
Do part 1 for x = 1(0)5.
Do part 1 for x = 5(-1)1.
```
```text
1.1 Type x.
Do part 1 for x = 5(1)1.
Error above:  Illegal set of values for iteration.
Do part 1 for x = 1(0)5.
Error above:  Illegal set of values for iteration.
Do part 1 for x = 5(-1)1.
       x =       5
       x =       4
       x =       3
       x =       2
       x =       1
```

A range from a number to itself is that one value.

## A letter before the bracket

A bracket that follows a letter is usually an index
([chapter 4](04-letters.md)). In the values of a `for` it is read as the
step of a range instead, whenever another value follows it: `a(h)8` is
from a, in steps of h, to 8.

```joss
1.1 Type x in form 1.
Form 1:
___.__
Set a = 2.
Set h = 2.
Do part 1 for x = a(h)8.
Set v(1) = 5.
Do part 1 for x = v(1), 2·a(h)4·a.
```
```text
1.1 Type x in form 1.
Form 1:
___.__
Set a = 2.
Set h = 2.
Do part 1 for x = a(h)8.
  2.00
  4.00
  6.00
  8.00
Set v(1) = 5.
Do part 1 for x = v(1), 2·a(h)4·a.
  5.00
  4.00
  6.00
  8.00
```

`v(1)` has nothing after it but a comma, so it is the first element of
`v`. JOSS's own summary of January 1965 gives the pattern in letters:
`Do part 1 for x = a(b)c(d)e, f, g.`

## The letter afterwards

The letter of a `for` is an ordinary letter. Inside the part it can be
used like any other, and when the `Do` is finished it still has its last
value.

```joss
1.1 Set s = s+i.
Set s = 0.
Do part 1 for i = 1(1)100.
Type i, s.
```
```text
1.1 Set s = s+i.
Set s = 0.
Do part 1 for i = 1(1)100.
Type i, s.
       i =     100
       s =    5050
```

## Parts that do parts

A step may be a `Do`. The part it names is done, and then the step after
the `Do` is done. This is how one loop is put inside another.

```joss
1.1 Do part 2 for j = 1(1)i.
1.2 Line.

2.1 Type i, j, i·j in form 1.

Form 1:
_ times _ is __

Do part 1 for i = 1(1)3.
```
```text
1.1 Do part 2 for j = 1(1)i.
1.2 Line.

2.1 Type i, j, i·j in form 1.

Form 1:
_ times _ is __

Do part 1 for i = 1(1)3.
1 times 1 is  1

2 times 1 is  2
2 times 2 is  4

3 times 1 is  3
3 times 2 is  6
3 times 3 is  9

```

`Line.` types a blank line. The limit of the inner range is `i`, the
letter of the outer one, and it is worked out afresh each time step 1.1
is reached.

Parts may be nested as deeply as there is room for. A part may even do
itself: see [chapter 14](14-storage.md).

## Do step

`Do step` with a `for` does the one step over and over. For a loop of one
line it saves giving the step a part of its own:

```joss
1.1 Set s = 0.
1.2 Set s = s + 1/k*2.
1.3 Type s, 3.14159265*2/6.
Do step 1.1.
Do step 1.2 for k = 1(1)1000.
Do step 1.3.
```
```text
1.1 Set s = 0.
1.2 Set s = s + 1/k*2.
1.3 Type s, 3.14159265*2/6.
Do step 1.1.
Do step 1.2 for k = 1(1)1000.
Do step 1.3.
       s =       1.64393452
3.14159265*2/6 =  1.64493406
```

## Do as a calculator's memory

Nothing says a part must be done from another step. A part that works out
something from a letter can be tried by hand, for one value and then
another, before anything else is written:

```joss
2.1 Set y = x*3 - 2·x - 5.
2.2 Type x, y.
Do part 2 for x = 2.
Do part 2 for x = 2.1.
Do part 2 for x = 2.09, 2.095.
```
```text
2.1 Set y = x*3 - 2·x - 5.
2.2 Type x, y.
Do part 2 for x = 2.
       x =       2
       y =      -1
Do part 2 for x = 2.1.
       x =       2.1
       y =        .061
Do part 2 for x = 2.09, 2.095.
       x =       2.09
       y =       -.050671
       x =       2.095
       y =        .00500738
```

`Do part 2 for x = 2.` is the nearest thing in JOSS to calling a function
with an argument. Shaw's root-finding program in
[chapter 15](15-worked-examples.md) is used in just this way.

Next: [Conditions](08-conditions.md)
