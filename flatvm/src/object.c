/* object.c */

#include <stdio.h>
#include <string.h>
#include "alzhmr.h"
#include "object.h"
#include "falloc.h"

static int  objarr_sgle (struct ObjArr *, struct DfVal *);
static void objarr_print(struct ObjArr *);
static void objarr_free (struct ObjArr *);

static void objtbl_print(struct ObjTbl *);
static void objtbl_free (struct ObjTbl *);

static void objpro_print(struct ObjPro *);
static void objpro_free (struct ObjPro *);

static void objfun_print(struct ObjFun *);
static void objfun_free (struct ObjFun *);

static struct Object * alloc_object(enum ObjType);
static inline int       arrt_valt_eq(enum DfType, enum ValType);

static inline enum DfType  valt2arrt(enum ValType);
static inline enum ValType arrt2valt(enum DfType);

static void print_out_of_bounds(uint32_t, uint32_t);

/* dummy function to pass when no destructor needed, see below */
static inline void pass(void *p) {(void)(p); return;}

DYNARR_API_C(DfCarr, uint8_t,  dfcarr, pass)
DYNARR_API_C(DfNarr, uint32_t, dfnarr, pass)
DYNARR_API_C(DfZarr, int32_t,  dfzarr, pass)
DYNARR_API_C(DfRarr, float,    dfrarr, pass)

void object_print(struct Object *o)
{
    switch (o->type) {
      case OBJ_ARR: objarr_print(OBJ_AS_ARR(o)); break;
      case OBJ_TBL: objtbl_print(OBJ_AS_TBL(o)); break;
      case OBJ_PRO: objpro_print(OBJ_AS_PRO(o)); break;
      case OBJ_FUN: objfun_print(OBJ_AS_FUN(o)); break;
    }
}

void object_free(struct Object *o)
{
    switch (o->type) {
      case OBJ_ARR: objarr_free(OBJ_AS_ARR(o)); break;
      case OBJ_TBL: objtbl_free(OBJ_AS_TBL(o)); break;
      case OBJ_PRO: objpro_free(OBJ_AS_PRO(o)); break;
      case OBJ_FUN: objfun_free(OBJ_AS_FUN(o)); break;
    }
    falloc_free(o);
}

enum DfType object_get_type(const struct Object *o)
{
    switch (o->type) {
      case OBJ_ARR: return DFTYPE_A;
      case OBJ_TBL: return DFTYPE_T;
      case OBJ_FUN: return DFTYPE_F;
      case OBJ_PRO: return DFTYPE_P;
    }
    return DFTYPE_V; /* unreachable */
}

/* create empty array */
struct ObjArr * objarr_new(void)
{
    struct ObjArr *arr = OBJ_AS_ARR(alloc_object(OBJ_ARR));
    arr->typ = DFTYPE_V;
    arr->obj.is_nat = FALSE;
    return arr;
}

uint32_t objarr_len(const struct ObjArr *arr)
{
    uint32_t len = 0;
    switch (arr->typ) {
      case DFTYPE_V: break;
      case DFTYPE_B: todo("B% array len"); break;
#define BASURA(arrx, x) case arrx: len = arr->as.x.len; break;
      BASURA(DFTYPE_C, c)
      BASURA(DFTYPE_N, n)
      BASURA(DFTYPE_Z, z)
      BASURA(DFTYPE_R, r)
#undef BASURA
      default: todo("other array types");
    }
    return len;
}

int objarr_try_push(struct ObjArr *a, struct DfVal *v)
{
    enum DfType at = a->typ;
    if (at == DFTYPE_V)
        return objarr_sgle(a, v);
    if (!arrt_valt_eq(at, v->type)) {
        fprintf(stderr,
            "Type ERROR: cannot push %c%% element to %c%% array\n",
            (char) val2type(v), (char) at);
    }
    switch (at) {
      case DFTYPE_V: unreachable(); break;
      case DFTYPE_B: todo("push B% array"); break;
#define BASURA(arrt, x) \
      case arrt: df ## x ## arr_push(&a->as.x, v->as.x); break
      BASURA(DFTYPE_C, c);
      BASURA(DFTYPE_N, n);
      BASURA(DFTYPE_Z, z);
      BASURA(DFTYPE_R, r);
#undef BASURA
      default: todo("other array types");
    }
    return TRUE;
}

/* returns V (null) is array is empty or idx is out of bounds */
struct DfVal objarr_get(const struct ObjArr *arr, uint32_t idx)
{
    struct DfVal val = {.type = VAL_V};
    switch (arr->typ) {
      case DFTYPE_V: eputln("cannot get from empty array"); break;
      case DFTYPE_B: todo("get from B% array"); break;
#define BASURA(arrx, x) \
      case arrx: do {                       \
        uint32_t len = arr->as.x.len;       \
        if (idx >= len) {                   \
            print_out_of_bounds(idx, len);  \
        }                                   \
        val.type = arrt2valt(arrx);         \
        val.as.x = arr->as.x.arr[idx];      \
        break;                              \
      } while (FALSE)
      BASURA(DFTYPE_C, c);
      BASURA(DFTYPE_N, n);
      BASURA(DFTYPE_Z, z);
      BASURA(DFTYPE_R, r);
#undef BASURA
      default: todo("other array types");
    }
    return val;
}

int objarr_set(struct ObjArr *arr, uint32_t idx, struct DfVal val)
{
    enum DfType at = arr->typ;
    if (at == DFTYPE_V) {
        print_out_of_bounds(idx, 0);
        return FALSE;
    }
    if (!arrt_valt_eq(at, val.type)) {
        fprintf(stderr,
            "Type ERROR: cannot push %c%% element to %c%% array\n",
            (char) val2type(&val), (char) at);
    }
    switch (at) {
      case DFTYPE_V: unreachable(); break;
      case DFTYPE_B: todo("B% array"); break;
#define BASURA(arrx, x) \
      case arrx: do {                       \
        uint32_t len = arr->as.x.len;       \
        if (idx >= len) {                   \
            print_out_of_bounds(idx, len);  \
            return FALSE;                   \
        }                                   \
        arr->as.x.arr[idx] = val.as.x;      \
        break;                              \
      } while (FALSE)
      BASURA(DFTYPE_C, c);
      BASURA(DFTYPE_N, n);
      BASURA(DFTYPE_Z, z);
      BASURA(DFTYPE_R, r);
#undef BASURA
      default: todo("other array types");
    }
    return TRUE;
}

/* returns NULL if error */
struct ObjArr * objarr_concat(const struct ObjArr *a, const struct ObjArr *b)
{
    struct ObjArr *ab = objarr_new();
    uint32_t i;
    uint32_t alen = objarr_len(a);
    /* TODO: more efficient */
    /* push a */
    for (i = 0; i < alen; ++i) {
        struct DfVal elem = objarr_get(a, i);
        objarr_try_push(ab, &elem);
    }
    /* push b */
    uint32_t blen = objarr_len(b);
    for (i = 0; i < blen; ++i) {
        struct DfVal elem = objarr_get(b, i);
        if (!objarr_try_push(ab, &elem)) {
            objarr_free(ab);
            return NULL;
        }
    }
    return ab;
}

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

static int objarr_sgle(struct ObjArr *a, struct DfVal *v)
{
    switch (v->type) {
      case VAL_V: eputln("cannot make an array of type V"); return FALSE;
      case VAL_B: todo("B% array"); return FALSE;
#define BASURA(valx, x)                 \
      case valx:                        \
        a->typ = valt2arrt(valx);       \
        df ## x ## arr_init(&a->as.x);  \
        df ## x ## arr_push(&a->as.x, v->as.x); \
        return TRUE
      BASURA(VAL_C, c);
      BASURA(VAL_N, n);
      BASURA(VAL_Z, z);
      BASURA(VAL_R, r);
#undef BASURA
      default: panic("foofiwej"); return FALSE;
    }
}

static void objarr_print(struct ObjArr *arr)
{
    switch (arr->typ) {
      case DFTYPE_V: printf("_;"); break;
      case DFTYPE_B: todo("print B% array"); break;
      case DFTYPE_C:
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
      BASURA(DFTYPE_N, n, "%u")
      BASURA(DFTYPE_Z, z, "%d")
      BASURA(DFTYPE_R, r, "%f")
#undef BASURA
      default: todo("other arrays");
    }
}

/* free only interior array, not þe objarr header */
static void objarr_free(struct ObjArr *arr)
{
    if (arr->typ == DFTYPE_V)
        return;
    switch (arr->typ) {
      case DFTYPE_V: return;
      case DFTYPE_B: todo("free B% array"); return ;
#define BASURA(arrx, x) case arrx: df ## x ## arr_free(&arr->as.x); break
    BASURA(DFTYPE_C, c);
    BASURA(DFTYPE_N, n);
    BASURA(DFTYPE_Z, z);
    BASURA(DFTYPE_R, r);
#undef BASURA
      default: todo("other array types");
    }
    /*realloc_or_free(arr->as.c, 0);*/ /* any would do */
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

/* return if a is compatible with v,
** DFTYPE_V is always eq to a val type.
*/
static inline int arrt_valt_eq(enum DfType a, enum ValType v)
{
    switch (a) {
      case DFTYPE_V: return TRUE;
      case DFTYPE_B: return VAL_B == v;
      case DFTYPE_C: return VAL_C == v;
      case DFTYPE_N: return VAL_N == v;
      case DFTYPE_Z: return VAL_Z == v;
      case DFTYPE_R: return VAL_R == v;
      default: todo("other array types");
    }
    panic("end of function");
    return FALSE;
}

static inline enum DfType valt2arrt(enum ValType vt)
{
    switch (vt) {
      case VAL_B: return DFTYPE_B;
      case VAL_C: return DFTYPE_C;
      case VAL_N: return DFTYPE_N;
      case VAL_Z: return DFTYPE_Z;
      case VAL_R: return DFTYPE_R;
      default:
        panic("unreachable");
        return DFTYPE_V;
    }
}

static inline enum ValType arrt2valt(enum DfType at)
{
    switch (at) {
      case DFTYPE_V: return VAL_V;
      case DFTYPE_B: return VAL_B;
      case DFTYPE_C: return VAL_C;
      case DFTYPE_N: return VAL_N;
      case DFTYPE_Z: return VAL_Z;
      case DFTYPE_R: return VAL_R;
      default: todo("other array types");
    }
    unreachable();
    return VAL_V;
}

static void print_out_of_bounds(uint32_t idx, uint32_t len)
{
    fprintf(stderr, "ERROR: index out of bounds %u, with length %u\n",
        idx, len);
}
