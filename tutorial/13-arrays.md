# 13. Arrays

[Chapter 4](04-letters.md) introduced the indexed letter: `p(1)`, `p(2)`,
`p(i)`. A letter may also have two indices, and then it is a table with
rows and columns.

```joss
Set a(1,1) = 5.
Set a(1,2) = 7.
Set a(2,1) = 6.
Set a(2,2) = 8.
Type a(1,2).
Type a(1,1)·a(2,2) - a(1,2)·a(2,1).
Type a.
```
```text
Set a(1,1) = 5.
Set a(1,2) = 7.
Set a(2,1) = 6.
Set a(2,2) = 8.
Type a(1,2).
  a(1,2) =       7
Type a(1,1)·a(2,2) - a(1,2)·a(2,1).
a(1,1)·a(2,2) - a(1,2)·a(2,1) = -2
Type a.
  a(1,1) =       5
  a(1,2) =       7
  a(2,1) =       6
  a(2,2) =       8
```

Each index is a whole number from 0 to 99, and each may be any
expression. Square brackets will do as well as round ones: `a[i,j]`. Two
indices are the limit; `a(1,2,3)` is not JOSS.

## No declarations

Nothing says in advance how big an array is to be. An element exists from
the moment it is given a value, and elements that have not been given
one do not exist and take up no room.

```joss
Set t(0) = 1.
Set t(50) = 2.
Set t(99) = 3.
Type t.
Type t(1).
```
```text
Set t(0) = 1.
Set t(50) = 2.
Set t(99) = 3.
Type t.
    t(0) =       1
   t(50) =       2
   t(99) =       3
Type t(1).
Error above:  t(1) is undefined.
```

Shaw described how it was done: "A vector, then, is represented by a list
of elements, each labeled explicitly with its index. It need not be
dense, since values are looked up by scanning the list for a match on the
explicit index" (P-2922, p. 30).

A letter is one kind of thing at a time: a single value, a row, or a
table. Giving it a value of another kind throws away what it had.

```joss
Set a(1,1) = 5.
Set a(1,2) = 7.
Set a(3) = 1.
Type a.
```
```text
Set a(1,1) = 5.
Set a(1,2) = 7.
Set a(3) = 1.
Type a.
    a(3) =       1
```

`Delete a.` deletes the whole array, and `Delete a(1,2).` one element.

## The whole array and its elements

`Type` with the bare letter types every element, in order of index. That
is the only thing the bare letter can do. Arithmetic is on single
elements, and so is a form:

```joss
Set v(1) = 3.
Set v(2) = 4.
Form 1:
__ __
Type v in form 1.
Type v(1), v(2) in form 1.
Type v+1.
```
```text
Set v(1) = 3.
Set v(2) = 4.
Form 1:
__ __
Type v in form 1.
Error above:  I need individual values for a form.
Type v(1), v(2) in form 1.
 3  4
Type v+1.
Error above:  v is undefined.
```

Anything to be done to a whole array is done with a `for`:

```joss
1.1 Set s = s + v(i)*2.
Set v(1) = 3.
Set v(2) = 4.
Set v(3) = 12.
Set s = 0.
Do part 1 for i = 1(1)3.
Type sqrt(s).
```
```text
1.1 Set s = s + v(i)*2.
Set v(1) = 3.
Set v(2) = 4.
Set v(3) = 12.
Set s = 0.
Do part 1 for i = 1(1)3.
Type sqrt(s).
 sqrt(s) =      13
```

## Filling and typing a table

Two parts and two `for`s go through a table by rows and columns. Here the
table is filled with a multiplication table, and each row is typed with a
form.

```joss
1.1 Do part 2 for j = 1(1)4.
2.1 Set m(i,j) = i·j.

3.1 Type m(i,1), m(i,2), m(i,3), m(i,4) in form 1.
Form 1:
___ ___ ___ ___

Do part 1 for i = 1(1)3.
Do part 3 for i = 1(1)3.
```
```text
1.1 Do part 2 for j = 1(1)4.
2.1 Set m(i,j) = i·j.

3.1 Type m(i,1), m(i,2), m(i,3), m(i,4) in form 1.
Form 1:
___ ___ ___ ___

Do part 1 for i = 1(1)3.
Do part 3 for i = 1(1)3.
  1   2   3   4
  2   4   6   8
  3   6   9  12
```

A form has a fixed number of fields, so the step that types a row names
each element of the row.

## Sorting

A row of numbers is sorted by going along it and exchanging neighbours
that are the wrong way round, and doing that until none are. Part 3 is
the exchange; it needs a spare letter to hold one value while the other
is moved.

```joss
1.1 Do part 2 for j = 1(1)n-i.

2.1 Do part 3 if a(j) > a(j+1).

3.1 Set t = a(j).
3.2 Set a(j) = a(j+1).
3.3 Set a(j+1) = t.

Set a(1) = 31.
Set a(2) = 4.
Set a(3) = 15.
Set a(4) = 92.
Set a(5) = 6.
Set n = 5.
Do part 1 for i = 1(1)n-1.
Type a.
```
```text
1.1 Do part 2 for j = 1(1)n-i.

2.1 Do part 3 if a(j) > a(j+1).

3.1 Set t = a(j).
3.2 Set a(j) = a(j+1).
3.3 Set a(j+1) = t.

Set a(1) = 31.
Set a(2) = 4.
Set a(3) = 15.
Set a(4) = 92.
Set a(5) = 6.
Set n = 5.
Do part 1 for i = 1(1)n-1.
Type a.
    a(1) =       4
    a(2) =       6
    a(3) =      15
    a(4) =      31
    a(5) =      92
```

## Inverting a matrix

This is page 6 of P-3131, and it is the largest program in the papers:
"Matrix inversion with simple pivoting on the diagonal". The matrix is
`a`, N rows by N columns, and it is inverted where it stands. Parts 11
and 12 ask for the elements with `Demand`, and part 21 types a row.

Shaw's comments are his own. Notice the line he abandons with an
asterisk when he sees that N has no value yet.

```joss
*  Matrix inversion with simple pivoting on the diagonal.

1.1 Set p=a(n,n).
1.2 Set a(n,n)=1.
1.3 Do part 2 for j=1(1)N.
1.4 Do part 3 for i=1(1)N.

2.1 Set a(n,j)=a(n,j)/p.

3.1 Done if i=n.
3.2 Set q=a(i,n).
3.3 Set a(i,n)=0.
3.4 Do part 4 for j=1(1)N.

4.1 Set a(i,j)=a(i,j)-a(n,j)·q.

*  JOSS can help with the input:

11.1 Demand a(i,j).
12.1 Do part 11 for j=1(1)N.

*  For the output:

21.1 Type a(i,1), a(i,2), a(i,3), a(i,4) in form 1.
Form 1:
__._____  __._____  __._____  __._____

Do part 12 for *
*  An asterisk at the end can kill the line... I forgot something:
Set N=4.
Do part 12 for i=1(1)N.
3.582
-.670
.873
1.055
-.251
4.569
.675
-.497
1.675
-.764
2.781
.778
.564
-.466
1.387
4.965
Do part 21 for i=1(1)4.
Do part 1 for n=1(1)N.
Do part 21 for i=1(1)4.

*  Invert the matrix back again:

Do part 1 for n=1(1)N.
Do part 21 for i=1(1)N.
```
```text
*  Matrix inversion with simple pivoting on the diagonal.

1.1 Set p=a(n,n).
1.2 Set a(n,n)=1.
1.3 Do part 2 for j=1(1)N.
1.4 Do part 3 for i=1(1)N.

2.1 Set a(n,j)=a(n,j)/p.

3.1 Done if i=n.
3.2 Set q=a(i,n).
3.3 Set a(i,n)=0.
3.4 Do part 4 for j=1(1)N.

4.1 Set a(i,j)=a(i,j)-a(n,j)·q.

*  JOSS can help with the input:

11.1 Demand a(i,j).
12.1 Do part 11 for j=1(1)N.

*  For the output:

21.1 Type a(i,1), a(i,2), a(i,3), a(i,4) in form 1.
Form 1:
__._____  __._____  __._____  __._____

Do part 12 for *
*  An asterisk at the end can kill the line... I forgot something:
Set N=4.
Do part 12 for i=1(1)N.
  a(1,1) = 3.582
  a(1,2) = -.670
  a(1,3) = .873
  a(1,4) = 1.055
  a(2,1) = -.251
  a(2,2) = 4.569
  a(2,3) = .675
  a(2,4) = -.497
  a(3,1) = 1.675
  a(3,2) = -.764
  a(3,3) = 2.781
  a(3,4) = .778
  a(4,1) = .564
  a(4,2) = -.466
  a(4,3) = 1.387
  a(4,4) = 4.965
Do part 21 for i=1(1)4.
 3.58200   -.67000    .87300   1.05500
 -.25100   4.56900    .67500   -.49700
 1.67500   -.76400   2.78100    .77800
  .56400   -.46600   1.38700   4.96500
Do part 1 for n=1(1)N.
Do part 21 for i=1(1)4.
  .32861    .02877   -.08326   -.05390
  .04839    .21513   -.07921    .02366
 -.19033    .04018    .42024   -.02139
  .02038    .00570   -.11537    .21573

*  Invert the matrix back again:

Do part 1 for n=1(1)N.
Do part 21 for i=1(1)N.
 3.58200   -.67000    .87300   1.05500
 -.25100   4.56900    .67500   -.49700
 1.67500   -.76400   2.78100    .77800
  .56400   -.46600   1.38700   4.96500
```

Part 1 is done once for each row n. It divides row n by its diagonal
element p (part 2), and then takes the right multiple of row n away from
every other row (parts 3 and 4). `3.1 Done if i=n.` passes over row n
itself. Putting 1 in the place of the pivot, and 0 in the rest of its
column, before the row operations is what makes the inverse appear in the
same sixteen places that held the matrix.

The whole inversion is the one line `Do part 1 for n=1(1)N.` Doing it
twice gives back the matrix that was typed in, to every figure shown.

The numbers JOSS typed on 28 April 1965 are the numbers above. This page
of P-3131 is one of those the test suite replays and compares.

Next: [Storage](14-storage.md)
