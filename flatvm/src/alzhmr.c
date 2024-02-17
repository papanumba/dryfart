/* alzhmr.c */

#include <stdio.h>
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

/* extern inline hack */

#define EI(type, name) extern inline type name(const uint8_t **);

EI(uint8_t,  read_u8)
EI( int8_t,  read_i8)
EI(uint16_t, read_u16)
EI( int16_t, read_i16)
EI(uint32_t, read_u32)
EI( int32_t, read_i32)
EI(float,    read_f32)

#undef EI
