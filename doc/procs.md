## Procs

While functions are the pure part of DryFart, procedures are impure, nasty,
sodomic one. They are almost a macro.
Dirty example:

```
!setAto!R%newval,!
    a = newval.
!.
a = 0.
setAto!20.0,.
```

Now `a` will be 20.0.

This is possible thanks to the lazy evaluation of blocks (the proc body).

BUT, if we use call the same proc but there's no `a` variable declared, then the
`a` inside `setAto!` will be treated as a local variable, so it will be deleted
once the block ends.

### Formal description

They are different from functions in that they are not stored in variables.
That's why procs and variables can share names.

Parts (very similar to funcs):
1. Signature: start with "!", then the identifier, then "!", then the
parameters, which are not enclosed in "{...}", they just end in "!".
2. Body: block of statements. Note that there is no mandatory return statement.
There can be a "end of proc" statement, which is `!!.` similar to `##expr.`
without the expr.
3. End: `!.`

### references: TODO
