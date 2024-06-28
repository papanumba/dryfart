## Strings

Strings are arrays but I thought they deserved a chapter together with characters.

Strings are delimited by single quotes, e.g. 'a string'.
Characters by double quotes, e.g. "c".

Internally, strings are implemented as `C%` arrays, but unlike the explicit array creator syntax (e.g. `_1,2,3;`), a string isn't allocated every time it is evaluated, it just references a constant created at the start of the program. Thus, `_"a"," ","s","t","r","i","n","g";` the same value as `'a string'` but would be a waste of resources.

### Escape sequences

They start by `?`:

* `?N` newline
* `?R` carriage return
* `?T` tab
* `?0` NUL char
* `?"` double quote
* `?'` single quote
* `??` ? itself

In some future, there may be numerical (hex, octo, etc) escape sequences.

[Next ch.](tables.md)
