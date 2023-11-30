/* alzhmr.c */

#include <stdlib.h>
#include "alzhmr.h"

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
short b2toh(const uchar *b)
{
    union { unsigned short us; short s; } u;
    u.us = (b[0] << 8)
          | b[1];
    return u.s;
}

/* expected b to be 4 byte Big Endian */
int b4toi(const uchar *b)
{
    union { uint ui; int i; } u;
    u.ui = (b[0] << 24)
         | (b[1] << 16)
         | (b[2] <<  8)
         |  b[3];
    return u.i;
}

/* expected b to be 4 byte Big Endian */
uint b4tou(const uchar *b)
{
    return (b[0] << 24)
         | (b[1] << 16)
         | (b[2] <<  8)
         |  b[3];
}

/* expected b to be 4 byte Big Endian */
float b4tof(const uchar *b)
{
    union { uint ui; float f; } u;
    u.ui = (b[0] << 24)
         | (b[1] << 16)
         | (b[2] <<  8)
         |  b[3];
    return u.f;
}
