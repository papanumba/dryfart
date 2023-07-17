# Control flow

## If-then-(else)

Example:

```
(a > 0) =>
    put!"yes",.
() =>
    put!"no",.
().
```

There is no else-if/elif in-between case.

## While loop

Example:

```
a = 0.
@(a < 10)
    put!"a",.
    a = a + 1.
@.
```

Example with "break" inside an "if":

```
a = 0.
@(a < 10)
    (a == 5) =>
        @().
    ().
    put!"a",.
    a = a + 1.
@.
```

[Next ch.](funcs_n_procs.md)
