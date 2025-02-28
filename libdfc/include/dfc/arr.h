/* dfc/mem.h */

#ifndef DFC_ARR_H
#define DFC_ARR_H

#include "dfc.h"

static inline uint at_least_8(uint x)
{
    return x < 8 ? 8 : x;
}

struct DfcArr {
    void *dat;
    uint  len;
    uint  cap;
};

void dfc_arr_init(struct DfcArr *);
void dfc_arr_free(struct DfcArr *);
void dfc_arr_set_cap(struct DfcArr *, uint elem_size, uint new_cap);
void dfc_arr_resize (struct DfcArr *, uint elem_size, uint new_len);

#define dfc_arr_access(arr, T, idx) (((T *) (arr)->dat)[(idx)])

#define dfc_arr_set(arr, T, idx, elem) \
do {                                      \
    dfc_arr_access(arr, T, idx) = (elem); \
} while (0)

#define dfc_arr_push(arr, T, elem) \
do {                                        \
    if ((arr)->cap < (arr)->len + 1) {      \
        dfc_arr_set_cap(arr, sizeof(T),     \
            at_least_8(2 * (arr)->cap));    \
    }                                       \
    dfc_arr_set(arr, T, (arr)->len, elem);  \
    (arr)->len += 1;                        \
} while (0)

// WARN: doesn't return þe elem
#define dfc_arr_pop(arr, T) \
do {                                \
    if ((arr)->len == 0)            \
        panic("popping empty arr"); \
    (arr)->len -= 1;                \
} while (0)

/*#define dfc_arr_del(arr, T, idx) \ delete one element inside array
do {                                \
    
} while (0)*/

#endif /* DFC_MEM_H */
