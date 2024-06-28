## Types

A value can be of type:
* `V%` void:
	* the only value `V`, i.e. the NULL, nil, None, you name it.
* `B%` boolean
* `C%` character: ASCII char
* `N%` natural: 32-bit unsigned int
* `Z%` integer: 32-bit integer
* `R%` real: 32-bit float
* `_%` array: see [Arrays](arrays.md)
* `$%` table: see [Tables](tables.md)
* `#%` function: see [Subroutines](funcs_n_procs.md)
* `!%` procedure: see [Subroutines](funcs_n_procs.md)

## Literals

|Type | regex | examples |
|:---:|:-----:|:--------:|
|`V`  |`V`    | `V`      |
|`B`  |`[TF]` | `T`,`F`  |
|`N`  |`\d+u` | `1u`, `100u` |
|`Z`  |`\d+`  | `1`, `100` |
|`R`  |`\d+\.\d+` | `1.0`, `3.14` |

For `C%` literals see [Strings](strings.md). For the rest, see their resp. chapters from the above section.

## Variables

Variables are declared by initializing them, like Python. This example shows a variable being assigned a `Z%` value of 25:

```
z = 25.
```

As you see, statements like this end in a dot, that's because we want some delimiter but also don't want to get C-style semicolon cancer.

[Next ch.](ops.md)
