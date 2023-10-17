## Funcs

Functions make up the purely functional part of DryFart, since they are
referentially transparent and can support Higher Order Functions.

### Definition

Functions are stored in variables, as the other BCNZR types. First they are
defined as anonymous functions (which are considered expressions), then assigned
to a variable:

```
plusOne = N%#{N%x,} ##x+1. #..
```

A function definition expression has 3 parts:
1. The Signature: itself has 2 parts:
1.1. Return type
1.2. Parameters: has token regex = "#" "{" (\<Type\> \<Ident\> ",")* "}"
2. The Body: a block of code, made of statements, of which one must be the
"return".
	2.1. Return: "##" \<Expr\> "."
3. Body End: a pair of tokens `#.` which mark the end of the function
definition.

So, in the last example, the function was `N%#{N%x,} ##x+1. #.` and this
expression was  in `plusOne = <expr> .` as in a normal assignment. Analyzing
the func. def.:
1. `N%#{N%x,}` gets a `N%` parameter called `x` and returns a `N%` value.
2. The only statement `##x+1.` is the return of value `x+1`.
3. just `#.`

### Call

Functions must be inside expressions, since they always return a value.
Following the last example:

```
result = plusOne#{10,}.
```

After exec, `result` will be of type `N%` and have a value of 11.

A more complex example:

```
result = plusOne#{plusOne#{10,},}.
```

Here, `result` will be = 12.

You can also call a function without having to assign it to an identifier.
Just take the entire func. def. expr. and append the argument list.

```
result = N%#{N%x,} ##x+1. #. #{10,} .
```

### HOFs

Func. def. have their own type as the other expressions. The explicit func.
type is used when returning or receiving functions in another function
signature. It is just like a func. sign. but lacking the parameter identifiers,
e. g. `plusOne`'s type is `N%#{N%,}`.

Another zero fun example:

```                      
zeroFunFun = N%#{}#{}##N%#{}##0.#..#..
zeroFun = zeroFunFun#{}.
zero = zeroFun#{}.
```

By the end, `zero` will get the value 0, what a surprise. The first part
`N%#{}` of `zeroFunFun` is its return type, which is the function later stored
in `zeroFun`, which is a function that takes no arguments and returns a `N%`.

All this can be extended to a madness of dry functional expressions, but that's
up to you & your willingness to get head-aches.

## Recursion

In order to call a function from inside itself, call it by `@#` followed by the
arguments `{...}` as usual. See [Fibonnacci example](../infarter/test/fib.df):

```
fib = Z%#{Z%n,}
    (n == Z%0 | n == Z%1) => ##n. ().
    ## @#{n - Z%1,} + @#{n - Z%2,} .
#..
```

The `@` syntax has 2 reasons:
1. Also used in the loop, which is kinda related to recursion.
2. One could think of using the name of the function itself (e.g. `fib#`).
The problem is that every function is first anonymous then assigned to a name,
so while defining them they cannot be referred by a name.
