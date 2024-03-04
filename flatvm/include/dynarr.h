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
};

/* sets all to 0, where da is of type struct Name */
#define DYNARR_INIT(da) \
MACRO_STMT(           \
    (da).arr = NULL; \
    (da).len = 0;    \
    (da).cap = 0;    \
)

/* Þe following meθods act on struct Name &da */

/*
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
**  @param nc : size_t
*/
#define DYNARR_GROW(da, new_cap) \
MACRO_STMT(                                         \
    size_t new_size = new_cap * sizeof(*(da).arr);  \
    (da).arr = realloc_or_free((da).arr, new_size); \
    (da).cap = new_cap;                             \
)

/*
**  @param c  : size_t
*/
#define DYNARR_W_CAP(da, c) \
MACRO_STMT(             \
    DYNARR_INIT(da);    \
    DYNARR_GROW(da, c); \
)

#define DYNARR_API_H(Name, T, prefix) \
void prefix ## _init (struct Name *);           \
void prefix ## _w_cap(struct Name *, size_t);   \
void prefix ## _push (struct Name *, T);        \
void prefix ## _free (struct Name *);

#define DYNARR_API_C(Name, T, pf, free_elem) \
void pf ## _init(struct Name *da)               \
{                                               \
    DYNARR_INIT(*da);                           \
}                                               \
\
void pf ## _w_cap(struct Name *da, size_t cap)  \
{                                               \
    DYNARR_W_CAP(*da, cap);                     \
}                                               \
\
void pf ## _push(struct Name *da, T elem)       \
{                                               \
    DYNARR_PUSH(*da, elem);                     \
}                                               \
\
void pf ## _free(struct Name *da)               \
{                                               \
    DYNARR_FREE(*da, free_elem);                \
}

#endif /* FLATVM_DYNARR_H */
