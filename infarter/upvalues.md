## Description of how upvalues in DF work

Each subroutine in a single-file sauce code can capture variables from outer
scopes, even multilevel.

```
x = 3.
someProc = !.
    local = x.
..
```

Here, `someProc` captures x (by value) and assigns `3` to a new variable `local`
in the scope of `someProc`.

## Shadowing

Shadowing happens when declaring a variable in a subroutine scope where hasn't
been declared. Even if an outer variable exists.

```
a = 3.
someProc = !;
    a = 2. ' this a is not the outer captured, but a new local
..
```

The same works for parameters of a subroutine, as they are also local variables.

## Multi-level capture

If a variable appears further than the parent scope, every scope in between is
forced to capture. What the compiler does is, that it makes the outermost scope
capture the variable, the next scope capture that upvalue, and the every child
scope capture its parent's upvalue.

```
x = 3.
p = !.
    p2 = !.
        local = x.
    ..
..
```

`p` captures `x` because of `p2` needing `x`, and then `p2` captures the value
captured by 3.
