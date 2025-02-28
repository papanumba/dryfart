/* dfc.h */

#ifndef DFC_H
#define DFC_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef unsigned int uint;

#define FORU(var, beg, end) for (uint var = beg; var < end; ++var)
#define TILU(var, end) FORU(var, 0, end)

static inline uint min_uint(uint a, uint b)
{
    return a < b ? a : b;
}

#endif /* DFC_H */
