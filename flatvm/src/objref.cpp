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

bool ObjRef::get_gc_mark() const
{
    switch (this->typ()) {
#define BASURA(TTT, ttt) \
      case OBJ_##TTT: return this->as_##ttt()->gc_mark;
      BASURA(ARR, arr)
      BASURA(TBL, tbl)
      BASURA(FUN, fun)
      BASURA(PRO, pro)
#undef BASURA
    }
}

void ObjRef::set_gc_mark(bool m = true)
{
    switch (this->typ()) {
#define BASURA(TTT, ttt) \
      case OBJ_##TTT: this->as_##ttt()->gc_mark = m; break;
      BASURA(ARR, arr)
      BASURA(TBL, tbl)
      BASURA(FUN, fun)
      BASURA(PRO, pro)
#undef BASURA
    }
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
