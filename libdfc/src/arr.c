/* arr.c */

#include <stdlib.h>
#include "dfc/arr.h"

void dfc_arr_init(struct DfcArr *arr)
{
    arr->dat = NULL;
    arr->len = 0;
    arr->cap = 0;
}

void dfc_arr_free(struct DfcArr *arr)
{
    free(arr->dat);
}

void dfc_arr_set_cap(struct DfcArr *arr, uint elem_size, uint new_cap)
{
    arr->cap = new_cap;
    arr->dat = realloc(arr->dat, new_cap * elem_size);
    if (arr->dat == NULL && new_cap != 0) { /* failed */
        exit(EXIT_FAILURE);
    }
}

/* warning: may lose data or reserve uninitialized bytes */
void dfc_arr_resize(struct DfcArr *arr, uint elem_size, uint new_len)
{
    dfc_arr_set_cap(arr, elem_size, new_len);
    arr->len = new_len;
}
