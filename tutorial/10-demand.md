# 10. Demand

A stored program gets its numbers from letters, and so far every letter
has been set by the user before the program was started. `Demand` lets the
program ask.

```joss
1.1 Demand r.
1.2 Type 3.14159265·r*2.
Do part 1.
2.5
```
```text
1.1 Demand r.
1.2 Type 3.14159265·r*2.
Do part 1.
       r = 2.5
3.14159265·r*2 =  19.6349541
```

When JOSS reaches step 1.1 it types `r = ` and stops with the carriage
where it is. The typewriter is handed back, and the user finishes the
line. On paper the first half of that line was black and the second half
green. In a session file the answer is simply the next line: here, `2.5`.

`joss --marked` shows who typed what. A line that JOSS began and the user
finished is marked `D:`. With the session above saved as `area.joss`:

```bash
target/release/joss --marked area.joss
```

```
U: 1.1 Demand r.
U: 1.2 Type 3.14159265·r*2.
U: Do part 1.
D:        r = 2.5
J: 3.14159265·r*2 =  19.6349541
```

`Demand` may only be a step. Given directly it would be a roundabout
`Set`, and JOSS will not have it:

```joss
Demand r.
```
```text
Demand r.
Error above:  Don't give this command directly.
```

## The answer is an expression

What the user types is worked out like any other expression, so it need
not be a plain number. A full stop after it is allowed and not needed.

```joss
1.1 Demand a.
1.2 Demand b.
1.3 Type sqrt(a*2+b*2).
Do part 1.
3·4
10/2.
```
```text
1.1 Demand a.
1.2 Demand b.
1.3 Type sqrt(a*2+b*2).
Do part 1.
       a = 3·4
       b = 10/2.
sqrt(a*2+b*2) =  13
```

The answer may use letters that have values, including ones the program
has just asked for.

```joss
1.1 Demand x.
1.2 Demand y.
1.3 Type x, y.
Do part 1.
7
2·x+1
```
```text
1.1 Demand x.
1.2 Demand y.
1.3 Type x, y.
Do part 1.
       x = 7
       y = 2·x+1
       x =       7
       y =      15
```

## Filling a list

`Demand` takes an indexed letter as readily as a plain one, and JOSS types
the index as a number so that the user can see which value is wanted.
With a `for`, a program collects a list. This is figure 3h of P-2922: it
asks for four numbers, adds them, and then divides each by the total.

```joss
4.1 Demand b(i).
4.2 Set s = s+b(i).

Set s=0.
Do part 4 for i=1(1)4.
543
237
411
733
Type s.
5.1 Set b(i) = b(i)/s.
Do part 5 for i=1(1)4.
Type b.
```
```text
4.1 Demand b(i).
4.2 Set s = s+b(i).

Set s=0.
Do part 4 for i=1(1)4.
    b(1) = 543
    b(2) = 237
    b(3) = 411
    b(4) = 733
Type s.
       s =    1924
5.1 Set b(i) = b(i)/s.
Do part 5 for i=1(1)4.
Type b.
    b(1) =        .282224532
    b(2) =        .123180873
    b(3) =        .213617464
    b(4) =        .380977131
```

The mean and standard deviation of a list are done the same way in
`examples/mean-and-deviation.joss`.

## A wrong answer

If the answer cannot be read or cannot be worked out, that is an error at
the `Demand` step, and JOSS stops there like any other error. `Go.` makes
it ask again.

```joss
1.1 Demand n.
1.2 Type n*2.
Do part 1.
twelve
Go.
12
```
```text
1.1 Demand n.
1.2 Type n*2.
Do part 1.
       n = twelve
Error at step 1.1:  Eh?
Go.
       n = 12
     n*2 =     144
```

## Stopping a list early

A list of unknown length can be ended with a value that could not be
data. `Done` leaves the part ([chapter 11](11-to-done-stop.md)); here
step 1.2 leaves it when the answer is negative, and `To` asks for the
next.

```joss
1.1 Demand x.
1.2 Done if x < 0.
1.3 Set s = s+x.
1.4 Set n = n+1.
1.5 To step 1.1.

Set s = 0.
Set n = 0.
Do part 1.
12.5
7.25
9
-1
Type n, s, s/n.
```
```text
1.1 Demand x.
1.2 Done if x < 0.
1.3 Set s = s+x.
1.4 Set n = n+1.
1.5 To step 1.1.

Set s = 0.
Set n = 0.
Do part 1.
       x = 12.5
       x = 7.25
       x = 9
       x = -1
Type n, s, s/n.
       n =       3
       s =      28.75
     s/n =       9.58333333
```

Next: [To, Done and Stop](11-to-done-stop.md)
