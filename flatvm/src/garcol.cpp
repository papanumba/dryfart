// garcol.cpp

#include <cstdio>
#include <cstdlib>
#include "garcol.h"
#include "virmac.h"
#include "object.h"

using namespace garcol;

static DynArr<ObjRef> grey_stack;

static void mark_roots(VirMac *);
static void mark(Htable *);
static void mark(TblObj *);
static void mark(DynArr<DfVal> &);
static void trace_refs();
static void blacken_obj(ObjRef &);

void mark_dfval(DfVal &v)
{
    if (v.type != VAL_O)
        return;
    mark_object(v.as.o);
}

// mark as "grey", i.e. push to grey & set mark true
void mark_object(ObjRef &obj)
{
    if (obj.get_gc_mark())
        return;
    obj.set_gc_mark(true);
    grey_stack.push(ObjRef(obj));
}

void garcol::do_it(VirMac *vm)
{
#ifdef DEBUG
    puts("garcol starts");
#endif
    mark_roots(vm);
    trace_refs();
    maitre::sweep();
#ifdef DEBUG
    puts("garcol ends");
#endif
}

// static stuff --------------------------------------

/* marks globals & stack */
static void mark_roots(VirMac *vm)
{
    TIL(i, STACK_MAX)
        mark_dfval(vm->stack[i]);
}

static void mark(TblObj *t)
{
    if (!t->is_nat)
        mark(&t->as.usr);
    // TODO: future non singleton nat tbls
}

static void mark(Htable *t)
{
    for (auto e = t->begin(); e != t->end(); t->next(e))
        mark_dfval(e.val());
}

static void mark(DynArr<DfVal> &a)
{
    TIL(i, a.len())
        mark_dfval(a[i]);
}

static void trace_refs(void)
{
    while (!grey_stack.is_empty()) {
        auto o = grey_stack.pop();
        blacken_obj(o);
    }
}

/* mark neibrhood */
static void blacken_obj(ObjRef &obj)
{
    switch (obj.typ()) {
      case OBJ_ARR:
        return;
      case OBJ_TBL:
        mark(obj.as_tbl());
        return;
      // for subroutines, mark upvalues
#define BASURA(TTT, ttt) \
      case OBJ_##TTT: {           \
        auto *x = obj.as_##ttt(); \
        if (!x->is_nat)           \
            mark(x->as.usr.upv);  \
        break;                    \
      }
      BASURA(FUN, fun)
      BASURA(PRO, pro)
#undef BASURA
    }
}
