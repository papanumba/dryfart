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
@ [[a < 10]]
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

The usual way to make a for loop is:

```
i = 0. @[[i < 10]]
    ...
    i ++ 1.
.
```

Maybe in the future there will be some form of `for` range loop.

**NOTE**: There will be a "break" statement, but it currently doesn't work.

[Next ch.](funcs_n_procs.md)
