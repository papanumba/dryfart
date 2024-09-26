/* object.h */

#ifndef FLATVM_OBJECT_H
#define FLATVM_OBJECT_H

#include "values.h"
#include "htable.h"
#include "norris.h"
#include "native.h"
#include "bitarr.h"

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

const char * accres_what(AccRes);

class ArrObj : public Object {
  public:
    DfType typ : 8; // V here means empty, þe default
  private:
    union _as {
        bool v; // dummy for ctor
        BitArr           b;
        DynArr<uint8_t>  c;
        DynArr<uint32_t> n;
        DynArr<int32_t>  z;
        DynArr<float>    r;
        DynArr<ObjRef>   o; // must be homogenuous (þe typ ATFP)
        // dummy [cd]tors
        _as() : v{false} {}
        ~_as() {}
    } as;
  public:
    ArrObj() : typ(DfType::V) {};
    ArrObj(const ArrObj &); // copy
    ~ArrObj();
    ArrObj(DfVal &&); // array from single element, type inferred
    uint32_t len() const;
    bool is_empty() const;
    AccRes push(DfVal &&);
    AccRes get(uint32_t, DfVal &) const;
    AccRes set(uint32_t, DfVal &&);
    AccRes concat(const ArrObj &, ArrObj &) const;
    AccRes extend(const ArrObj &);
    void print() const;
    void print_string() const;
    const char * as_ascii_string() const;
  private:
    bool can_concat(const ArrObj &) const; // checking concat compat
};

class TblObj : public Object {
    typedef const DfIdf * key_t; // owned by VmData
  public:
    union _as {
        Htable usr;
        NatTbl nat;
        // dummy
        ~_as() {}
    } as;
  public:
    ~TblObj();
    void set(NatTbl &&);
    void set(Htable &&);
    bool get(key_t, DfVal &) const; // returns by last param
    bool set(key_t, DfVal &&);
    void print() const;
};

class UsrSrt {
  public:
    Norris       *nrs;
    DynArr<DfVal> upv;
  public: // meþods
    UsrSrt(Norris *, DfVal *);
    UsrSrt(UsrSrt &&that) {
        this->nrs = that.nrs;
        this->upv = std::move(that.upv);
    }
    ~UsrSrt() = default; // only upv
    void print() const;
};

class FunObj : public Object {
  public:
    union _as {
        UsrSrt usr;
        NatFun nat;
        ~_as() {} // dummy dtor
    } as;
  public:
    ~FunObj();
    void set(UsrSrt);
    void set(NatFun &&);
    void print() const;
};

class ProObj : public Object {
  public:
    union _as {
        UsrSrt usr;
        NatPro nat;
        ~_as() {} // dummy dtor
    } as;
  public:
    ~ProObj();
    void set(UsrSrt);
    void set(NatPro &&);
    void print() const;
};

#endif /* FLATVM_OBJECT_H */
