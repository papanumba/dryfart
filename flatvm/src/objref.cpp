// objref.cpp

#include "objref.h"
#include "object.h"

bool ObjRef::operator==(const ObjRef &that) const
{
    // ignore mut when comparing
    return (this->p & ~1) == (that.p & ~1);
}

bool ObjRef::operator!=(const ObjRef &that) const
{
    return !(*this == that);
}

void ObjRef::print() const
{
    switch (this->typ()) {
      case OBJ_ARR: this->as_arr()->print(); break;
      case OBJ_TBL: this->as_tbl()->print(); break;
      case OBJ_FUN: this->as_fun()->print(); break;
      case OBJ_PRO: this->as_pro()->print(); break;
    }
}
