# DryFart

Wellcome to the docs of the DryFart language.

Before reading these documents, make sure you have good understanding about programming languages, paradigms and usual terms.

## General description

Features:
* No keywords, only symbols, that's why it's _Dry_
* Control flow: if-else, loops, and a (TODO) switch
* Imperative: both procedural and functional
* Functional: all subroutines are 1st-class 
* Closures with capture by value (only VM)
* Mutability in functions (only VM)
* Strong typing in operations
* Dynamically typed values
* Dynamic homogeneous arrays
* Tables (proto-OOP)
* Garbage collected
    * RC in the interpreter
    * tracing GC in the VM
* a standard library (in construction)

## Sources of Inspiration

* Pascal:
	* distinction between functions vs procedures.
	* strong typing
	* (TODO) Case statement syntax. (Note: I **hate** to have to `break;` every `case` in a C switch)
* Lua, Lox: tables, indexed by identifiers.
* Python: no declaration, only assignments
* Bash: gotta luv da `[ ]` and `[[ ]]` for conditions

## Hello World

May be broken until a stable version :stuck_out_tongue: but should be something like:

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
