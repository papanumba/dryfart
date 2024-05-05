## Procs

While functions are the pure part of DryFart, procedures are nasty one.
They are used to produce side effects on parameters or upvalues.

```
setAto = !table, a.
.
t = $a = 1;.
setA
```

Now `t$a` will be 20.0.

### Formal description

They are different from functions in that they are not stored in variables.
That's why procs and variables can share names.

Parts (very similar to funcs):
1. Head: start with `!`, then the parameters comma-separated and a final "." dot.
2. Body: block of statements. Note that there is no mandatory return statement.
There can be a "end of proc" statement, which is `!!.` similar to `##expr.`
without the expr.
3. End: `.` a dot.
