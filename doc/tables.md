## Tables

Tables can store named fields with values of different types. Similar to an
object, but without methods, similar to simple Lua tables.

Table constructors are denoted starting by a dollar `$`, then 0..* "simple"
assignments (Identifier `=` Expression `.`) and ended by a semicolon `;`.

```
t = $f = 10. g = (5 < 6).;.
t = $
  f = 10.
  g = (5 < 6).
;.
```

In this example, both assignments to `t` are the same, they only differ by
some newlines.

Like arrays, tables are objects, passed by reference.

### Field access

The `$` "binary" operator is used to access a certain field, in the
previous example, `t$f` would be `10`. It is not a typical binary operator
since the RHS can only be an identifier. But the LHS can be anything of type
Table, e.g. if `$f = 10.;` is a table value, then `$f = 10.;$f` is `10`.

Fields don't have to exist to be created (unlike arrays that give Error out
of bounds). The 1st example would be equivalent to create a table, then set
each new field:

```
t = $;.
t$f = 10.
t$g = (5 < 6).
```

### Recursive or `self` reference

Sometimes, one might want to reuse a table's field to express a new one,
e.g., compute `t$y` from a previously set `t$x`:

```
t = $x = 1.;.
t$y = t$x + 1.
```

This will result in `y` = `2`.

So as to express that without having to store the table into some variable,
there's the `$@`. It is available when creating a table, it references the
table being created (itself). So the previous example can be expressed as:

```
t = $
  x = 1.
  y = $@$x + 1.
;.
```

This way, `$x = 1. y = $@$x+1.;` can be used as a stand-alone value without
storing it to `t`.

**Note:** the fields are created in the same order as in the source code, so
reversing the definitions of `x` & `y` would give some "non-existing field"
error.

This thing doesn't just end here: what happens if you want to access an even
further table? e.g.

```
t = $
  x = $
    f = 4.
  ;.
  y = $
    f = ' here i want to get x$f 
  ;.
;.
```

The solution is a relative-depth-indexed reference: `$@1`

```
t = $
  x = $
    f = 4.
  ;.
  y = $
    f = $@1$x$f.
  ;.
;.
```

Or more esoterically `t = $x=$f=4.;.y=$f=$@1$x$f.;.;.`.
Now both `f` fields will have a value of `4`.

**Note:** `$@0` is the same as `$@`.
