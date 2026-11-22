# 9. Forms

Left to itself, JOSS types one number to a line with its label in front.
That is right for a quick answer and wrong for a table. For anything that
is to be read by somebody else, the user draws JOSS a picture of the line
that is wanted, and JOSS fills it in.

## A form

A form is given in two lines. The first is `Form`, a number and a colon.
The second is the picture.

```joss
Form 1:
The square of ___ is _____.
Type 12, 12*2 in form 1.
```
```text
Form 1:
The square of ___ is _____.
Type 12, 12*2 in form 1.
The square of  12 is   144.
```

Each run of underscores is a *field*, a blank for one number. `Type ... in
form 1.` takes a list of values, one for each field from left to right,
and types the form with the blanks filled. Everything else in the form is
typed as it stands. The number is set to the right of its field.

A form is the way to put words and numbers on one line, and with a `for`
it is the way to make a table.

```joss
1.1 Type n, n*2, n*3 in form 1.
Form 1:
___   _____   _______
Do part 1 for n = 1(1)5, 10, 50.
```
```text
1.1 Type n, n*2, n*3 in form 1.
Form 1:
___   _____   _______
Do part 1 for n = 1(1)5, 10, 50.
  1       1         1
  2       4         8
  3       9        27
  4      16        64
  5      25       125
 10     100      1000
 50    2500    125000
```

The line after `Form 1:` is taken exactly as it is typed. It is not an
instruction, it does not end with a full stop unless you want one typed,
and stand-ins such as `@` are not replaced in it.

## Decimals

A decimal point in a field says where the point is to go, and the
underscores after it say how many decimals are wanted. JOSS rounds to
fit.

```joss
Form 1:
____   ____._   ____.___   _.______
Set x = 3.14159265.
Type x, x, x, x in form 1.
Type 100·x, 100·x, 100·x, x/100 in form 1.
Type 2.5, 2.5, 2.5, 2.5 in form 1.
```
```text
Form 1:
____   ____._   ____.___   _.______
Set x = 3.14159265.
Type x, x, x, x in form 1.
   3      3.1      3.142   3.141593
Type 100·x, 100·x, 100·x, x/100 in form 1.
 314    314.2    314.159    .031416
Type 2.5, 2.5, 2.5, 2.5 in form 1.
   3      2.5      2.500   2.500000
```

A field with no point takes a whole number, and a value that is not whole
is rounded to one. As everywhere in JOSS, there is no nought in front of
a bare fraction.

A negative number needs a place for its minus sign. If the number will
not go in, JOSS does not squeeze it or spoil the line. It stops:

```joss
Form 1:
__.__
Type 3.14159 in form 1.
Type -3.14159 in form 1.
Type -31.4159 in form 1.
Type 314.159 in form 1.
```
```text
Form 1:
__.__
Type 3.14159 in form 1.
 3.14
Type -3.14159 in form 1.
-3.14
Type -31.4159 in form 1.
Error above:  I can't express value in your form.
Type 314.159 in form 1.
Error above:  I can't express value in your form.
```

So make the fields wide enough for the largest number expected, and one
wider if the numbers may be negative. The forms of the JOSS film mark the
extra place with a hyphen written against the field, which reads well and
means the same:

```joss
Form 1:
-_.___ amp.
Type 1.778902 in form 1.
Type -1.405807 in form 1.
```
```text
Form 1:
-_.___ amp.
Type 1.778902 in form 1.
 1.779 amp.
Type -1.405807 in form 1.
-1.406 amp.
```

## Scientific notation

A run of periods is the other kind of field. It types a number as a digit
part and a power of ten: "a tabular form of scientific notation in which
only the digit part and the corresponding exponent part are typed with
the base ten understood" (P-2922, p. 28).

```joss
Form 1:
.........   .............
Type 299792458, 299792458 in form 1.
Type .000123456789, -.000123456789 in form 1.
Type 1, 0 in form 1.
```
```text
Form 1:
.........   .............
Type 299792458, 299792458 in form 1.
 2.998 08    2.9979246 08
Type .000123456789, -.000123456789 in form 1.
 1.235-04   -1.2345679-04
Type 1, 0 in form 1.
 1.000 00    0.0000000 00
```

The field holds a place for the sign, the digit part, a place for the
sign of the power, and two digits of power. `2.998 08` is 2.998·10*8. The
longer the field, the more digits. It must be at least seven periods
long:

```joss
Form 1:
x = ......
Type 5 in form 1.
```
```text
Form 1:
x = ......
Type 5 in form 1.
Error above:  Field definition has fewer than 7 periods.
```

A single period is just a full stop, so a form can be a sentence.

These two kinds of field are all there are. Shaw remarked on it: "The
remarkable thing about these forms is the general adequacy of just two
types of field specification: one for fixed-point output and one for
scientific notation" (P-3149, p. 8).

## The right number of values

There must be exactly as many values as fields.

```joss
Form 1:
__ and __
Type 1 in form 1.
Type 1, 2, 3 in form 1.
Type 1, 2 in form 1.
```
```text
Form 1:
__ and __
Type 1 in form 1.
Error above:  I have too few values for the form.
Type 1, 2, 3 in form 1.
Error above:  I have too many values for the form.
Type 1, 2 in form 1.
 1 and  2
```

Two fields must have something between them, or JOSS cannot tell where
one ends:

```joss
Form 1:
___.__.__
Type 1, 2 in form 1.
```
```text
Form 1:
___.__.__
Type 1, 2 in form 1.
Error above:  Your fields run together in the form.
```

## Headings

`Type form 1.` types the form itself, blanks and all. A form with no
fields in it is therefore a line of text, and that is how a table gets
its heading.

```joss
1.1 Type form 1.
1.2 Type form 2.
1.3 Do part 2 for n = 1(1)5.

2.1 Type n, sqrt(n), 1/n in form 3.

Form 1:
  n     root    reciprocal
Form 2:
 ---   ------   ----------
Form 3:
 ___   _.____     _.____

Do part 1.
```
```text
1.1 Type form 1.
1.2 Type form 2.
1.3 Do part 2 for n = 1(1)5.

2.1 Type n, sqrt(n), 1/n in form 3.

Form 1:
  n     root    reciprocal
Form 2:
 ---   ------   ----------
Form 3:
 ___   _.____     _.____

Do part 1.
  n     root    reciprocal
 ---   ------   ----------
   1   1.0000     1.0000
   2   1.4142      .5000
   3   1.7321      .3333
   4   2.0000      .2500
   5   2.2361      .2000
```

Forms are kept by number, like steps. A form is replaced by giving a new
one with the same number, listed by `Type all forms.`, and removed by
`Delete form 1.` or `Delete all forms.`

```joss
Form 1:
x = ___
Form 2:
y = ___.__
Type all forms.
Delete form 1.
Type 5 in form 1.
```
```text
Form 1:
x = ___
Form 2:
y = ___.__
Type all forms.

Form 1:
x = ___

Form 2:
y = ___.__

Delete form 1.
Type 5 in form 1.
Error above:  I can't find the required form.
```

## Line and Page

`Line.` types a blank line. `Page.` moves the paper to the top of a new
sheet. Both are usually steps with conditions on them.

```joss
1.1 Type n, n*2 in form 1.
1.2 Line if fp(n/5) = 0.
Form 1:
___ _____
Do part 1 for n = 1(1)12.
```
```text
1.1 Type n, n*2 in form 1.
1.2 Line if fp(n/5) = 0.
Form 1:
___ _____
Do part 1 for n = 1(1)12.
  1     1
  2     4
  3     9
  4    16
  5    25

  6    36
  7    49
  8    64
  9    81
 10   100

 11   121
 12   144
```

The condition `fp(n/5) = 0` picks out every fifth line. The tables in the
papers are all spaced this way.

On a terminal there is only one sheet, so `Page.` shows nothing here. It
does start the count of lines again, which is the next thing.

## The line number: $

The paper had 54 lines to a page. JOSS keeps count of the line the
typewriter is on, and `$` is that number. It may be used in any
expression.

```joss
Type $.
Type $.
Page.
Type $.
```
```text
Type $.
       $ =       8
Type $.
       $ =      10
Page.
Type $.
       $ =       8
```

`$` is the number of the line JOSS is typing on, and every line counts,
the user's as well as JOSS's. The session began on line 7, where the user
typed the first `Type $.`, and the answer went on line 8. The second
`Type $.` was on line 9 and its answer on line 10. After `Page.` the
typewriter was on line 7 again. A sheet begins there because JOSS headed
each one with the time, the date and the user's initials, and left a
margin under them.

The use of `$` is to keep a table from running over the fold. Shaw's
example is a step that reads `Page if $=50.` (P-2922, p. 28). A step like
it at the top of a part starts a new page, and a new heading, whenever
the old page is nearly full:

```joss
1.1 Do part 2 if $ > 50.
1.2 Type n, n*2 in form 1.

2.1 Page.
2.2 Type form 2.
2.3 Line.

Form 1:
____ _______
Form 2:
   n   n*2

Do part 2.
Do part 1 for n = 1(1)3.
```
```text
1.1 Do part 2 if $ > 50.
1.2 Type n, n*2 in form 1.

2.1 Page.
2.2 Type form 2.
2.3 Line.

Form 1:
____ _______
Form 2:
   n   n*2

Do part 2.
   n   n*2

Do part 1 for n = 1(1)3.
   1       1
   2       4
   3       9
```

Left alone, JOSS turns the page itself when line 54 has been used.

Next: [Demand](10-demand.md)
