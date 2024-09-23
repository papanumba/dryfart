# Control flow

Generally speaking, a "condition" is an expression that evaluates to `B%` values.
There are no truthy or falsy values e.g. 0 vs 1.

## If-Else

The basic form is `[` condition `=>` statements `]`. For example:

```
[a > 0 =>
    b = a.
    a = b.
]
```

# Control flow

Generally speaking, a "condition" is an expression that evaluates to `B%` values.
There are no truthy or falsy values e.g. 0 vs 1.

## If-Else

The basic form is `[` condition `=>` statements `]`. For example:

```
[a > 0 =>
    b = a.
    a = b.
]
```

For more cases (`else-if`), add `|` condition `=>` statements before the last
`]`. For example:

```
[a > 0 =>
    b = a.
|a < 0 =>
    b = -a.
]
```

For the `else` general case, add `|` `=>` statements before the last `]`. For
example:

```
[a > 0 =>
    b = a.
|a < 0 =>
    b = -a.
| =>
    b = 0.
]
```

## Switch

It has a similar syntax to the If-Esle, example:

```
a = 3.
b = 4.
c = V.
op = "+".
[ op :
| "+" => c = a + b.
| "-" => c = a - b.
| "*" => c = a * b.
| "/" => c = a / b.
|     => c = V. `unknown
]
```

As it is inspired by Pascal, it doesn't need a `break;` like C. Note that the expressions in each case do not need to be constants, and they're eval'd as needed successively.

## While loop

The most basic loop is the infinite loop, it is `@` statements `.`

For example:

```
@
    a = 1.
    forever = a.
.
```

For a contidion-controlled loop, add `[[` condition `]]` anywhere between the statements.

It can go at the beginning:

```
a = 0.
@
  [[a < 10]]
    a ++ 1.
.
```

in the middle:

```
a = 0.
@
    a = a.
  [[a < 10]]
    a ++ 1.
.
```

or at the end of the loop

```
a = 0.
@
    a ++ 1.
  [[a < 10]]
.
```

The usual way to make a "for" counting loop is just by ignoring the newlines:

```
i = 0. @[[i < 10]]
    ` ...
    i ++ 1.
.
```

**Note**: In the future there may be some form of actual `for` loop.

### Continue & Break

Where many languages have `continue` and `break`, DryFart has `@@` and `.@`.

Since they are statements they are always followed by a period `.` except when specifying the level (explained later).

Taking the previous example with the condition at the beginning of the loop, we can now rewrite it as an infinite loop with an `if` and a `break`:

```
a = 0.
@
    [a >= 10 => .@.]
    a ++ 1.
.
```

**Note**: The `@@`/`continue` only gets the flow to the top of the loop, it doesn't skip iterations like in a `for` loop.

#### With nested loops

C doesn't have it, but many other languages have deep nested exits. Most of them specify the loop by a label: [Ada](https://www.adaic.org/resources/add_content/standards/05aarm/html/AA-5-7.html), [D](https://tour.dlang.org/tour/en/basics/loops), [Fortran](https://www.tutorialspoint.com/fortran/fortran_exit.htm), [Go](https://www.ardanlabs.com/blog/2013/11/label-breaks-in-go.html), [Java](https://www.tutorialspoint.com/break-continue-and-label-in-Java-loop), [Javascript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/label), [Perl](https://www.perltutorial.org/perl-last/), [Rust](https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html). With all these languages following this `goto`-esque Worship of Labels*, why not `break` out of it? (pun intended)

That's why DryFart specifies the loop by relative depth. Example:

```
i = 0. @[[i < 10]]
  j = 0. @[[j < 10]]
    [j == 3 => .@1.] ` break the i loop
    j ++ 1.
  .
  i ++ 1.
.
```

The default level is zero, so `.@.` is the same as `.@0.`.

Syntactically, the level must be a `N%` or `Z%` literal but NOT an expression, even if it's compile-time computable.

*Actually, I found that [PHP](https://www.slingacademy.com/article/php-using-break-and-continue-in-loops) does something similar, but it's the only one afaik.

[Next ch.](funcs_n_procs.md)
