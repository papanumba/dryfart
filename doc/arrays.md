## Arrays

Arrays must have all the elements of the same type, e.g. `Z%` array `_0, 1, 2;`,
`R%` array `_0.0, 1.0, 2.0;`, etc.

Array constructors are denoted starting by underscore `_`, then the elements
separated by a commas `,`, and ended by a semicolon `;`.

```
a = _1, 2, 3;.
```

Arrays are objects, which means that they are heap-allocated and only the
references are stored to variables.

### Element access

If `a` is an array and `i` is a `N%` or positive `Z%` value, then `a_i` would
be the (0-indexed) i-th element of a. The `_` works as any other binary
operator, has stronger precedence than the `*` multiplication and weaker than
the `$` field access.

Examples of access:
* Explicit arrays: `_1, 2;_0`
* Identifiers: `a_0` also `a_(0)`
* Table's field: `t$a_0` also `(t$a)_0`

So as to know the length of the array, there's the (TODO) built-in function 
`len#`. It will return a `N%`.

For example, to set a value in an array:

```
a_0 = 1 + 2 + 3.
```

### Strings (TODO)

Strings are implemented as `C%` character arrays. Their explicit array are
the usual strings surrounded by double quotes `"`.
Escape sequences start by a backtick:
* ``"`N"`` newline
* ``"`T"`` tab
* ``"`""`` double quote
* ``"`'"`` single quote

### Operations

`+` can be used to concatenate 2 arrays (and create a new one).

```
a = _1, 2;.
b = _3, 4;.
```

Then `a + b` will be `_1, 2, 3, 4;`.

[Next ch.](tables.md)
