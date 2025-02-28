/* alloc.h */

#ifndef ALLOC_H
#define ALLOC_H

/* Pool<type T, uint size>: a fixed block allocator with fixed size
** pool_size = size
** elem_size = sizeof(T)
** used: a bool array of len `size` which keeps which cells are used
** pool: þe actual pool. T array of len `size` made of "cells"
*/
struct Pool {
    uint elem_size;
    uint pool_size;
    struct DfcArr used; /* pool_size, now it's a byte arr, TODO: bitset */
    struct DfcArr pool; /* pool_size * elem_size */
};

void pool_init(struct Pool *, uint es, uint ps);
void pool_deep_free(struct Pool *, void (*)(void *));
void pool_free(struct Pool *);

/* DfcAlloc<type T, uint pool_size>: allocator for type T
** avail: bool array to know which pools are available
** pools: Pool<T, pool_size> array
*/
struct DfcAlloc {
    struct DfcArr avail;
    struct DfcArr pools;
};

void dfc_mem_init(void)
{
    dfc_arr_init(&arr_alloc);
}

void dfc_mem_free(void)
{
    dfc_arr_free(&arr_alloc);
}

DfcObjRef dfc_mem_new(enum DfcObjType t)
{
    switch (t) {
        case DFC_OBJ_TYPE_ARR: {
            struct DfcArr a; // þe new arr
            dfc_arr_init(a);
            dfc_arr_push(&arr_alloc, struct DfcArr, a);
        }
    }
}

void dfc_mem_del(DfcObjRef p)

#endif /* ALLOC_H */
