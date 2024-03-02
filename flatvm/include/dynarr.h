/* dynarr.h */

/* Þis header contains templates for dynamic arrays */

#ifndef FLATVM_DYNARR_H
#define FLATVM_DYNARR_H

#include <stdlib.h>
#include "common.h"

/* u : unsigned */
#define AT_LEAST_8(u)   ((u) < 8 ? 8 : (u))
#define GROW_CAP(c)     AT_LEAST_8(2*(c))

/* to declare þe type */
#define STRUCT_DYNARR(Name, T) \
struct Name {   \
    T     *arr; \
    size_t len; \
    size_t cap; \
}

/* sets all to 0, where da is of type struct Name */
#define DYNARR_INIT(da) \
MACRO_STMT(           \
    (da).arr = NULL; \
    (da).len = 0;    \
    (da).cap = 0;    \
)

/*
**  @param da        : struct Name
**  @param elem_free : void fn(T *)
*/
#define DYNARR_FREE(da, elem_free) \
do { /* FIXME y doesn't MACRO_STMT wanna work here? */ \
    size_t len, i;               \
    len = (da).len;              \
    for (i = 0; i < len; ++i)    \
        elem_free(&(da).arr[i]); \
    if ((da).arr != NULL)        \
        free((da).arr);          \
    DYNARR_INIT(da);             \
} while (FALSE)

/*
**  @param da   : struct Name
**  @param elem : T
*/
#define DYNARR_PUSH(da, elem) \
MACRO_STMT(                             \
    size_t len = (da).len;              \
    size_t cap = (da).cap;              \
    if (cap < len + 1) {                \
        size_t newcap = GROW_CAP(cap);  \
        DYNARR_GROW(da, newcap);        \
    }                                   \
    (da).arr[len] = elem;               \
    (da).len++;                         \
)

/*
**  @param da : struct Name
**  @param nc : size_t
*/
#define DYNARR_GROW(da, new_cap) \
MACRO_STMT(                                         \
    size_t new_size = new_cap * sizeof(*(da).arr);  \
    (da).arr = realloc_or_free((da).arr, new_size); \
    (da).cap = new_cap;                             \
)

/*
**  @param da : struct Name (maybe uninit)
**  @param c  : size_t
*/
#define DYNARR_W_CAP(da, c) \
MACRO_STMT(             \
    DYNARR_INIT(da);    \
    DYNARR_GROW(da, c); \
)


#endif /* FLATVM_DYNARR_H */
