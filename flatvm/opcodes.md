
jumps:
    direct(2): `JP[SL]` þe tipical jump 
    conditional(14)
        - non-POP( 2): JB[TF] // þese are Short
        - popping(12): `J([TFEN][SL]|[LG][TE])` þe unm are Long

| Hex  | OP    | operands     | stack before | stack after | Explnation             |
|------|-------|--------------|--------------|-------------|------------------------|
| 0x00 | `NOP` |              |              |             | do noþing              |
| 0x01 | `LVV` |              |              | V           | Load Void Value        |
| 0x02 | `LBT` |              |              | T           | Load Bool True         |
| 0x03 | `LBF` |              |              | F           | Load Bool False        |
| 0x04 | `LN0` |              |              | 0           | Load Natural 0         |
| 0x05 | `LN1` |              |              | 1           | Load Natural 1         |
| 0x06 | `LN2` |              |              | 2           | Load Natural 2         |
| 0x07 | `LN3` |              |              | 3           | Load Natural 3         |
| 0x08 | `LM1` |              |              | -1          | Load Minus 1 (zahl)    |
| 0x09 | `LZ0` |              |              | 0           | Load Zahl 0            |
| 0x0A | `LZ1` |              |              | 1           | Load Zahl 1            |
| 0x0B | `LZ2` |              |              | 2           | Load Zahl 2            |
| 0x0C | `LR0` |              |              | 0.0         | Load Real 0            |
| 0x0D | `LR1` |              |              | 1.0         | Load Real 1            |
| 0x0E | `LKS` | u8:ctn idx   |              | [K]         | Loads Konstant Short   |
| 0x0F | `LKL` | u16:ctn idx  |              | [K]         | Loads Konstant Long    |
|      |       |              |              |             |                        |
| 0x50 | `JJS` | i8:rel dist  |              |             | Jump Short             |
| 0x51 | `JJL` | i16 "        |              |             | Jump Long              |
| 0x52 | `JBT` | i8  "        |              |             | Jump Bool True         |
| 0x53 | `JBF` | i8  "        |              |             | Jump Bool False        |
| 0x54 | `JTS` | i8  "        |              |             | Jump True Short        |
| 0x55 | `JTL` | i16 "        |              |             | Jump True Long         |
| 0x56 | `JFS` | i8  "        |              |             | Jump False Short       |
| 0x57 | `JFL` | i16 "        |              |             | Jump False Long        |
| 0x58 | `JES` | i8  "        |              |             | Jump Equal Short       |
| 0x59 | `JEL` | i16 "        |              |             | Jump Equal Long        |
| 0x5A | `JNS` | i8  "        |              |             | Jump Not equal Short   |
| 0x5B | `JNL` | i16 "        |              |             | Jump Not equal Long    |
| 0x5C | `JLT` | i16 "        |              |             | Jump Less Than         |
| 0x5D | `JLE` | i16 "        |              |             | Jump Less or Equal     |
| 0x5E | `JGT` | i16 "        |              |             | Jump Greater Than      |
| 0x5F | `JGE` | i16 "        |              |             | Jump Greater or Equal  |
|      |       |              |              |             |                        |
| 0x60 | `AMN` |              |              | _;          | Array Make New         |
| 0x61 | `APE` |              | _%arr, elem  | arr         | Array Push Element     |
| 0x62 | `AGE` |              | _%arr, N%idx | elem        | Array Get Element      |
| 0x62 | `ASE` |              | _%arr, N%idx | elem        | Array Set Element      |
|      |       |              |              |             |                        |
| 0x70 | `TMN` |              |              | $;          | Table Make New         |
| 0x71 | `TSF` | u16:idf idx  | $%tab, val   | $%tab       | Table Set Field        |
| 0x72 | `TGF` | u16:idf idx  | $%tab        | value       | Table Get Field        |
|      |       |              |              |             |                        |
| 0x80 | `PMN` | u16:pag idx  |              | proc        | Procedure Make New     |
| 0x82 | `PCL` | u8:arity     |              | proc        | Procedure CaLL         |
|      |       |              |              |             |                        |
| 0xE6 | `CAN` |              | val          | N%val       | CAst N%                |
| 0xE8 | `CAZ` |              | val          | Z%val       | CAst Z%                |
| 0xEA | `CAR` |              | val          | R%val       | CAst R%                |
|      |       |              |              |             |                        |
| 0xF0 | `RET` |              | val          | val (outer) | RETurn func #%         |
| 0xF1 | `END` |              |              |             | END proc !%            |
| 0xF4 | `DUP` |              | val          | val, val    | DUPlicate              |
| 0xF5 | `SWP` |              | a, b         | b, a        | SWaP top 2 vals        |
| 0xF8 | `POP` |              | val          |             | POP                    |
| 0xFF | `HLT` |              |              | ºOº         | HaLT                   |
