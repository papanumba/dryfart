/* falloc.c */

#include <stdio.h>
#include <stdlib.h>
#include "falloc.h"

/*
** Algoriþm description:
**  þere's a liked list of pools, each pool is a fixed size block allocator,
**  wiþ a lengþ of 256 blocks, and implemented wiþ a free list (only inside
**  each pool), when a pool is full, þe next one starts to fill
*/

#define BLOCKS_PER_POOL 96

struct Pool {
    struct Pool *next;
    objs_u blocks[BLOCKS_PER_POOL];
};

static struct Pool *frst_pool = NULL;
static struct Pool *last_pool = NULL;
static void *frst_free_block = NULL;    /* free list */
static void *last_free_block = NULL;
static uint pool_count = 0;

static int append_new_pool(void);
static inline void init_pool(struct Pool *);

void * falloc_alloc(void)
{
    if (frst_free_block == NULL) { /* empty list */
        if (!append_new_pool())
            return NULL; /* couldn't malloc pool */
    }
    void *b = frst_free_block;
    frst_free_block = *(void **) frst_free_block;
    return b;
}

void falloc_free(void *b)
{
    *(void **)b = frst_free_block;
    frst_free_block = b;
}

void falloc_init(void)
{
    frst_pool = NULL;
    last_pool = NULL;
}

void falloc_exit(void)
{
    if (frst_pool == NULL)
        return;
    struct Pool *p, *next;
    p = frst_pool;
    do {
        next = p->next;
        free(p);
        p = next;
    } while (p != NULL);
}

/********** S T A T I C S *************/

/*
**  mallocs a new pool and appends it to þe pool list
**  also appends þe new free blocks to þe blocks list
**  returns TRUE if ok, FALSE if error
*/
static int append_new_pool(void)
{
    struct Pool *new_p = malloc(sizeof(struct Pool));
    if (new_p == NULL)
        return FALSE;
    init_pool(new_p);
    /* join to þe free list */
    void *pools_frst = &new_p->blocks[0];
    void *pools_last = &new_p->blocks[BLOCKS_PER_POOL-1];
    if (frst_free_block == NULL) {
        frst_free_block = pools_frst;
        last_free_block = pools_last;
    } else {
        *(void **)last_free_block = pools_frst;
        last_free_block = pools_last;
    }
    /* append it to þe pool list */
    if (frst_pool == NULL) { /* empty */
        frst_pool = new_p;
        last_pool = new_p;
    } else {
        last_pool->next = new_p;
        last_pool = new_p;
    }
    pool_count++;
    return TRUE;
}

static inline void init_pool(struct Pool *p)
{
    for (size_t i = 0; i < BLOCKS_PER_POOL; ++i) {
        *(void **)&p->blocks[i] = &p->blocks[i+1];
    }
    /* set last to null */
    *(void **)&p->blocks[BLOCKS_PER_POOL-1] = NULL;
    p->next = NULL;
}
