# Funcs

## Definition

Functions are values. They are first defined as anonymous functions (which are considered expressions), then assigned to a variable, passed as an argument or whatever.

A function definition expression has 2 parts:
1. **Head**: mainly contains the names of the arguments. Starts by a hash `#`, then an optional debug name (string literal), then the comma-list of identifiers, and ends in a semicolon `;`.
2. **Body**: a block of statements, among which (in most cases) one must be the return `##` \<Expr\> `.`; then a dot `.` marks the end of the body.

Simple example:

```
add = #a,b;##a+b...
```

Here, the function is `#a,b;##a+b..` and this expression is inside the assignment statement `add = <expr> .`.

Analyzing the func. def.:
1. The head is `#a,b;`, where `a` and `b` are the parameters
2. The only statement in the body is `##a+b.`

## Call

Following the last example, to call `add` and store the result in the `result` variable:

```
result = add#1,2;.
```

An important thing to notice is that this `#` is an operator and is not tied to the `add` identifier. So, it is equivalent to  `(add)#1,2;` and `(add)#(1),(2);`.

Since function definitions are values, this is also the same as `#a,b;##a+b..#1,2;` or `(#a,b;##a+b..)#1,2;`.

## Higher Order Functions

Let's start with an example:

```
zero = #;##0...
```

This is a function with no parameters and always returns `0`. Now, to have some fun with functions, let's make one that returns the last example:

```
zeroFun = #;###;##0.....
```

Even further:

```
zeroFunFun = #;###;###;##0.......
```

So, when executing:

```
zero = #;###;###;##0......#;#;#;.
```

`zero` will have the value `0`, what a surprise.

As thou canst see, all this can be extended to a madness of dry functional expressions, but that's up to you & your willingness to get head-aches.

## Recursion

In order to call a function from inside itself, call it by `#@`. See [Fibonnacci example](../infarter/test/fib.df):

```
fib = #n;[n<2=>##n.]###@#n-1;+#@#n-2;...
```

The `@` syntax has 2 reasons:
1. Also used in the loop, which is kinda related to recursion.
2. One could think of using the name of the function itself (e.g. `fib#`).
The problem is that every function is first anonymous then assigned to a name,
so while defining them they cannot be referred by a name.

