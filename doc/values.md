## Types

A value can be of type:
* `V%` void:
	* the value `V`, i.e. the NULL, nil, None, you get it.
* `B%` boolean: can be `T` true xor `F` false
* `C%` character: ASCII char
* `N%` natural: 32-bit unsigned int
* `Z%` integer: 32-bit integer
* `R%` real: 32-bit float
* `_%` array: see [Arrays](arrays.md)
* `$%` table: see [Tables](tables.md)
* `#%` function: see [Subroutines](funcs_n_procs.md)
* `!%` procedure: see [Subroutines](funcs_n_procs.md)

## Variables

Variables are declared by initializing them. This example shows a variable of
type `Z%` _integer_ assigned a value of 25:

```
z = 25.
```

As you see, statements like this end in a dot, that's because we don't want to
get C-style semicolon cancer.

_Real_ `%R` numbers must have a decimal part, e.g. to represent `3` as a _real_
`3.` is not valid, it must be `3.0`.

## Arithmetic

The numerical types (i.e. N%, Z%, R%) support basic arithmetic but only when
they are of the same type, there's no implicit coercing because of the strong
typing. Their behaviour follows this rules:

```
N% + N% -> N%
N% * N% -> N%

Z% + Z% -> Z%
Z% - Z% -> Z%
Z% * Z% -> Z%

R% + R% -> R%
R% - R% -> R%
R% * R% -> R%
R% / R% -> R%
```

If you want (explicit) type casting, trust me you'll need it, just put the type
before the value:

```
x = 1.
y = 1.0 / R% x.
```

## Boolean stuff

The comparison operators are the classic relations of order (<, >, <=, >=) and
equivalence (==, ~=). The order operators work with the numerical types; the
equivalence work with all 5 types except `R%`. A comparison gives a `B%`.

The logical short-circuit (TODO in VM) operators are `&&` for "and", `||` for "or".
And the standard boolean `~` for "not".

