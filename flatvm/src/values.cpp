/* values.c */

#include <stdio.h>
#include "object.h" // includes values.h

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
      case VAL_R: return false; // must use an ε
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
      case VAL_R: return true; // must use an ε
      case VAL_O: return this->as.o != that.as.o; // see objref.cpp
    }
    unreachable();
}

void DfVal::print() const
{
    switch (this->type) {
      case VAL_V: putchar('V');                      break;
      case VAL_B: putchar(this->as.b ? 'T' : 'F');   break;
      case VAL_C: putchar(this->as.c);               break;
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
        case VAL_V: t = DfType::V; break;
        case VAL_B: t = DfType::B; break;
        case VAL_C: t = DfType::C; break;
        case VAL_N: t = DfType::N; break;
        case VAL_Z: t = DfType::Z; break;
        case VAL_R: t = DfType::R; break;
        case VAL_O: t = objt2dft(this->as.o.get_type()); break;
    }
    return t;
}
