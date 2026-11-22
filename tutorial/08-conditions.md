# 8. Conditions

## if

Any instruction may end with `if` and a condition. It is done if the
condition holds, and passed over if it does not.

```joss
Set x = 7.
Type "x is big" if x > 5.
Type "x is small" if x < 5.
Set y = 0 if x = 7.
Type y.
```
```text
Set x = 7.
Type "x is big" if x > 5.
x is big
Type "x is small" if x < 5.
Set y = 0 if x = 7.
Type y.
       y =       0
```

The second `Type` did nothing, and JOSS said nothing about it.

There are six relations:

| | | you can type |
|---|---|---|
| `a = b` | equal | |
| `a ≠ b` | not equal | `a != b` |
| `a < b` | less than | |
| `a > b` | greater than | |
| `a ≤ b` | at most | `a <= b` |
| `a ≥ b` | at least | `a >= b` |

Each side is an expression. The full stop comes after the condition, at
the very end of the sentence.

## There is no else

`if` is the whole of it. There is no "otherwise" and no way to group
several instructions under one condition. To do one thing or another,
write two steps with opposite conditions:

```joss
1.1 Type "even" if fp(n/2) = 0.
1.2 Type "odd" if fp(n/2) ≠ 0.
Do part 1 for n = 4, 7.
```
```text
1.1 Type "even" if fp(n/2) = 0.
1.2 Type "odd" if fp(n/2) ≠ 0.
Do part 1 for n = 4, 7.
even
odd
```

To do several things under one condition, put them in a part and make
the `Do` conditional:

```joss
1.1 Do part 2 if x < 0.
1.2 Type x, sqrt(x).

2.1 Type "x was negative: using -x".
2.2 Set x = -x.

Do part 1 for x = 16, -25.
```
```text
1.1 Do part 2 if x < 0.
1.2 Type x, sqrt(x).

2.1 Type "x was negative: using -x".
2.2 Set x = -x.

Do part 1 for x = 16, -25.
       x =      16
 sqrt(x) =       4
x was negative: using -x
       x =      25
 sqrt(x) =       5
```

## Conditions in a stored program

A condition on a `Type` chooses which lines of a table are typed. This is
the whole of Shaw's program for prime numbers (P-3149, figure 1):

```joss
1.1 Do part 2 for d = 2(1)ip[sqrt(x)] if x > 3.
1.2 Type x if x ≠ 0.

2.1 Set x = 0 if fp(x/d) = 0.

Do part 1 for x = 2(1)30.
```
```text
1.1 Do part 2 for d = 2(1)ip[sqrt(x)] if x > 3.
1.2 Type x if x ≠ 0.

2.1 Set x = 0 if fp(x/d) = 0.

Do part 1 for x = 2(1)30.
       x =       2
       x =       3
       x =       5
       x =       7
       x =      11
       x =      13
       x =      17
       x =      19
       x =      23
       x =      29
```

Read it from the bottom. Part 1 is done for each x from 2 to 30. Step 1.1
tries every divisor d from 2 up to the square root of x, by doing part 2
for each. Step 2.1 sets x to nought if d divides it. Step 1.2 types x if
it has survived.

Three things in it are worth a second look.

Step 1.1 has both a `for` and an `if`. The `if` is for the whole step: if
x is not more than 3, the `Do` is not done at all, and its range is never
worked out. That matters here, because for x = 2 the range would be
`2(1)1`, which is not a legal one.

Step 2.1 changes x, and x is the letter of the `for` in the last line.
That is allowed. The `for` keeps its own count of where it has got to, and
gives x its next value when part 1 comes round again.

And the test for divisibility is `fp(x/d) = 0`: the fraction part of the
quotient is nought.

## Chains

Relations may be chained, as they are in mathematics. `a < b < c` holds
when `a < b` and `b < c` both do.

```joss
1.1 Type x if 3 ≤ x < 6.
Do part 1 for x = 1(1)8.
```
```text
1.1 Type x if 3 ≤ x < 6.
Do part 1 for x = 1(1)8.
       x =       3
       x =       4
       x =       5
```

Any relations may be mixed in a chain: `0 < a = b ≤ 10` is a condition.

## and, or

Conditions are joined with `and` and `or`.

```joss
1.1 Type x if x < 3 or x > 6.
Do part 1 for x = 1(1)8.
```
```text
1.1 Type x if x < 3 or x > 6.
Do part 1 for x = 1(1)8.
       x =       1
       x =       2
       x =       7
       x =       8
```

```joss
1.1 Type x if fp(x/2) = 0 and fp(x/3) = 0.
Do part 1 for x = 1(1)20.
```
```text
1.1 Type x if fp(x/2) = 0 and fp(x/3) = 0.
Do part 1 for x = 1(1)20.
       x =       6
       x =      12
       x =      18
```

When both are used in one condition they are taken strictly **from left
to right**. Neither binds more tightly than the other, and there are no
brackets for conditions. The condition

    if A or B and C

means (A or B) and C, which is not what it would mean in most later
languages.

```joss
Type "left to right" if 1 = 1 or 1 = 1 and 1 = 2.
Type "and before or" if 1 = 2 and 1 = 1 or 1 = 1.
```
```text
Type "left to right" if 1 = 1 or 1 = 1 and 1 = 2.
Type "and before or" if 1 = 2 and 1 = 1 or 1 = 1.
and before or
```

The first line typed nothing: (true or true) and false is false. The
second is (false and true) or true, which is true.

So put the `and`s first when you can. A leap year is one that divides by
4 and not by 100, or else divides by 400, and read from left to right
that is exactly what is wanted:

```joss
1.1 Type y if fp(y/4) = 0 and fp(y/100) ≠ 0 or fp(y/400) = 0.
Do part 1 for y = 1900, 1964, 1965, 2000.
```
```text
1.1 Type y if fp(y/4) = 0 and fp(y/100) ≠ 0 or fp(y/400) = 0.
Do part 1 for y = 1900, 1964, 1965, 2000.
       y =    1964
       y =    2000
```

When the order cannot be arranged, split the condition over two steps or
use a part.

## Both sides are worked out

Every expression in a condition is evaluated, whether or not the answer
is already settled. A condition cannot be used to guard its own second
half:

```joss
Set d = 0.
Type "big" if d ≠ 0 and 1/d > 10.
```
```text
Set d = 0.
Type "big" if d ≠ 0 and 1/d > 10.
Error above:  I have a zero divisor.
```

Use two steps, or a conditional `Do`.

## Conditions are not values

A condition can only follow `if`. It cannot be typed, or given to a
letter:

```joss
Type 2 > 1.
Set t = 2 > 1.
```
```text
Type 2 > 1.
Eh?
Set t = 2 > 1.
Eh?
```

When a truth value has to be kept, keep a number. `sgn` is often enough:

```joss
Set a = 5.
Set b = 3.
Set g = sgn(a-b).
Type "a is the greater" if g = 1.
```
```text
Set a = 5.
Set b = 3.
Set g = sgn(a-b).
Type "a is the greater" if g = 1.
a is the greater
```

## Direct instructions

A condition may be put on a direct instruction too. It is most useful on
a `Do`, to try a part only when it makes sense:

```joss
1.1 Type sqrt(x).
Set x = -4.
Do part 1 if x ≥ 0.
Set x = 4.
Do part 1 if x ≥ 0.
```
```text
1.1 Type sqrt(x).
Set x = -4.
Do part 1 if x ≥ 0.
Set x = 4.
Do part 1 if x ≥ 0.
 sqrt(x) =       2
```

Next: [Forms](09-forms.md)
