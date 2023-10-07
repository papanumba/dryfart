# DryFart

Wellcome to the docs of the DryFart language.

## General description

Features:
* Lazy evaluation of blocks
* Imperative: procedural & a bit of functional
* Strong typing in expressions
* Dynamic typing in variables
* Static typing in func/proc parameters
* No keywords, that's why it's "Dry"
* (TODO) modular

## Hello World

Doesn't work yet :P but should be something like:

```
put!"Hello, world!",.
```

## Types & Variables

There are 5 primitive types:
* `B%` boolean: its values are `T` or `F`,
* `C%` character: ASCII char (as in C),
* `N%` natural: 32-bit unsigned int
* `Z%` integer: 32-bit int
* `R%` real: 32-bit float

Variables are declared by initializing them. This example shows a variable of
type `N%` _natural_ assigned a value of 25:

```
natural = 25.
```

As you see, statements like this end in a dot, since we don't want to get
semicolon cancer like many C-style languages.

_Real_ numbers must have a decimal part, e.g. to represent `3` as a _real_
`3.` is not valid, it must be `3.0`.

## Arithmetic

The numerical types (i.e. N%, Z%, R%) support basic arithmetic but only when
they are of the same type, there's no implicit typecasting because of the strong
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
x = N% 1.
y = 1.0 / R% x.
```

## Boolean stuff

The comparison operators are the classic relations of order (<, >, <=, >=) and
equivalence (==, /=). The order operators work with the numerical types; the
equivalence work with all 5 types except R%. A comparison gives a B%.

The logical operators are `&` for "and", `|` for "or" and `~` for "not".

[Next ch.](control.md)
