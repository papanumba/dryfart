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

#define BASURA_CASES \
      BASURA(ARR, arr) \
      BASURA(TBL, tbl) \
      BASURA(FUN, fun) \
      BASURA(PRO, pro)

bool ObjRef::get_gc_mark() const
{
    switch (this->typ()) {
#define BASURA(TTT, ttt) case OBJ_##TTT: return this->as_##ttt()->gc_mark;
      BASURA_CASES
#undef BASURA
    }
}

void ObjRef::set_gc_mark(bool m = true)
{
    switch (this->typ()) {
#define BASURA(TTT, ttt) case OBJ_##TTT: this->as_##ttt()->gc_mark = m; break;
      BASURA_CASES
#undef BASURA
    }
}

void ObjRef::print() const
{
    switch (this->typ()) {
#define BASURA(TTT, ttt) case OBJ_##TTT: this->as_##ttt()->print(); break;
      BASURA_CASES
#undef BASURA
    }
}

#undef BASURA_CASES
