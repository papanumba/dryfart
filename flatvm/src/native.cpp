// native.cpp

#include <cstdio>
#include <cstring>
#include <utility>
#include "virmac.h"
#include "native.h"
#include "object.h"
#include "garcol.h"
#include "maitre.h"

#define FAC_MET(Ttt, ttt, TTT) \
\
static DynArr<Nat##Ttt##Tag> ttt##tags;  \
static DynArr<ObjRef>        ttt##refs;  \
\
ObjRef NatFactory::get(Nat##Ttt##Tag tag)   \
{                                           \
    auto &tags = ttt##tags;                 \
    auto &refs = ttt##refs;                 \
    TIL(i, tags.len()) {                    \
        if (tags[i] == tag)                 \
            return refs[i];                 \
    }                                       \
    auto new_obj = maitre::alloc(OBJ_##TTT);\
    auto p = new_obj.as_ ## ttt();          \
    p->gc_mark = true;                      \
    auto tmp = Nat##Ttt(tag);               \
    p->set(std::move(tmp));                 \
    tags.push(std::move(tag));              \
    refs.push(std::move(new_obj));          \
    return new_obj;                         \
}

//FAC_MET(Arr, arr, ARR)
FAC_MET(Tbl, tbl, TBL)
FAC_MET(Fun, fun, FUN)
FAC_MET(Pro, pro, PRO)

#undef FAC_MET

void NatFactory::mark_all()
{
#define BASURA(ttt) \
    TIL(i, ttt##refs.len()) mark_object(ttt##refs[i]);
    BASURA(tbl)
    BASURA(fun)
    BASURA(pro)
#undef BASURA
}

NatTbl::NatTbl(NatTblTag tag)
{
    this->tag = tag;
    this->priv = nullptr;
}

NatTbl::NatTbl(NatTbl &&that)
{
    this->priv = that.priv;
    this->tag  = that.tag;
    // TODO: set tag or priv to prevent destruction
    that.priv = nullptr;
}

NatTbl::~NatTbl()
{
    // pass
}

void NatTbl::print() const
{
    switch (this->tag) {
      case DF_STD:    printf("<STD>");    break;
      case DF_STD_IO: printf("<STD$io>"); break;
      case DF_STD_A:  printf("<STD$a>");  break;
    }
}

#include "df-std/df_std.cpp"

bool NatTbl::get(key_t k, DfVal &v) const
{
    switch (this->tag) {
      case DF_STD:
        return df_std::get(k, v);
      case DF_STD_IO:
        return df_std::io_get(k, v);
      case DF_STD_A:
        return df_std::a_get(k, v);
      default: unreachable();
    }
}

NatFun::NatFun(NatFunTag t)
{
    this->tag = t;
    switch (t) {
      case DF_STD_A_LEN: this->eval = df_std::a_len; break;
    }
}

void NatPro::print() const
{
    switch (this->tag) {
      case DF_STD_IO_PUT: printf("! \"STD$io$put\""); break;
      case DF_STD_GC: printf("! STD$gc"); break;
      case DF_STD_A_EKE: printf("! STD$a$eke"); break;
    }
}

NatPro::NatPro(NatProTag t)
{
    this->tag = t;
    switch (t) {
      case DF_STD_IO_PUT: this->exec = df_std::io_put; break;
      case DF_STD_GC:     this->exec = df_std::gc;     break;
      case DF_STD_A_EKE:  todo("eke");//this->exec = df_std_a_eke;   break;
    }
}

#if 0
void nat_fn_print(enum NatFnTag t)
{
    switch (t) {
      case DF_STD_A_LEN: printf("\"STD$a$eke#\""); break;
    }
}

struct NatFn nat_fn_from(enum NatFnTag t)
{
    struct NatFn nf;
    nf.tag = t;
    switch (t) {
      case DF_STD_A_LEN: nf.eval = df_std_a_len; break;
    }
    return nf;
}

#endif
