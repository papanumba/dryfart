/* values.c */

#include <stdio.h>
#include "object.h" // includes values.h
#include "latin1.h"

void DfVal::print() const
{
    switch (this->type) {
      case VAL_V: putchar('V');                      break;
      case VAL_B: putchar(this->as.b ? 'T' : 'F');   break;
      case VAL_C: latin1_putchar(this->as.c);        break;
      case VAL_N: printf("%luu", (ulong)this->as.n); break;
      case VAL_Z: printf("%ld",   (long)this->as.z); break;
      case VAL_R: printf("%f", this->as.r);          break;
      case VAL_O: this->as.o.print();                break;
    }
}

DfType DfVal::as_type() const
{
    DfType t = DfType::V;
    switch (this->type) {
#define BASURA(X) case VAL_##X: t = DfType::X; break;
      BASURA(V)
      BASURA(B)
      BASURA(C)
      BASURA(N)
      BASURA(Z)
      BASURA(R)
#undef BASURA
      case VAL_O: t = objt2dft(this->as.o.get_type()); break;
    }
    return t;
}

bool DfVal::operator==(const DfVal &that) const
{
    if (this->type != that.type)
        return false;
    switch (this->type) {
      case VAL_V: return true;
      case VAL_B: return this->as.b == that.as.b;
      case VAL_C: return this->as.c == that.as.c;
      case VAL_N: return this->as.n == that.as.n;
      case VAL_Z: return this->as.z == that.as.z;
      case VAL_R: return this->as.r == that.as.r;
      case VAL_O: return this->as.o == that.as.o; // see objref.cpp
    }
    unreachable();
}

bool DfVal::operator!=(const DfVal &that) const
{
    if (this->type != that.type)
        return true;
    switch (this->type) {
      case VAL_V: return false;
      case VAL_B: return this->as.b != that.as.b;
      case VAL_C: return this->as.c != that.as.c;
      case VAL_N: return this->as.n != that.as.n;
      case VAL_Z: return this->as.z != that.as.z;
      case VAL_R: return this->as.r != that.as.r;
      case VAL_O: return this->as.o != that.as.o; // see objref.cpp
    }
    unreachable();
}

/* see C99's §6.5.8 Relational Operators ¶6 */

#define BASURA(M, m, c) \
case VAL_ ## M: return this->as.m c that.as.m;

#define DFVAL_CMP_FN(op) \
int DfVal::operator op (const DfVal &that) const \
{                                   \
    if (this->type != that.type)    \
        return CMP_ERR;             \
    switch (that.type) {            \
      BASURA(C, c, op)              \
      BASURA(N, n, op)              \
      BASURA(Z, z, op)              \
      BASURA(R, r, op)              \
      default: return CMP_ERR;      \
    }                               \
}

DFVAL_CMP_FN(<)
DFVAL_CMP_FN(<=)
DFVAL_CMP_FN(>)
DFVAL_CMP_FN(>=)

#undef BASURA
#undef DFVAL_CMP_FN
