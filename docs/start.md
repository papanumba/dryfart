## General description

Features:
* Lazy evaluation of blocks
* Imperative: procedural & a bit of functional
* Strong typing
* Static type in variables & func/proc arguments
* (TODO) modular

## Hello World

```
show!"Hello, world!",.
```

## Types & Variables

There are 5 primitive types:
* `B%` = boolean: its values are `T` or `F`,
* `C%` = character: ASCII char (as in C),
* `N%` = natural: 32-bit unsigned int
* `Z%` = integer: 32-bit (signed) int
* `R%` = real: 32-bit float

Variables are first declared, then initialized. This example show a variable of
type _natural_ declared, then assigned a value of 25:

```
N% natural.
natural = 25.
```

_Real_ numbers must have a decimal part, e.g. to represent `3` as a _real_
C-style `3.` is not valid, it must be `3.0`.

## TODO
