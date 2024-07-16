/* latin1.c */

#include <stdio.h>
#include "latin1.h"

void latin1_putchar(uint8_t c)
{
    if (c < 128) {
        // ASCII
        putchar(c);
    } else {
        // convert to 2 byte UTF-8
        uint8_t b[3] = {
            192 | (c >> 6), // 110000xx
            128 | (c & 63), // 10xxxxxx
            0               // NUL
        };
        printf("%s", b);
    }
}

void latin1_print(cbyte_p str, size_t len)
{
    TIL(i, len)
        latin1_putchar(str[i]);
}
