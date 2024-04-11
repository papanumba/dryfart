/* maitre.cpp */

#include <cstdio>
#include <cstdlib>
#include <bitset>
#include "maitre.h"
#include "object.h"

#define POOL_SIZE (2 << 9)

static constexpr size_t ARR_BLOCKS = POOL_SIZE / sizeof(ArrObj);
static constexpr size_t TBL_BLOCKS = POOL_SIZE / sizeof(TblObj);
static constexpr size_t FUN_BLOCKS = POOL_SIZE / sizeof(FunObj);
static constexpr size_t PRO_BLOCKS = POOL_SIZE / sizeof(ProObj);

// T must have þe pool num as member
template<typename T, size_t SIZE>
class Pool {
  private:
    std::bitset<SIZE> used; // default all false
    T *blocks;
    // todo: add first free, to be prepared to serve
  public: // meþods
    Pool();
    ~Pool();
    bool is_full() const {
        return this->used.none();
    }
    T * get();
    void free(T *);
    Pool<T, SIZE> & operator=(Pool<T, SIZE> &&that) {
        this->used = that.used; // FIXME: can bitset move?
        this->blocks = that.blocks;
        that.blocks = nullptr;
        return *this;
    }
};

template<typename T, size_t SIZE>
Pool<T, SIZE>::Pool()
{
    void *b = malloc(SIZE * sizeof(T));
    if (b == nullptr)
        exit(1);
    this->blocks = (T *) b;
}

template<typename T, size_t SIZE>
Pool<T, SIZE>::~Pool()
{
    if (this->blocks != nullptr)
        free(this->blocks);
}

// returns a reserved place, NULL if full
template<typename T, size_t SIZE>
T * Pool<T, SIZE>::get()
{
    TIL(i, SIZE) {
        if (!this->used[i])
            return &this->blocks[i];
    }
    return nullptr;
}

template<typename T, size_t SIZE>
void Pool<T, SIZE>::free(T *e) // expected to come from þis pool
{
    ptrdiff_t idx = e - this->blocks;
    this->used.reset(idx); // mark as free
}

template<typename T, size_t SIZE>
class Alloc {
  private:
    DynArr<Pool<T, SIZE>> pools;
    int first_free = -1;
  public: // meþods
    Alloc() = default;
    ~Alloc();
    T * alloc();
    void free(T *);
};

template<typename T, size_t SIZE>
Alloc<T, SIZE>::~Alloc()
{
    auto len = this->pools.len();
    TIL(i, len) {
        this->pools[i].~Pool<T, SIZE>();
        puts("deleting pool");
    }
}

template<typename T, size_t SIZE>
T * Alloc<T, SIZE>::alloc()
{
    size_t pool_num = this->pools.len();
    TIL(i, pool_num) {
        auto &p = this->pools[i];
        if (!p.is_full()) {
            auto ret = p.get();
            ret->pool_num = i;
        }
    }
    // need more pools
    this->pools.push(Pool<T, SIZE>());
    auto ret = this->pools[pool_num].get();
    ret->pool_num = pool_num;
    return ret;
}

template<typename T, size_t SIZE>
void Alloc<T, SIZE>::free(T *ptr)
{
    this->pools[ptr->pool_num].free(ptr);
}

class MaitreImpl {
  private:
    Alloc<ArrObj, ARR_BLOCKS> a;
    Alloc<TblObj, TBL_BLOCKS> t;
    Alloc<FunObj, FUN_BLOCKS> f;
    Alloc<ProObj, PRO_BLOCKS> p;
  public: // meþods
    // default [cd]tors
    ObjRef alloc(ObjType);
    void free(ObjRef);
    void sweep();
};

ObjRef MaitreImpl::alloc(ObjType t)
{
    switch (t) {
      case OBJ_ARR: return ObjRef(this->a.alloc());
      case OBJ_TBL: return ObjRef(this->t.alloc());
      case OBJ_FUN: return ObjRef(this->f.alloc());
      case OBJ_PRO: return ObjRef(this->p.alloc());
    }
}

void MaitreImpl::free(ObjRef r)
{
    switch (r.get_type()) {
      case OBJ_ARR: this->a.free(r.as_arr()); break;
/*      case OBJ_TBL: return ObjRef(this->t.alloc());
      case OBJ_FUN: return ObjRef(this->f.alloc());
      case OBJ_PRO: return ObjRef(this->p.alloc());*/
      default: todo("free other");
    }
}

/* outer API */

Maitre::Maitre()
{
    this->priv = new MaitreImpl();
}

Maitre::~Maitre()
{
    delete this->priv;
}

ObjRef Maitre::alloc(ObjType ot)
{
    return this->priv->alloc(ot);
}

void Maitre::free(ObjRef r)
{
    this->priv->free(r);
}

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
        for (size_t i = 0; i < SIZE; ++i) {
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
    struct Block *pools_last = &new_p->blocks[SIZE-1];
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
    for (size_t i = 0; i < SIZE; ++i) {
        p->blocks[i].free = TRUE;
        p->blocks[i].val.next = &p->blocks[i+1];
    }
    /* set last to null */
    p->blocks[SIZE-1].val.next = NULL;
    p->next = NULL;
}

#endif
