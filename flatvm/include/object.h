/* object.h */

#ifndef FLATVM_OBJECT_H
#define FLATVM_OBJECT_H

//#include <unordered_map>
#include "common.hpp"
#include "values.h"
//#include "htable.h"
#include "norris.h"
#include "native.h"
#include "dynarr.h"

class Object {
  public:
    bool gc_mark : 1;
    bool is_nat  : 1;
    uint pool_num : 14;
    Object() :
        gc_mark(false),
        is_nat (false) {}
    Object(Object &)  = default;
    Object(Object &&) = default;
    Object & operator=(Object &)  = default;
    Object & operator=(Object &&) = default;
};

enum class AccRes { // access result
    OK,
    OUT_OF_BOUNDS,
    DIFF_TYPE,
};

class ArrObj : public Object {
  public:
    DfType typ : 8; // V here means empty, þe default
  private:
    union _as {
        bool v; // dummy for ctor
        // TODO: B% bit array
        DynArr<uint8_t>  c;
        DynArr<uint32_t> n;
        DynArr<int32_t>  z;
        DynArr<float>    r;
        // dummy [cd]tors
        _as() : v{false} {}
        ~_as() {}
    } as;
  public:
    ArrObj() = default;
    ~ArrObj();
    ArrObj(DfVal &&); // array from single element, type inferred
    uint32_t len() const;
    bool push(DfVal &&);
    AccRes get(uint32_t, DfVal &) const;
    AccRes set(uint32_t, DfVal &&);
    void print() const;
    ArrObj & operator=(ArrObj &&from) { // move =
        Object::operator=(from);
        std::memcpy(&this->as, &from.as, sizeof(ArrObj::_as));
        return *this;
    }
};

// inject DfIdf hash to std namespace
/*template<>
struct std::hash<const DfIdf *> {
    std::size_t operator()(const DfIdf *&idf) const noexcept
    {
        return (size_t) idf->get_hash();
    }
};*/

class TblObj : public Object {
    typedef const DfIdf * key_t; // owned by VmData
    union {
        //std::unordered_map<key_t, DfVal, std::hash<key_t>> usr;
        NatTb  nat;
    } as;
  public:
    TblObj();
    TblObj(NatTb);
    bool set(key_t, DfVal &&);
    bool get(key_t, DfVal &); // returns by last par
    void print() const;
};

class ProObj : public Object {
    union {
        Norris *usr;/* FUTURE: eke upvalues */
        struct NatPc nat;
    } as;
  public:
    ProObj();
    ProObj(NatPc);
    void print() const;
};

class FunObj : public Object {
    union {
        Norris *usr; /* FUTURE: eke upvalues */
        struct NatFn nat;
    } as;
  public:
    FunObj();
    FunObj(NatFn);
    void print() const;
};

#endif /* FLATVM_OBJECT_H */
