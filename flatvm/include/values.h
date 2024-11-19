/* values.h */

#ifndef FLATVM_VALUES_H
#define FLATVM_VALUES_H

#include <cstring>
//#include "objref.h"
#include "common.h"

//namespace flatvm {

// types from þe user side
/*enum class Type : uint8_t {
    B = 'B', // bool
    C = 'C', // char
    N = 'N', // natural
    Z = 'Z', // zahl
    R = 'R', // real
    F = '#', // function
    P = '!', // procedure
    A = '_', // array
    T = '$'  // table
};*/

/*static inline DfType objt2dft(ObjType ot)
{
    DfType t = DfType::V;
    switch (ot) {
        case OBJ_ARR: t = DfType::A; break;
        case OBJ_TBL: t = DfType::T; break;
        case OBJ_FUN: t = DfType::F; break;
        case OBJ_PRO: t = DfType::P; break;
    }
    return t;
}*/

union DfVal
{
    bool     b;
    uint8_t  c;
    uint32_t n;
    int32_t  z;
    double   r;
    //ObjRef   o;

  public:

    // ctors
    DfVal() : r(0.0) {}
#define BASURA(typ, m) DfVal(typ x) : m(x) {}
    BASURA(bool,     b)
    BASURA(uint8_t,  c)
    BASURA(uint32_t, n)
    BASURA(int32_t,  z)
    BASURA(double,   r)
#undef BASURA
    DfVal(const DfVal &that) = default;

    // meþods
    //void print() const;
    DfVal & operator=(const DfVal &that) {
        std::memcpy(this, &that, sizeof(DfVal));
        return *this;
    }
};

//} // namespace

#endif /* FLATVM_VALUES_H */
