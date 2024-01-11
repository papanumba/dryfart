# DryFart

Wellcome to the docs of the DryFart language.

## General description

Features:
* Basic control flow: if-else & loops
* Imperative: both procedural & functional
* Strong & dynamic typing
* Variable length arrays
* Tables (dynamic structs)
* (TODO) modular

## Hello World

Doesn't work yet :P but should be something like:

```
putLn!"Hello, world!".
```

## Types & Variables

There are 5 primitive types:
* `B%` boolean: its values are `T` or `F`,
* `C%` character: ASCII char (as in C),
* `N%` natural: 32-bit unsigned int
* `Z%` integer: 32-bit int
* `R%` real: 32-bit float

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

The logical short-circuit operators are `&&` for "and", `||` for "or".
And the standard boolean `~` for "not".

## Chapters

* [Control flow](control.md)
* [Subroutines](funcs_n_procs.md)
* [Arrays](arrays.md)
* [Tables](tables.md)
