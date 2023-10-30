## Arrays

Arrays must have all the elements of the same type: N% array `{0, 1, 2,}`,
R% array `{0.0, 1.0, 2.0,}`, etc.

Explicit arrays are denoted each element followed by a comma `,`, then
surounded by curly braces `{}`. They can be stored to variables as other types:
(primitives BCNZR & functions).

When passed to a function, its type is denoted as the elements' type surrounded
by `{}`. For example, a function that recieves a natural array and returns a
boolean would be of type `B%#{{N%},}`.

TODO: optimization, by now all arrays are deep-copied so when passing arrays to
functions, its horribly slow.

### Element access

If `a` is an array and `i` is a `N%`, then `a_i` would be the i-th element of a.
This operator can be used with the following expressions:
* Explicit arrays: `{1, 2,}_0`
* Identifiers: `a_0`
* Function calls: `myFunc#{}_0` if `myFunc` returns an array.

So as to know the length of the array, there's the built-in function 
`len#{a,}`. It is (sugar-syntactically) overloaded to recieve any type of array
and it returns a `N%`.


### Strings

Strings are implemented as character arrays `{C%}`. Their explicit array are the
usual strings surrounded by double quotes `""`.
Escape sequences are followed by a dollar `$`:
* `"N$"` newline
* `"T$"` tab
* `""$"` double quote char
* `"$$"` dollar itself
