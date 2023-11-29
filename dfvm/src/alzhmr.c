/* memory.c */

#include <stdlib.h>
#include "../include/alzhmr.h"

void *realloc_or_free(void *ptr, size_t new_size)
{
    void *result = NULL;
    if (new_size == 0) { /* no need to realloc */
        free(ptr);
    } else {
        result = realloc(ptr, new_size);
        if (result == NULL)
            exit(1);
    }
    return result;
}

/* expected b to be 2 byte Big Endian */
short uc2toh_be(uchar *b)
{
    union { unsigned short us; short s; } u;
    u.us = (b[0] << 8)
          | b[1];
    return u.s;
}

/* expected b to be 4 byte Big Endian */
int uc4toi_be(uchar *b)
{
    union { uint ui; int i; } u;
    u.ui = (b[0] << 24)
         | (b[1] << 16)
         | (b[2] <<  8)
         |  b[3];
    return u.i;
}

