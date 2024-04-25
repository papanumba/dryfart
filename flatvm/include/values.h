/* values.h */

#ifndef FLATVM_VALUES_H
#define FLATVM_VALUES_H

#include <cstring>
#include "common.hpp"
#include "objref.h"

// types from þe user side
enum class DfType : uint8_t {
    V = 'V', /* void */
    B = 'B', /* bool */
    C = 'C', /* char */
    N = 'N', /* natural */
    Z = 'Z', /* zahl */
    R = 'R', /* real */
    F = '#', /* function */
    P = '!', /* procedure */
    A = '_', /* array */
    T = '$'  /* table */
};

static inline DfType objt2dft(ObjType ot)
{
    DfType t = DfType::V;
    switch (ot) {
        case OBJ_ARR: t = DfType::A; break;
        case OBJ_TBL: t = DfType::T; break;
        case OBJ_FUN: t = DfType::F; break;
        case OBJ_PRO: t = DfType::P; break;
    }
    return t;
}

enum ValType {
    VAL_V = 0x00,
    VAL_B = 0x02,
    VAL_C = 0x04,
    VAL_N = 0x06,
    VAL_Z = 0x08,
    VAL_R = 0x0A,
    VAL_O = 0x0C /* any heap stuff */
};

class DfVal {
  public:
    enum ValType type;
    union _as {
        bool     b;
        uint8_t  c;
        uint32_t n;
        int32_t  z;
        float    r;
        ObjRef   o;
    } as;
  public:
    DfVal() : type(VAL_V) { }
    DfVal(const DfVal &that) {
        this->type = that.type;
        this->as.o = that.as.o; // largest member
    }
#define BASURA(typ, m, M) \
    DfVal(typ m) :        \
        type(VAL_ ## M) { \
        this->as.m = m;   \
    }
    BASURA(bool,     b, B)
    BASURA(uint8_t,  c, C)
    BASURA(uint32_t, n, N)
    BASURA(int32_t,  z, Z)
    BASURA(float,    r, R)
    BASURA(ObjRef,   o, O)
#undef BASURA
    // meþods
    void set_mut(bool m = true) {
        if (this->type == VAL_O)
            this->as.o.set_mut(m);
    }
    void print() const;
    DfType as_type() const;
#define BASURA(X, x) \
    bool is_ ## x() const { \
        return this->type == VAL_O && \
            this->as.o.get_type() == OBJ_ ## X; \
    }
    BASURA(ARR, arr)
    BASURA(TBL, tbl)
    BASURA(FUN, fun)
    BASURA(PRO, pro)
#undef BASURA
    // operators
    bool operator==(const DfVal &) const;
    bool operator!=(const DfVal &) const;
    DfVal & operator=(const DfVal &that) {
        this->type = that.type;
        this->as.o = that.as.o;
        return *this;
    }
};

#endif /* FLATVM_VALUES_H */
