## Operators

### Arithmetic

The numerical types (i.e. N%, Z%, R%) support basic arithmetic but only when
they are of the same type (except the modulus operator `\`, there's no implicit coercing because of the strong
typing. Their behaviour follows this rules:

```
N% + N% -> N%
N% * N% -> N%
N% \ N% -> N%

Z% + Z% -> Z%
Z% - Z% -> Z%
Z% * Z% -> Z%
Z% \ N% -> N%

R% + R% -> R%
R% - R% -> R%
R% * R% -> R%
R% / R% -> R%
```

If you want (explicit) type casting, trust me you'll need it more often than you might think, just put the type
before the value:

```
x = 2.
y = 1.0 / R%x.
```

### Comparison

The classic relations of equivalence (`==`, `~=`) and order (`<`, `>`, `<=`, `>=`).

Things to have in mind:
* The order ones only work with the numerical types;
* `R%` doesn't have an equivalence in the interpreter because of Rust.
* All comparisons give a `B%` value.

## Boolean & bit stuff

There are the bitwise operators `~`, `&`, `|` , `^` (not, and, or, xor), which can operate between `B%` as 1-bit and `N%` as 32-bit. In some future, there will be the shifts.

There are also the short-circuit operators are `&?`, `|?` (and, or).

## Assignement

Unlike C, the `=` assignement cannot be used as an expression, it is solely a statement.

There are also the combined assignement operators, which are made by just duplicating the operator, i.e. the C's `+=` is `++`.

The currently supported ones 
are `+`, `-`, `*`, `/`, `\`, `&`, `|`, `^`.

For example, to find if `q` is a [quadratic residue](https://en.wikipedia.org/wiki/Quadratic_residue) mod `n`, both `N%` values, a program might look like:

```
isQuadRes = F.
x = 0u. @[[x < n &? ~isQuadRes]]
  y = x.
  y ** y.
  y \\ n.
  [y == q => isQuadRes = T.]
  x ++ 1u.
.
```

[Next ch.](control.md)
