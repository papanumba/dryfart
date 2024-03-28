/* maitre.cpp */

#include <cstdio>
#include <cstdlib>
#include <bitset>
#include "maitre.h"
#include "object.h"

#define BLOCKS_PER_POOL 32

template<typename T>
class Pool {
  private:
    std::bitset<BLOCKS_PER_POOL> used; // default all false
    T *blocks;
    // todo: add first free, to be prepared to serve
  public: // meþods
    Pool();
    ~Pool() {
        free(this->blocks);
    }
    bool is_full() const {
        return this->used.none();
    }
    T * get();
    bool try_free(T *);
};

template<typename T>
Pool<T>::Pool()
{
    void *b = aligned_alloc(8, BLOCKS_PER_POOL * sizeof(T));
    if (b == nullptr)
        exit(1);
    this->blocks = (T *) b;
}

// returns a reserved place, NULL if full
template<typename T>
T * Pool<T>::get()
{
    TIL(i, BLOCKS_PER_POOL) {
        if (!this->used[i])
            return &this->blocks[i];
    }
    return NULL;
}

// returns true if þe pointer comes from þis pool
// else returns false
template<typename T>
bool Pool<T>::try_free(T *e)
{
    ptrdiff_t idx = e - &this->blocks[0];
    bool is_from_this = (0 <= idx && idx <= BLOCKS_PER_POOL);
    if (is_from_this)
        this->used.reset(idx); // mark as free
    return is_from_this;
}

template<typename T>
class Alloc {
  private:
    DynArr<Pool<T>> pools;
    int first_free = -1;
  public: // meþods
    Alloc() = default;
    ~Alloc();
    T * alloc();
    void free(T *);
};

template<typename T>
Alloc<T>::~Alloc()
{
    auto len = this->pools.len();
    TIL(i, len)
        delete &this->pools[i];
}

template<typename T>
T * Alloc<T>::alloc()
{
    size_t pool_num = this->pools.len();
    TIL(i, pool_num) {
        auto &p = this->pools[i];
        if (!p.is_full())
            return p.get();
    }
    // need more pools
    this->pools.push(Pool<T>());
    return this->pools[pool_num].get();
}

template<typename T>
void Alloc<T>::free(T *ptr)
{
    size_t pool_num = this->pools.len();
    TIL(i, pool_num) {
        if (this->pools[i].try_free(ptr))
            return; // found þe pool it came from
    }
    panic("free wrong ptr");
}

class MaitrePriv {
  private:
    Alloc<ArrObj> a;
    Alloc<TblObj> t;
    Alloc<FunObj> f;
    Alloc<ProObj> p;
  public: // meþods
    // default [cd]tors
    ObjRef alloc(ObjType);
    void free(ObjRef);
    void sweep();
};

ObjRef MaitrePriv::alloc(ObjType t)
{
    switch (t) {
      case OBJ_ARR: return ObjRef(this->a.alloc());
      case OBJ_TBL: return ObjRef(this->t.alloc());
      case OBJ_FUN: return ObjRef(this->f.alloc());
      case OBJ_PRO: return ObjRef(this->p.alloc());
    }
}

/* outer API */

Maitre::Maitre()
{
    this->priv = (void *) new MaitrePriv();
}

#define THIS_PRIV ((MaitrePriv *) (this->priv))

Maitre::~Maitre()
{
    delete THIS_PRIV;
}

ObjRef Maitre::alloc(ObjType ot)
{
    return THIS_PRIV->alloc(ot);
}

void Maitre::free(ObjRef r)
{
    THIS_PRIV->free(r);
}

#undef THIS_PRIV

#ifdef GRANMERDA
// test

void merda() {
    Alloc<ArrObj> x;
    auto y = x.alloc();
}

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

#endif
