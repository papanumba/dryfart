/* falloc.c */

#include <stdio.h>
#include <stdlib.h>
#include "falloc.h"


#define BLOCKS_PER_POOL 64

struct Block {
    int free;
    union {
        struct Block *next; /* free list */
        objs_u user_space;
    } val;
};

#define GET_BLOCK_FROM_US(us) \
    ((struct Block *) (((uint8_t *)(us)) - offsetof(struct Block, val)))

struct Pool {
    struct Pool *next;
    struct Block blocks[BLOCKS_PER_POOL];
};

static struct Pool  *frst_pool = NULL;
static struct Pool  *last_pool = NULL;
static struct Block *frst_free_block = NULL;    /* free list */
static struct Block *last_free_block = NULL;
static size_t pool_count = 0;
static size_t objs_count = 0;

static int append_new_pool(void);
static inline void init_pool(struct Pool *);

void * falloc_alloc(void)
{
    if (frst_free_block == NULL) { /* empty list */
        if (!append_new_pool())
            return NULL; /* couldn't malloc pool */
    }
    struct Block *b = frst_free_block;
    frst_free_block = frst_free_block->val.next;
    b->free = FALSE;
    objs_count++;
    return &b->val.user_space;
}

/* pointer passed is user space */
void falloc_free(void *us)
{
    struct Block *b = GET_BLOCK_FROM_US(us);
    b->free = TRUE;
    b->val.next = frst_free_block;
    frst_free_block = b;
    objs_count--;
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

size_t falloc_objs_num(void)
{
    return objs_count;
}

void falloc_sweep(void)
{
    for (struct Pool *p = frst_pool; p != NULL; p = p->next) {
        for (size_t i = 0; i < BLOCKS_PER_POOL; ++i) {
            struct Block *b = &p->blocks[i];
            if (b->free)
                continue;
            struct Object *o = &b->val.user_space.o;
            if (!o->gc_mark) {
#ifdef DEBUG
                fprintf(stderr, "freeeing object: ");
                object_print(o);
                fputs("\n", stderr);
#endif
                object_free(o);
            } else {
                o->gc_mark = FALSE;
            }
        }
    }
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
    struct Block *pools_frst = &new_p->blocks[0];
    struct Block *pools_last = &new_p->blocks[BLOCKS_PER_POOL-1];
    if (frst_free_block == NULL) {
        frst_free_block = pools_frst;
        last_free_block = pools_last;
    } else {
        last_free_block->val.next = pools_frst;
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
        p->blocks[i].free = TRUE;
        p->blocks[i].val.next = &p->blocks[i+1];
    }
    /* set last to null */
    p->blocks[BLOCKS_PER_POOL-1].val.next = NULL;
    p->next = NULL;
}
