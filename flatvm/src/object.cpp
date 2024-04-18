/* object.c */

#include <cstdio>
#include <cstring>
#include <new>
#include "alzhmr.h"
#include "object.h"
//#include "falloc.h"

/*static void objtbl_print(struct ObjTbl *);
static void objtbl_free (struct ObjTbl *);

static void objpro_print(struct ObjPro *);
static void objpro_free (struct ObjPro *);

static void objfun_print(struct ObjFun *);
static void objfun_free (struct ObjFun *);

static struct Object * alloc_object(enum ObjType);*/
static inline bool     arrt_valt_eq(DfType, enum ValType);

static inline DfType  valt2arrt(ValType);
static inline ValType arrt2valt(DfType);

/*************************** A R R A Y S ***************************/

const char * accres_what(AccRes ar)
{
    switch (ar) {
      case AccRes::OK:            return "OK";
      case AccRes::OUT_OF_BOUNDS: return "out of bounds";
      case AccRes::DIFF_TYPE:     return "different types";
    }
}

ArrObj::ArrObj(DfVal &&v)
{
    switch (v.as_type()) {
      case DfType::V: unreachable();
      case DfType::B: todo("B% arr, in ArrObj(DfVal &&)");
#define BASURA(dft, m, t) \
      case dft:                             \
        new (&this->as.m) DynArr<t>();      \
        this->as.m.push(std::move(v.as.m)); \
        break;
      BASURA(DfType::C, c, uint8_t)
      BASURA(DfType::N, n, uint32_t)
      BASURA(DfType::Z, z, int32_t)
      BASURA(DfType::R, r, float)
#undef BASURA
      default: todo("singleton other array types");
    }
    this->typ = v.as_type();
}

ArrObj::~ArrObj()
{
    switch (this->typ) {
      case DfType::V: return;
#define BASURA(dft, m, t) \
      case dft: this->as.m.~DynArr<t>(); break;
      BASURA(DfType::C, c, uint8_t)
      BASURA(DfType::N, n, uint32_t)
      BASURA(DfType::Z, z, int32_t)
      BASURA(DfType::R, r, float)
#undef BASURA
      default: todo("destruct other arrays");
    }
}

uint32_t ArrObj::len() const
{
    uint32_t len = 0;
    switch (this->typ) {
      case DfType::V: return 0;
      case DfType::B: todo("B% array len"); break;
#define BASURA(M, m) case M: len = this->as.m.len(); break;
      BASURA(DfType::C, c)
      BASURA(DfType::N, n)
      BASURA(DfType::Z, z)
      BASURA(DfType::R, r)
#undef BASURA
      default: todo("len other array types");
    }
    return len;
}

// returns true if OK, returns false if are different types
AccRes ArrObj::push(DfVal &&v)
{
    DfType at = this->typ;
    if (at == DfType::V) {
        new(this) ArrObj(std::move(v));
        return AccRes::OK;
    }
    if (!arrt_valt_eq(at, v.type))
        return AccRes::DIFF_TYPE;
    switch (at) {
      case DfType::V: unreachable(); break;
      case DfType::B: todo("push B% array"); break;
#define BASURA(arrt, m, T) \
      case arrt:                            \
        this->as.m.push(std::move(v.as.m)); \
        break;
      BASURA(DfType::C, c, uint8_t)
      BASURA(DfType::N, n, uint32_t)
      BASURA(DfType::Z, z, int32_t)
      BASURA(DfType::R, r, float)
#undef BASURA
      default: todo("push other array types");
    }
    return AccRes::OK;
}

AccRes ArrObj::get(uint32_t idx, DfVal &ret) const
{
    switch (this->typ) {
      case DfType::V: return AccRes::OUT_OF_BOUNDS; // coz it's mt
      case DfType::B: todo("get from B% array"); break;
#define BASURA(arrx, x) \
      case arrx:                          \
        if (idx >= this->as.x.len())      \
            return AccRes::OUT_OF_BOUNDS; \
        ret = DfVal(this->as.x[idx]);     \
        break;
      BASURA(DfType::C, c)
      BASURA(DfType::N, n)
      BASURA(DfType::Z, z)
      BASURA(DfType::R, r)
#undef BASURA
      default: todo("get other array types");
    }
    return AccRes::OK;
}

AccRes ArrObj::set(uint32_t idx, DfVal &&val)
{
    DfType at = this->typ;
    if (at == DfType::V)
        return AccRes::OUT_OF_BOUNDS;
    if (!arrt_valt_eq(at, val.type))
        return AccRes::DIFF_TYPE;
    switch (at) {
      case DfType::V: unreachable(); break;
      case DfType::B: todo("B% array"); break;
#define BASURA(arrx, x) \
      case arrx:                          \
        if (idx >= this->as.x.len())      \
            return AccRes::OUT_OF_BOUNDS; \
        this->as.x[idx] = val.as.x;       \
        break;
      BASURA(DfType::C, c)
      BASURA(DfType::N, n)
      BASURA(DfType::Z, z)
      BASURA(DfType::R, r)
#undef BASURA
      default: todo("set other array types");
    }
    return AccRes::OK;
}

void ArrObj::print() const
{
    putchar('_');
    switch (this->typ) {
      case DfType::V: break;
      case DfType::B: todo("B% array"); break;
#define BASURA(arrx, x, fmt) \
      case arrx: {                      \
        auto &arr = this->as.x;         \
        printf("%" #fmt, arr[0]);       \
        auto len = arr.len();           \
        FOR(i, 1, len)                  \
            printf(", %" #fmt, arr[i]); \
        break;                          \
      }
      BASURA(DfType::C, c, c)
      BASURA(DfType::N, n, u)
      BASURA(DfType::Z, z, d)
      BASURA(DfType::R, r, f)
#undef BASURA
      default: todo("print other array types");
    }
    putchar(';');
}

AccRes ArrObj::concat(const ArrObj &that, ArrObj &res) const
{
    new (&res) ArrObj();
    // TODO: more efficient, idea: extend from slice
    // push 1st array (*this)
    TIL(i, this->len()) {
        DfVal elem;
        (void) this->get(i, elem);
        (void) res.push(std::move(elem));
    }
    // push 2nd array (that)
    TIL(i, that.len()) {
        DfVal elem;
        (void) that.get(i, elem);
        auto r = res.push(std::move(elem));
        if (r != AccRes::OK) {
            res.~ArrObj();
            return r;
        }
    }
    return AccRes::OK;
}

void FunObj::print() const
{
    printf("some func");
}

void ProObj::print() const
{
    printf("some proc");
}

void TblObj::print() const
{
    printf("some table");
}

#if 0

/* create empty table */
struct ObjTbl * objtbl_new(void)
{
    struct ObjTbl *tbl = OBJ_AS_TBL(alloc_object(OBJ_TBL));
    tbl->obj.is_nat = FALSE;
    htable_init(&tbl->as.usr);
    return tbl;
}

struct ObjTbl * objtbl_new_nat(enum NatTb nt)
{
    struct ObjTbl *tbl = OBJ_AS_TBL(alloc_object(OBJ_TBL));
    tbl->obj.is_nat = TRUE;
    tbl->as.nat = nt;
    return tbl;
}

int objtbl_get(struct ObjTbl *t, struct DfIdf *k, struct DfVal *v)
{
    if (t->obj.is_nat)
        return nat_tb_get(t->as.nat, k, v);
    else
        return htable_get(&t->as.usr, k, v);
}

int objtbl_set(struct ObjTbl *t, struct DfIdf *k, struct DfVal v)
{
    if (t->obj.is_nat)
        return FALSE; /* immutable native tables */
    else
        return htable_set(&t->as.usr, k, v);
}

struct ObjPro * objpro_new(struct Norris *n)
{
    struct ObjPro *pro = OBJ_AS_PRO(alloc_object(OBJ_PRO));
    pro->obj.is_nat = FALSE;
    pro->as.usr = n;
    return pro;
}

struct ObjPro * objpro_new_nat(enum NatPcTag t)
{
    struct ObjPro *pro = OBJ_AS_PRO(alloc_object(OBJ_PRO));
    pro->obj.is_nat = TRUE;
    pro->as.nat = nat_pc_from(t);
    return pro;
}

struct ObjFun * objfun_new(struct Norris *n)
{
    struct ObjFun *fun = OBJ_AS_FUN(alloc_object(OBJ_FUN));
    fun->as.usr = n;
    return fun;
}

struct ObjFun * objfun_new_nat(enum NatFnTag t)
{
    struct ObjFun *fun = OBJ_AS_FUN(alloc_object(OBJ_FUN));
    fun->obj.is_nat = TRUE;
    fun->as.nat = nat_fn_from(t);
    return fun;
}

/******************** S T A T I C ***************************/

static void ArrObj::print(struct ObjArr *arr)
{
    switch (arr->typ) {
      case DfType::V: printf("_;"); break;
      case DfType::B: todo("print B% array"); break;
      case DfType::C:
        for (uint i = 0; i < arr->as.c.len; ++i)
            putchar((char) arr->as.c.arr[i]);
        break;
#define BASURA(arrx, x, fmt) \
      case arrx: {                              \
        putchar('_');                           \
        uint len1 = arr->as.x.len - 1; /* len is > 0 */ \
        for (uint i = 0; i < len1; ++i)         \
            printf(fmt ", ", arr->as.x.arr[i]); \
        printf(fmt ";", arr->as.x.arr[len1]);   \
        break;                                  \
      }
      BASURA(DfType::N, n, "%u")
      BASURA(DfType::Z, z, "%d")
      BASURA(DfType::R, r, "%f")
#undef BASURA
      default: todo("print other arrays");
    }
}

/* free only interior array, not þe objarr header */
static void ArrObj::free(struct ObjArr *arr)
{
    if (arr->typ == DfType::V)
        return;
    switch (arr->typ) {
      case DfType::V: return;
      case DfType::B: todo("free B% array"); return ;
#define BASURA(arrx, x) case arrx: df ## x ## arr_free(&arr->as.x); break
    BASURA(DfType::C, c);
    BASURA(DfType::N, n);
    BASURA(DfType::Z, z);
    BASURA(DfType::R, r);
#undef BASURA
      default: todo("free other array types");
    }
}

static void objtbl_print(struct ObjTbl *t)
{
    if (t->obj.is_nat)
        nat_tb_print(t->as.nat);
    else
        htable_print(&t->as.usr);
}

static void objtbl_free (struct ObjTbl *t)
{
    if (!t->obj.is_nat)
        htable_free(&t->as.usr);
}

static void objpro_print(struct ObjPro *p)
{
    if (p->obj.is_nat) {
        nat_pc_print(p->as.nat.tag);
        return;
    }
    struct Norris *nor = p->as.usr;
    if (nor->nam != NULL)
        printf("<! \"%s\">", nor->nam->str);
    else
        printf("<! from line %u>", nor->lne);
}

static void objpro_free (struct ObjPro *p)
{
    (void)(p);
    /* FUTURE: free upvalues */
}

static void objfun_print(struct ObjFun *f)
{
    if (f->obj.is_nat) {
        nat_fn_print(f->as.nat.tag);
        return;
    }
    struct Norris *nor = f->as.usr;
    if (nor->nam != NULL)
        printf("<# \"%s\">", nor->nam->str);
    else
        printf("<# from line %u>", nor->lne);
}

static void objfun_free (struct ObjFun *f)
{
    (void)(f);
    /* FUTURE: free upvalues */
}

static struct Object * alloc_object(enum ObjType type)
{
    struct Object *obj = falloc_alloc();
    obj->type = type;
    obj->gc_mark = FALSE;
    return obj;
}

#endif

/* return if a is compatible with v,
** DfType::V is always eq to a val type.
*/
static inline bool arrt_valt_eq(DfType a, enum ValType v)
{
    switch (a) {
      case DfType::V: return true;
      case DfType::B: return VAL_B == v;
      case DfType::C: return VAL_C == v;
      case DfType::N: return VAL_N == v;
      case DfType::Z: return VAL_Z == v;
      case DfType::R: return VAL_R == v;
      default:
        fprintf(stderr, "%u and %u ", (uint) a, (uint) v);
        todo("arrt valt eq other array types");
    }
}

static inline enum DfType valt2arrt(enum ValType vt)
{
    switch (vt) {
      case VAL_B: return DfType::B;
      case VAL_C: return DfType::C;
      case VAL_N: return DfType::N;
      case VAL_Z: return DfType::Z;
      case VAL_R: return DfType::R;
      default: unreachable();
    }
}

static inline enum ValType arrt2valt(enum DfType at)
{
    switch (at) {
      case DfType::V: return VAL_V;
      case DfType::B: return VAL_B;
      case DfType::C: return VAL_C;
      case DfType::N: return VAL_N;
      case DfType::Z: return VAL_Z;
      case DfType::R: return VAL_R;
      default:
        todo("arrt2valt other array types");
    }
    unreachable();
}
