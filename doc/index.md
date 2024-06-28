# DryFart

Wellcome to the docs of the DryFart language.

## General description

Features:
* No keywords, only symbols, that's why it's _Dry_
* Basic control flow: if-else, loops, TODO switch
* Imperative: both procedural and functional
* Functional: all subroutines are 1st-class 
* Closures (by value)
* Mutability in functions (only VM)
* Strong typing: no implicit coercion
* Dynamic typing
* Dynamic homogeneous arrays
* Tables (dynamic structs)
* (TODO) modular
* Garbage collected
* standard library (in construction)

## Sources of Inspiration

* Pascal:
	* distinction between functions vs procedures.
	* strong typing
	* (TODO) Case statement syntax. (Note: I **hate** to have to `break;` every `case` in a C switch)
* Lua, Lox: tables, indexed by identifiers.
* Python: no declaration, only assignments
* Bash: gotta love the `[ ]` and `[[ ]]` for conditions

## Hello World

May be broken until a stable version :P but should be something like:

```
STD$io$put!'Hello, world!?N'.
```

## Chapters

* [Values](values.md)
* [Operators](ops.md)
* [Control flow](control.md)
* [Subroutines](funcs_n_procs.md)
* [Arrays](arrays.md)
* [Strings](strings.md)
* [Tables](tables.md)
