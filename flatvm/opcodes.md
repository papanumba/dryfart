
jumps:
    direct(2): `JP[SL]` þe tipical jump 
    conditional(14)
        - non-POP( 2): JB[TF] // þese are Short
        - popping(12): `J([TFEN][SL]|[LG][TE])` þe unm are Long

| Hex  | OP    | operands | stack before | stack after | Explnation             |
|------|-------|----------|--------------|-------------|------------------------|
| 0x00 | `NOP` |          |              |             | do noþing              |
| 0x01 | `LVV` |          |              | V           | Load Void Value        |
| 0x02 | `LBT` |          |              | T           | Load Bool True         |
| 0x03 | `LBF` |          |              | F           | Load Bool False        |
| 0x04 | `LN0` |          |              | 0           | Load Natural 0         |
| 0x05 | `LN1` |          |              | 1           | Load Natural 1         |
| 0x06 | `LN2` |          |              | 2           | Load Natural 2         |
| 0x07 | `LN3` |          |              | 3           | Load Natural 3         |
| 0x08 | `LM1` |          |              | -1          | Load Minus 1 (zahl)    |
| 0x09 | `LZ0` |          |              | 0           | Load Zahl 0            |
| 0x0A | `LZ1` |          |              | 1           | Load Zahl 1            |
| 0x0B | `LZ2` |          |              | 2           | Load Zahl 2            |
| 0x0C | `LR0` |          |              | 0.0         | Load Real 0            |
| 0x0D | `LR1` |          |              | 1.0         | Load Real 1            |
| 0x0E | `LKS` | 0:u8     |              | [K]         | Loads Konstant Short   |
| 0x0F | `LKL` | 0:u16    |              | [K]         | Loads Konstant Long    |
|      |       |          |              |             |                        |
| 0x50 | `JJS` | 0:i8     |              |             | Jump Short             |
| 0x51 | `JJL` | 0:i16    |              |             | Jump Long              |
| 0x52 | `JBT` | 0:i8     |              |             | Jump Bool True         |
| 0x53 | `JBF` | 0:i8     |              |             | Jump Bool False        |
| 0x54 | `JTS` | 0:i8     |              |             | Jump True Short        |
| 0x55 | `JTL` | 0:i16    |              |             | Jump True Long         |
| 0x56 | `JFS` | 0:i8     |              |             | Jump False Short       |
| 0x57 | `JFL` | 0:i16    |              |             | Jump False Long        |
| 0x58 | `JES` | 0:i8     |              |             | Jump Equal Short       |
| 0x59 | `JEL` | 0:i16    |              |             | Jump Equal Long        |
| 0x5A | `JNS` | 0:i8     |              |             | Jump Not equal Short   |
| 0x5B | `JNL` | 0:i16    |              |             | Jump Not equal Long    |
| 0x5C | `JLT` | 0:i16    |              |             | Jump Less Than         |
| 0x5D | `JLE` | 0:i16    |              |             | Jump Less or Equal     |
| 0x5E | `JGT` | 0:i16    |              |             | Jump Greater Than      |
| 0x5F | `JGE` | 0:i16    |              |             | Jump Greater or Equal  |
|      |       |          |              |             |                        |
| 0x60 | `MEA` |          |              | _;          | Make Empty Array       |
| 0x61 | `TPE` |          | arr, elem    | arr         | Try Push Element       |
| 0x62 | `TGE` |          | arr, N%idx   | elem        | Try Get Element        |
