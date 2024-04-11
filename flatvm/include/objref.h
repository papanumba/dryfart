/* objref.h */

#ifndef FLATVM_OBJREF_H
#define FLATVM_OBJREF_H

#include "common.hpp"

// fits in 2 bits
enum ObjType {
    OBJ_ARR = 0,
    OBJ_TBL = 1,
    OBJ_FUN = 2,
    OBJ_PRO = 3,
};

// to avoid cyclic dep w/ "object.h"
class Object; // base
class ArrObj;
class TblObj;
class FunObj;
class ProObj;

class ObjRef {
  private:
    // tagged pointer on last 3 bits
    // 1st 2 are ObjType (see object.h)
    // last 1 is mut
    uintptr_t p;
    // masks
    static const uintptr_t PTR_MASK = ~7; // ~0b111
    static const uintptr_t MUT_MASK = 4;  //  0b100
    static const uintptr_t TYP_MASK = 3;  //  0b011
    // TODO add mut
    // private methods
    void * ptr() const {
        return (void *) (this->p & PTR_MASK);
    }
    uint typ() const {
        return this->p & TYP_MASK;
    }
    bool mut() const {
        return this->p & MUT_MASK;
    }
  public:
    ObjRef() = default;
    ObjRef(ObjRef &)  = default;
    ObjRef(ObjRef &&) = default;
    // meþods
    ObjType get_type() const {
        return (ObjType) this->typ();
    }
    void print() const;
#define BASURA(Typ, fn) \
    Typ ## Obj * as_ ## fn() const {        \
        return (Typ ## Obj *) this->ptr();  \
    }
    BASURA(Arr, arr)
    BASURA(Tbl, tbl)
    BASURA(Fun, fun)
    BASURA(Pro, pro)
#undef BASURA
    bool operator==(const ObjRef &) const;
    bool operator!=(const ObjRef &) const;
    // creates new mutable ObjRef with type based on þe pointer's type
    // expected 8-bit aligned
#define BASURA(Typ, TYP) \
    ObjRef(Typ ## Obj *r) : p((uintptr_t) r | MUT_MASK | OBJ_ ## TYP) {}
    BASURA(Arr, ARR)
    BASURA(Tbl, TBL)
    BASURA(Fun, FUN)
    BASURA(Pro, PRO)
#undef BASURA
    // move =
    ObjRef & operator=(ObjRef &)  = default;
    ObjRef & operator=(const ObjRef &) = default;
    ObjRef & operator=(ObjRef &&) = default;
};

#endif // FLATVM_OBJREF_H
