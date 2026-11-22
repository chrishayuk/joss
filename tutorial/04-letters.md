# 4. Letters

## Set

A value is kept by giving it to a letter.

```joss
Set x = 3.
Type x.
Type x+2, x·x, x*x.
```
```text
Set x = 3.
Type x.
       x =       3
Type x+2, x·x, x*x.
     x+2 =       5
     x·x =       9
     x*x =      27
```

`Set` works out the expression on the right and gives the value to the
letter on the left. JOSS types nothing in reply: an instruction that
has been carried out needs no comment. Shaw wrote that "the user
obviously does not want a message for successful completion, because it
would be so frequent and because it would intrude on his formal output"
(P-2922, p. 26).

The letter keeps its value until it is given another, and the expression
may use the old value to make the new one:

```joss
Set n = 10.
Set n = n+1.
Set n = 2·n.
Type n.
```
```text
Set n = 10.
Set n = n+1.
Set n = 2·n.
Type n.
       n =      22
```

A letter that has not been given a value has none. It is not nought.

```joss
Type y.
Set z = y+1.
```
```text
Type y.
Error above:  y is undefined.
Set z = y+1.
Error above:  y is undefined.
```

## The short way

Given directly, a `Set` may leave out the word, and the full stop with
it. The one-page summary of JOSS that RAND gave its users lists this as
`x=a` (P-3149, figure 3).

```joss
x = 5
y = x*2
Type x, y.
```
```text
x = 5
y = x*2
Type x, y.
       x =       5
       y =      25
```

This is allowed only for a line typed directly. A step of a stored program
must say `Set`.

## Fifty-two names

A name is one letter. There are 52 of them, because a capital letter and
its small letter are different names:

> The 52 upper- and lower-case letters are the only identifiers to which
> the user can assign numerical values. (P-2922, p. 24)

```joss
Set r = 2.
Set R = 100.
Type r, R, R/r.
```
```text
Set r = 2.
Set R = 100.
Type r, R, R/r.
       r =       2
       R =     100
     R/r =      50
```

Two or more letters together are never a name. They are a word of the
language, or they are nothing:

```joss
Set a = 2.
Set b = 3.
Type ab.
Type a·b.
```
```text
Set a = 2.
Set b = 3.
Type ab.
Eh?
Type a·b.
     a·b =       6
```

There are no declarations. A value is a number and nothing else: there
are no whole numbers as a separate kind, no truth values and no strings.

## Looking at what there is

`Type all values.` lists every letter that has a value, small letters
first and then capitals.

```joss
Set v = 12.5.
Set t = 4.
Set D = v·t.
Type all values.
```
```text
Set v = 12.5.
Set t = 4.
Set D = v·t.
Type all values.

       t =       4
       v =      12.5
       D =      50

```

JOSS puts a blank line before the list and another after it.

## Delete

`Delete` takes a value away again. It takes a list of letters, or `all
values`.

```joss
Set a = 1.
Set b = 2.
Set c = 3.
Delete a, b.
Type all values.
Type a.
Delete all values.
Type all values.
```
```text
Set a = 1.
Set b = 2.
Set c = 3.
Delete a, b.
Type all values.

       c =       3

Type a.
Error above:  a is undefined.
Delete all values.
Type all values.

```

Deleting a letter that has no value is not a mistake.

## Indexed letters

A letter may be given an index in brackets, and then it is a row of
values instead of one. The index is a whole number from 0 to 99.

```joss
Set p(1) = 2.
Set p(2) = 3.
Set p(3) = 5.
Type p(2).
Type p(1)+p(2)+p(3).
Set i = 3.
Type p(i), p(i-1).
```
```text
Set p(1) = 2.
Set p(2) = 3.
Set p(3) = 5.
Type p(2).
    p(2) =       3
Type p(1)+p(2)+p(3).
p(1)+p(2)+p(3) =  10
Set i = 3.
Type p(i), p(i-1).
    p(i) =       5
  p(i-1) =       3
```

The index may be any expression. Round or square brackets will do, and
the bracket must follow the letter with no space between.

`Type` with the bare letter types every value the letter has. So does
`Type all values`.

```joss
Set p(1) = 2.
Set p(2) = 3.
Set p(3) = 5.
Type p.
Delete p(2).
Type p.
```
```text
Set p(1) = 2.
Set p(2) = 3.
Set p(3) = 5.
Type p.
    p(1) =       2
    p(2) =       3
    p(3) =       5
Delete p(2).
Type p.
    p(1) =       2
    p(3) =       5
```

The elements need not be given in order, and there need not be one for
every index: only the ones that have been set exist.

A letter is either a single value or an indexed one, not both. Giving it
the other kind of value replaces what was there:

```joss
Set p(1) = 2.
Set p(2) = 3.
Set p = 7.
Type p.
Type p(1).
```
```text
Set p(1) = 2.
Set p(2) = 3.
Set p = 7.
Type p.
       p =       7
Type p(1).
Error above:  p(1) is undefined.
```

An index outside the range, or one that is not a whole number, is an
error:

```joss
Set p(100) = 1.
Set p(1.5) = 1.
```
```text
Set p(100) = 1.
Error above:  Index value must be integer and 0≤index≤99.
Set p(1.5) = 1.
Error above:  Index value must be integer and 0≤index≤99.
```

Letters with two indices are in [chapter 13](13-arrays.md).

Next: [Functions](05-functions.md)
