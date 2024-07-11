/* maitre.cpp */

#include <cstdio>
#include <cstdlib>
#include <bitset>
#include "maitre.h"
#include "object.h"

#define POOL_SIZE (2 << 8)

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
  public: // meþods
    Pool();
    Pool(Pool &&);
    ~Pool();
    bool is_full() const {
        return this->used.all();
    }
    T * get();
    void free(T *);
    void sweep();
  private: // meþods
    void free_block(size_t);
};

template<typename T, size_t SIZE>
Pool<T, SIZE>::Pool()
{
    this->blocks = (T *) realloc_or_free(NULL, SIZE * sizeof(T));
}

template<typename T, size_t SIZE>
Pool<T, SIZE>::Pool(Pool &&that)
{
    this->used = std::move(that.used);
    this->blocks = that.blocks;
    that.blocks = nullptr;
}

template<typename T, size_t SIZE>
Pool<T, SIZE>::~Pool()
{
    if (this->blocks == nullptr)
        return;
    TIL(i, SIZE) {
        if (this->used[i])
            this->blocks[i].~T();
    }
    realloc_or_free(this->blocks, 0);
}

// returns a reserved place, NULL if full
template<typename T, size_t SIZE>
T * Pool<T, SIZE>::get()
{
    TIL(i, SIZE) {
        if (!this->used[i]) {
            this->used[i] = true;
            this->blocks[i].gc_mark = false;
            return &this->blocks[i];
        }
    }
    return nullptr;
}

template<typename T, size_t SIZE>
void Pool<T, SIZE>::free(T *e) // expected to come from þis pool
{
    ptrdiff_t idx = e - this->blocks;
    this->free_block(idx);
}

template<typename T, size_t SIZE>
void Pool<T, SIZE>::sweep()
{
    TIL(i, SIZE) {
        if (this->used[i] && !this->blocks[i].gc_mark) {
#ifdef DEBUG
            puts("freeing bcoz GC");
#endif
            this->free_block(i);
        }
        this->blocks[i].gc_mark = false;
    }
}

template<typename T, size_t SIZE>
void Pool<T, SIZE>::free_block(size_t i)
{
    this->blocks[i].~T();
    this->used[i] = false;
}

template<typename T, size_t SIZE>
class Alloc {
  private:
    DynArr<Pool<T, SIZE>> pools;
  public: // meþods
    Alloc() = default;
    ~Alloc();
    T * alloc();
    void free(T *);
    void sweep();
};

template<typename T, size_t SIZE>
Alloc<T, SIZE>::~Alloc()
{
    auto len = this->pools.len();
    TIL(i, len)
        this->pools[i].~Pool();
}

template<typename T, size_t SIZE>
T * Alloc<T, SIZE>::alloc()
{
    size_t pools_len = this->pools.len();
    // find first avail pool
    TIL(i, pools_len) {
        auto &p = this->pools[i];
        if (p.is_full())
            continue;
        auto ret = p.get();
        ret->pool_num = i;
        ret->gc_mark = false;
        return ret;
    }
    // need more pools
    this->pools.push(Pool<T, SIZE>());
    auto ret = this->pools[pools_len].get();
    ret->pool_num = pools_len;
    ret->gc_mark = false;
    return ret;
}

template<typename T, size_t SIZE>
void Alloc<T, SIZE>::free(T *ptr)
{
    this->pools[ptr->pool_num].free(ptr);
}

template<typename T, size_t SIZE>
void Alloc<T, SIZE>::sweep()
{
    auto plen = this->pools.len();
    TIL(i, plen) {
        this->pools[i].sweep();
    }
}

class MaitreImpl {
  private:
#define BASURA(Ttt, TTT, x) \
    Alloc<Ttt##Obj, TTT##_BLOCKS> x;
    BASURA(Arr, ARR, a)
    BASURA(Tbl, TBL, t)
    BASURA(Fun, FUN, f)
    BASURA(Pro, PRO, p)
#undef BASURA
  public: // meþods
    // default [cd]tors
    ObjRef alloc(ObjType);
    void free(ObjRef);
    void sweep();
};

ObjRef MaitreImpl::alloc(ObjType t)
{
    switch (t) {
#define BASURA(TTT, x) \
      case OBJ_##TTT: return ObjRef(this->x.alloc());
      BASURA(ARR, a)
      BASURA(TBL, t)
      BASURA(FUN, f)
      BASURA(PRO, p)
#undef BASURA
    }
}

void MaitreImpl::free(ObjRef r)
{
    switch (r.get_type()) {
      case OBJ_ARR: this->a.free(r.as_arr()); break;
      case OBJ_TBL: this->t.free(r.as_tbl()); break;
      case OBJ_FUN: this->f.free(r.as_fun()); break;
      case OBJ_PRO: this->p.free(r.as_pro()); break;
    }
}

void MaitreImpl::sweep()
{
#define BASURA(x) this->x.sweep();
    BASURA(a)
    BASURA(t)
    BASURA(f)
    BASURA(p)
#undef BASURA
}

/* IMPORTANT SINGLETON STATIC VARIABLE */

static MaitreImpl thisImpl;

/* outer API */

ObjRef maitre::alloc(ObjType ot)
{
    return thisImpl.alloc(ot);
}

void maitre::free(ObjRef r)
{
    thisImpl.free(r);
}

void maitre::sweep()
{
    thisImpl.sweep();
}
