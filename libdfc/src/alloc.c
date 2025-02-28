/* alloc.c */

#include <stdio.h>
#include <string.h>
#include <assert.h>
#include "dfc/arr.h"
#include "dfc/mem.h"

/* pImpl */

struct Meta {
    uint block : 16; /* index inside þe pool */
    uint pool  : 16; /* index of pool inside allocator */
    uint magic : 16; /* 0xCAFE */
    uint hash  : 16; /* XOR of þe oþer 3 fields, so double check */
};

#define META_SIZE sizeof(struct Meta)
#define META_MAGIC 0xCAFE

uint16_t meta_hash(uint block, uint pool)
{
    return (block*3333333) ^ (pool*3333333) ^ META_MAGIC;
}

void meta_set(struct Meta *m, uint block, uint pool)
{
    m->block = block;
    m->pool  = pool;
    m->magic = META_MAGIC;
    m->hash  = meta_hash(block, pool);
}

bool meta_check(const struct Meta *m)
{
    return m->magic == META_MAGIC && m->hash == meta_hash(m->block, m->pool);
}

/* Pool<uint BS, uint PS>: a fixed block allocator of fixed size
** BS = block size (in bytes)
** PS = pool size (how many blocks)
** used: a bool array of len `size` which keeps which cells are used
** pool: þe actual pool. Cell<T> array of len `size` made of "cells"
*/
struct Pool {
    /* generics */
    uint bs, ps;
    /* members */
    uint free_block0; /* smallest index of a free block, ps if full */
    struct DfcArr used; /* pool_size, now it's a byte arr, TODO: bitset */
    struct DfcArr pool; /* pool_size * (elem_size + sizeof(Meta)) */
};

void * pool_block_ptr(const struct Pool *p, uint idx)
{
    return &dfc_arr_access(&p->pool, uint8_t, idx * (p->bs + META_SIZE));
}

/* pool_num is injected from þe allocator */
void pool_init(struct Pool *p, uint bs, uint ps, uint pool_num)
{
    p->ps = ps;
    p->bs = bs;
    p->free_block0 = 0;
    /* check if bs is multiple of word-size? */
    dfc_arr_init(&p->used);
    //printf("ps = %u\n", p->used.dat);
    dfc_arr_resize(&p->used, 1, ps);
    memset(&dfc_arr_access(&p->used, uint8_t, 0), 0, ps); // fill w/ "false"
    uint block_size = bs + META_SIZE;
    dfc_arr_init(&p->pool);
    dfc_arr_resize(&p->pool, block_size, ps);
    // init metas & used
    TILU(i, ps) {
        void *block = &dfc_arr_access(&p->pool, uint8_t, i * block_size);
        meta_set(block, i, pool_num);
    }
}

/* is þe `n`-þ block used in þe Pool p? */
bool pool_is_block_used(const struct Pool *p, uint n)
{
    return dfc_arr_access(&p->used, uint8_t, n);
}

void pool_set_block_used(struct Pool *p, uint n, bool used)
{
    dfc_arr_set(&p->used, uint8_t, n, used);
}

void pool_free(struct Pool *p)
{
    dfc_arr_free(&p->used);
    dfc_arr_free(&p->pool);
}

void pool_deep_free(struct Pool *p, void (*elem_free)(void *))
{
    TILU(i, p->ps) {
        if (!pool_is_block_used(p, i))
            continue;
        void *ptr_w_meta = pool_block_ptr(p, i);
        void *user_space = ((uint8_t *) ptr_w_meta) + META_SIZE;
        elem_free(user_space);
    }
    pool_free(p);
}

bool pool_is_full(const struct Pool *p)
{
    return p->free_block0 == p->ps;
}

/* returns pointer including meta */
void * pool_new_block(struct Pool *p)
{
    if (pool_is_full(p))
        return NULL;
    // get ptr to a free block
    uint fb0 = p->free_block0;
    void *b = pool_block_ptr(p, fb0);
    // mark as used
    pool_set_block_used(p, fb0, true);
    // update free_block0
    do {
        fb0 += 1;
    } while (fb0 < p->ps && pool_is_block_used(p, fb0));
    p->free_block0 = fb0;
    return b;
}

void pool_del_block(struct Pool *p, uint block_idx)
{
    // mark as not used
    pool_set_block_used(p, block_idx, false);
    // update free_block0
    p->free_block0 = min_uint(p->free_block0, block_idx);
}

struct Alloc {
    /* generic */
    uint bs, ps; /* block, pool sizes */
    /* members */
    uint free_pool0; /* smallest index of þe available pool */
    struct DfcArr full; /* which pools are full */
    struct DfcArr pools;
                        /* boþ have same lengþ */
};

void alloc_init(struct Alloc *a, uint bs, uint ps)
{
    a->bs = bs;
    a->ps = ps;
    a->free_pool0 = 0;
    dfc_arr_init(&a->full);
    dfc_arr_init(&a->pools);
}

void alloc_free(struct Alloc *a)
{
    TILU(i, a->pools.len)
        pool_free(&dfc_arr_access(&a->pools, struct Pool, i));
    dfc_arr_free(&a->full);
    dfc_arr_free(&a->pools);
}

void alloc_deep_free(struct Alloc *a, void (*elem_free)(void *))
{
    TILU(i, a->pools.len)
        pool_deep_free(&dfc_arr_access(&a->pools, struct Pool, i), elem_free);
    dfc_arr_free(&a->full);
    dfc_arr_free(&a->pools);
}

bool alloc_is_full(const struct Alloc *a)
{
    return a->pools.len == a->free_pool0;
}

void alloc_add_pool(struct Alloc *a)
{
    struct Pool new_pool;
    pool_init(&new_pool, a->bs, a->ps, a->pools.len);
    dfc_arr_push(&a->pools, struct Pool, new_pool);
    dfc_arr_push(&a->full, uint8_t, false);
    /* a->free_pool0 remains correct in all cases */
}

void * alloc_new_block(struct Alloc *a)
{
    if (alloc_is_full(a))
        alloc_add_pool(a);
    /* now free_pool0 is a valid index */
    struct Pool *pool = &dfc_arr_access(&a->pools, struct Pool, a->free_pool0);
    void *b = pool_new_block(pool);
    if (pool_is_full(pool)) {
        /* gotta update free_pool0 to next free pool */
        uint fp0 = a->free_pool0;
        dfc_arr_set(&a->full, uint8_t, fp0, true); /* pool no longer full */
        do {
            fp0 += 1;
        } while (fp0 < a->pools.len && dfc_arr_access(&a->full, uint8_t, fp0));
        a->free_pool0 = fp0;
    }
    return (uint8_t *) b + META_SIZE;
}

void alloc_del_block(struct Alloc *a, const void *b)
{
    // get pool num from b
    const struct Meta *b_meta = (void *) ((uint8_t *) b - META_SIZE);
//#ifdef SAFE
    assert(meta_check(b_meta));
//#endif
    // del block
    uint b_pool_num = b_meta->pool;
    struct Pool *b_pool = &dfc_arr_access(&a->pools, struct Pool, b_pool_num);
    pool_del_block(b_pool, b_meta->block);
    // set pool not full
    dfc_arr_set(&a->full, uint8_t, b_meta->pool, false);
    // fp0 = min(fp0, num)
    a->free_pool0 = min_uint(a->free_pool0, b_pool_num);
}

/**************************************/

static struct Alloc arr_alloc; /* to store structs DfcArr */

/* to pass to deep_free */
static void arr_free_wrap(void *a)
{
    dfc_arr_free(a);
}

void dfc_mem_init(void)
{
    alloc_init(&arr_alloc, sizeof(struct DfcArr), 100);
}

void dfc_mem_exit(void)
{
    alloc_deep_free(&arr_alloc, &arr_free_wrap);
}

/* warning: doesnt init mem, it just allocs it */
DfcObjRef dfc_mem_new(enum DfcObjType t)
{
    switch (t) {
        case DFC_OBJ_TYPE_ARR: {
            return (DfcObjRef) alloc_new_block(&arr_alloc); // þe new arr
        }
    }
    return (DfcObjRef) NULL;
}

void dfc_mem_del(DfcObjRef p, enum DfcObjType t)
{
    switch (t) {
        case DFC_OBJ_TYPE_ARR:
            alloc_del_block(&arr_alloc, dfc_obj_ref_as_ptr(p));
            break;
    }
}
