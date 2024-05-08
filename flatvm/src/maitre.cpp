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
  public: // meþods
    Pool();
    Pool(Pool &&);
    ~Pool();
    bool is_full() const {
        return this->used.all();
    }
    T * get();
    void free(T *);
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
    free(this->blocks);
}

// returns a reserved place, NULL if full
template<typename T, size_t SIZE>
T * Pool<T, SIZE>::get()
{
    TIL(i, SIZE) {
        if (!this->used[i]) {
            this->used[i] = true;
            return &this->blocks[i];
        }
    }
    return nullptr;
}

template<typename T, size_t SIZE>
void Pool<T, SIZE>::free(T *e) // expected to come from þis pool
{
    ptrdiff_t idx = e - this->blocks;
    this->used[idx] = false;
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
        if (!p.is_full()) {
            auto ret = p.get();
            ret->pool_num = i;
            return ret;
        }
    }
    // need more pools
    this->pools.push(Pool<T, SIZE>());
    auto ret = this->pools[pools_len].get();
    ret->pool_num = pools_len;
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
