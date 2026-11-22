# 11. To, Done and Stop

The steps of a part are done one after another, and a `Do` with a `for`
does them again. Three verbs change the order from inside. All three can
only be steps: given directly they mean nothing, and JOSS says so.

```joss
To step 1.1.
Done.
Stop.
```
```text
To step 1.1.
Error above:  Don't give this command directly.
Done.
Error above:  Don't give this command directly.
Stop.
Error above:  Don't give this command directly.
```

## To

`To step n.` goes to step n, and the steps are done in order from there.

With a condition it makes a loop that runs until something happens, which
`for` cannot do. This is Euclid's algorithm for the greatest common
divisor: divide, keep the remainder, and go round again until the
remainder is nought.

```joss
1.1 Set r = a - b·ip(a/b).
1.2 Set a = b.
1.3 Set b = r.
1.4 To step 1.1 if b ≠ 0.
1.5 Type a.

Set a = 1071.
Set b = 462.
Do part 1.
```
```text
1.1 Set r = a - b·ip(a/b).
1.2 Set a = b.
1.3 Set b = r.
1.4 To step 1.1 if b ≠ 0.
1.5 Type a.

Set a = 1071.
Set b = 462.
Do part 1.
       a =      21
```

A `To` may go forwards as well, to pass over steps:

```joss
1.1 To step 1.4 if x ≥ 0.
1.2 Type "negative".
1.3 Set x = -x.
1.4 Type sqrt(x).
Do part 1 for x = 9, -9.
```
```text
1.1 To step 1.4 if x ≥ 0.
1.2 Type "negative".
1.3 Set x = -x.
1.4 Type sqrt(x).
Do part 1 for x = 9, -9.
 sqrt(x) =       3
negative
 sqrt(x) =       3
```

The step must exist:

```joss
1.1 To step 1.7.
Do part 1.
```
```text
1.1 To step 1.7.
Do part 1.
Error at step 1.1:  I can't find the required step.
```

## To part

`To part n.` goes to the first step of part n. The work goes on there,
and when that part comes to its end the `Do` that started things is
finished. A `To` is not a `Do`: JOSS does not come back to the step after
it.

```joss
1.1 Type "part 1".
1.2 To part 2.
1.3 Type "never typed".

2.1 Type "part 2".

Do part 1.
```
```text
1.1 Type "part 1".
1.2 To part 2.
1.3 Type "never typed".

2.1 Type "part 2".

Do part 1.
part 1
part 2
```

Compare `Do part 2.` in the same place:

```joss
1.1 Type "part 1".
1.2 Do part 2.
1.3 Type "back in part 1".

2.1 Type "part 2".

Do part 1.
```
```text
1.1 Type "part 1".
1.2 Do part 2.
1.3 Type "back in part 1".

2.1 Type "part 2".

Do part 1.
part 1
part 2
back in part 1
```

The use of `To part` is to choose between two ways of finishing. Here a
quadratic has real roots or it has not, and each case has a part and a
form of its own.

```joss
1.1 Set d = b*2 - 4·a·c.
1.2 To part 2 if d < 0.
1.3 Type [-b+sqrt(d)]/[2·a], [-b-sqrt(d)]/[2·a] in form 1.

2.1 Type -b/[2·a], sqrt(-d)/[2·a] in form 2.

Form 1:
real roots     ___.____ and ___.____
Form 2:
complex roots  ___.____ plus and minus ___.____ i

Set a = 1.
Set b = -2.
Do part 1 for c = -8, 5.
```
```text
1.1 Set d = b*2 - 4·a·c.
1.2 To part 2 if d < 0.
1.3 Type [-b+sqrt(d)]/[2·a], [-b-sqrt(d)]/[2·a] in form 1.

2.1 Type -b/[2·a], sqrt(-d)/[2·a] in form 2.

Form 1:
real roots     ___.____ and ___.____
Form 2:
complex roots  ___.____ plus and minus ___.____ i

Set a = 1.
Set b = -2.
Do part 1 for c = -8, 5.
real roots       4.0000 and  -2.0000
complex roots    1.0000 plus and minus   2.0000 i
```

When c is 5, step 1.2 goes to part 2, part 2 ends, and that is one turn
of the `for` complete.

`To part 1.` as the last step of part 1 goes round the whole part again.
Shaw's root finder in [chapter 15](15-worked-examples.md) does that.

## Done

`Done.` leaves the part at once. The steps after it are not done. If the
part is being done by a `for`, the `for` goes on to its next value.

```joss
1.1 Done if fp(n/3) = 0.
1.2 Type n.
Do part 1 for n = 1(1)7.
```
```text
1.1 Done if fp(n/3) = 0.
1.2 Type n.
Do part 1 for n = 1(1)7.
       n =       1
       n =       2
       n =       4
       n =       5
       n =       7
```

It almost always has a condition, and it usually stands first in a part,
to turn away the cases the part is not for. The matrix inversion in
[chapter 13](13-arrays.md) has `3.1 Done if i=n.` to leave one row alone.

`Done` leaves one part, the one it is in. A part that was doing this one
carries on:

```joss
1.1 Do part 2.
1.2 Type "part 1 goes on".

2.1 Type "in part 2".
2.2 Done.
2.3 Type "never typed".

Do part 1.
```
```text
1.1 Do part 2.
1.2 Type "part 1 goes on".

2.1 Type "in part 2".
2.2 Done.
2.3 Type "never typed".

Do part 1.
in part 2
part 1 goes on
```

## Stop

`Stop.` hands the typewriter back to the user in the middle of a
program. JOSS says where it stopped and waits.

```joss
1.1 Type "before".
1.2 Stop.
1.3 Type "after".
Do part 1.
Go.
```
```text
1.1 Type "before".
1.2 Stop.
1.3 Type "after".
Do part 1.
before
Stopped by step 1.2.
Go.
after
```

`Go.` takes the program up again at the step after the `Stop`. In between
the user can do anything that can be done directly: look at values,
change them, change steps.

That makes `Stop` a breakpoint. Put one where you want to look, with a
condition if you only want to look sometimes, and take it out again when
you have seen enough.

```joss
1.1 Set t = t + n*2.
1.2 Stop if n = 3.
1.3 Type n, t in form 1.
Form 1:
n = __   sum of squares = ____

Set t = 0.
Do part 1 for n = 1(1)5.
Type n, t.
Set t = 1000.
Delete step 1.2.
Go.
```
```text
1.1 Set t = t + n*2.
1.2 Stop if n = 3.
1.3 Type n, t in form 1.
Form 1:
n = __   sum of squares = ____

Set t = 0.
Do part 1 for n = 1(1)5.
n =  1   sum of squares =    1
n =  2   sum of squares =    5
Stopped by step 1.2.
Type n, t.
       n =       3
       t =      14
Set t = 1000.
Delete step 1.2.
Go.
n =  3   sum of squares = 1000
n =  4   sum of squares = 1016
n =  5   sum of squares = 1041
```

The program stopped with n at 3, the user looked, gave t another value
and deleted the `Stop`, and the program carried on with the new value as
if nothing had happened.

A program that is stopped is not finished. It is waiting, and it waits
until the user types `Go.`, or `Cancel.`, or starts something else with a
new `Do`. The next chapter is about that waiting state, which is where
JOSS is most itself.

Next: [Mistakes and Go](12-mistakes-and-go.md)
