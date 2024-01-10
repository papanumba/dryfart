/* object.c */

#include <stdio.h>
#include <string.h>
#include "alzhmr.h"
#include "object.h"
#include "falloc.h"

static void objarr_print(struct ObjArr *);
static void objarr_free (struct ObjArr *);
static void objarr_grow (struct ObjArr *, uint);
static int  objarr_eq   (struct ObjArr *, struct ObjArr *);

static void objtbl_print(struct ObjTbl *);
static void objtbl_free (struct ObjTbl *);

static struct Object * alloc_object(enum ObjType);
static inline int       arrt_valt_eq(enum ArrType, enum ValType);
static inline size_t sizeof_arr_elem(enum ArrType);

static inline enum ArrType valt2arrt(enum ValType);
static inline enum ValType arrt2valt(enum ArrType);
static inline char         arrt2char(enum ArrType);


void object_print(struct Object *o)
{
    switch (o->type) {
      case OBJ_ARR: objarr_print(OBJ_AS_ARR(o)); break;
      case OBJ_TBL: objtbl_print(OBJ_AS_TBL(o)); break;
    }
}

int object_eq(struct Object *o0, struct Object *o1)
{
    if (o0 == o1)
        return TRUE;
    if (o0->type != o1->type)
        return FALSE;
    int b = FALSE;
    switch (o0->type) {
      case OBJ_ARR: b = objarr_eq(OBJ_AS_ARR(o0), OBJ_AS_ARR(o1)); break;
      case OBJ_TBL: break;
    }
    return b;
}

void object_free(struct Object *o)
{
    switch (o->type) {
      case OBJ_ARR: objarr_free(OBJ_AS_ARR(o)); break;
      case OBJ_TBL: objtbl_free(OBJ_AS_TBL(o)); break;
    }
    falloc_free(o);
}

/* create empty array */
struct ObjArr * objarr_new(void)
{
    struct ObjArr *arr = OBJ_AS_ARR(alloc_object(OBJ_ARR));
    arr->len = 0;
    arr->cap = 0;
    arr->typ = ARR_E;
    arr->as.c = NULL; /* for safety */
    return arr;
}

int objarr_try_push(struct ObjArr *a, struct DfVal *v)
{
    if (a->typ != ARR_E && !arrt_valt_eq(a->typ, v->type)) {
        fprintf(stderr, "ERROR: cannot push %c value into %c array\n",
            valt2char(v->type), arrt2char(a->typ)
        );
        return FALSE;
    }
    if (a->typ == ARR_E)
        a->typ = valt2arrt(v->type); // checked þat are compatible
        // here a will have v's type, but size 0
    if (a->typ == ARR_B)
        panic("todo: arrb push");
    if (a->cap < a->len + 1)
        objarr_grow(a, GROW_CAP(a->cap));
    switch (a->typ) {
      case ARR_E: break; // unreachable
      case ARR_B: break; // todo
      case ARR_C: a->as.c[a->len] = v->as.c; break;
      case ARR_N: a->as.n[a->len] = v->as.n; break;
      case ARR_Z: a->as.z[a->len] = v->as.z; break;
      case ARR_R: a->as.r[a->len] = v->as.r; break;
    }
    a->len++;
    return TRUE;
}

/* returns V (null) is array is empty or idx is out of bounds */
struct DfVal objarr_get(struct ObjArr *arr, uint32_t idx)
{
    struct DfVal val = {.type = VAL_V};
    if (idx >= arr->len)
        return val; /* V */
    val.type = arrt2valt(arr->typ);
    switch (val.type) {
      case VAL_V: return val; /* empty array */
      case VAL_B: todo("get B arr"); return val;
#define BASURA(vt, x) \
      case vt: val.as.x = arr->as.x[idx]; break;
      BASURA(VAL_C, c)
      BASURA(VAL_N, n)
      BASURA(VAL_Z, z)
      BASURA(VAL_R, r)
#undef BASURA
      default: unreachable();
    }
    return val;
}

int objarr_set(struct ObjArr *arr, uint32_t idx, struct DfVal *val)
{
    if (idx >= arr->len) {
        fprintf(stderr, "ERROR: Index %u out of bounds (len = %u)\n",
            (uint) idx, (uint) arr->len);
        return FALSE;
    }
    enum ValType at = arrt2valt(arr->typ);
    if (at != val->type) {
        fprintf(stderr, "ERROR: cannot set %c%% value into %c%% array\n",
            valt2char(val->type), valt2char(at));
        return FALSE;
    }
    switch (at) {
      case VAL_B: todo("get B arr"); return FALSE;
#define BASURA(vt, x) \
      case vt: arr->as.x[idx] = val->as.x; break;
      BASURA(VAL_C, c)
      BASURA(VAL_N, n)
      BASURA(VAL_Z, z)
      BASURA(VAL_R, r)
#undef BASURA
      default: unreachable();
    }
    return TRUE;
}

/* returns NULL if error */
struct ObjArr * objarr_concat(struct ObjArr *a, struct ObjArr *b)
{
    struct ObjArr *ab = objarr_new();
    /* TODO: more efficient */
    /* push a */
    for (uint32_t i = 0; i < a->len; ++i) {
        struct DfVal elem = objarr_get(a, i);
        if (!objarr_try_push(ab, &elem))
            return NULL;
    }
    /* push b */
    for (uint32_t i = 0; i < b->len; ++i) {
        struct DfVal elem = objarr_get(b, i);
        if (!objarr_try_push(ab, &elem))
            return NULL;
    }
    return ab;
}

/* create empty table */
struct ObjTbl * objtbl_new(void)
{
    struct ObjTbl *tbl = OBJ_AS_TBL(alloc_object(OBJ_TBL));
    htable_init(&tbl->tbl);
    return tbl;
}

/******************** S T A T I C ***************************/

static void objarr_print(struct ObjArr *arr)
{
    size_t i;
    size_t len = arr->len;
    if (arr->typ == ARR_C) { // string special case
        putchar('"');
        for (i = 0; i < arr->len; ++i)
            putchar(arr->as.c[i]);
        putchar('"');
        return;
    }
    putchar('_');
    switch (arr->typ) {
      case ARR_E: break;
      case ARR_B:
        for (i = 0; i < len; ++i) {
            int b = arr->as.b[i/8] & (1 << (i%8));
            printf("%c, ", b?'T':'F');
        }
        break;
      case ARR_C: break; // unreachable
      case ARR_N:
        for (i = 0; i < len; ++i)
            printf("%lu, ", (ulong) arr->as.n[i]);
        break;
      case ARR_Z:
        for (i = 0; i < len; ++i)
            printf("%ld, ", (long) arr->as.z[i]);
        break;
      case ARR_R:
        for (i = 0; i < len; ++i)
            printf("%f, ", (float) arr->as.r[i]);
        break;
    }
    putchar(';');
}

/* free only interior array, not þe objarr header */
static void objarr_free(struct ObjArr *arr)
{
    if (arr->typ == ARR_E)
        return;
    realloc_or_free(arr->as.c, 0); // any would do
}

static void objarr_grow(struct ObjArr *arr, uint newcap)
{
    size_t new_size = newcap * sizeof_arr_elem(arr->typ);
    switch (arr->typ) {
      case ARR_E: panic("unreachable"); return;
#define BASURA(t, x) \
      case t: arr->as.x = realloc_or_free(arr->as.x, new_size); break;
      BASURA(ARR_B, b)
      BASURA(ARR_C, c)
      BASURA(ARR_N, n)
      BASURA(ARR_Z, z)
      BASURA(ARR_R, r)
#undef BASURA
    }
    arr->cap = newcap;
}

static int objarr_eq(struct ObjArr *a0, struct ObjArr *a1)
{
    if (a0->typ != a1->typ || a0->len != a1->len)
        return FALSE;
    switch (a0->typ) {
      case ARR_E: return TRUE;
      case ARR_B: panic("todo: eq arr B%%"); break;
#define BASURA(t, x) \
      case t: return 0 == memcmp(a0->as.x, a1->as.x, sizeof(a0->as.x[0]) * a0->len);
      BASURA(ARR_C, c)
      BASURA(ARR_N, n)
      BASURA(ARR_Z, z)
#undef BASURA
      case ARR_R: return FALSE;
    }
    return FALSE; /* unreachable */
}

static void objtbl_print(struct ObjTbl *t)
{
    htable_print(&t->tbl);
}

static void objtbl_free (struct ObjTbl *t)
{
    htable_free(&t->tbl);
}

static struct Object * alloc_object(enum ObjType type)
{
    struct Object *obj = falloc_alloc();
    obj->type = type;
    obj->gc_mark = FALSE;
    return obj;
}

/* return if a is compatible with v,
** ARR_E is always eq to a val type.
*/
static inline int arrt_valt_eq(enum ArrType a, enum ValType v)
{
    switch (a) {
      case ARR_E: return TRUE;
      case ARR_B: return VAL_B == v;
      case ARR_C: return VAL_C == v;
      case ARR_N: return VAL_N == v;
      case ARR_Z: return VAL_Z == v;
      case ARR_R: return VAL_R == v;
    }
    panic("end of function");
    return FALSE;
}

static inline size_t sizeof_arr_elem(enum ArrType a)
{
    size_t size = 0;
    switch (a) {
      case ARR_E: panic("cannot have sizeof(ARR_E)"); break;
#define BASURA(at, t) \
      case at: size = sizeof(t); break;
      BASURA(ARR_B, uint8_t)
      BASURA(ARR_C, char)
      BASURA(ARR_N, uint32_t)
      BASURA(ARR_Z, int32_t)
      BASURA(ARR_R, float)
    }
    return size;
}

static inline enum ArrType valt2arrt(enum ValType vt)
{
    switch (vt) {
      case VAL_B: return ARR_B;
      case VAL_C: return ARR_C;
      case VAL_N: return ARR_N;
      case VAL_Z: return ARR_Z;
      case VAL_R: return ARR_R;
      default:
        panic("unreachable");
        return ARR_E;
    }
}

static inline enum ValType arrt2valt(enum ArrType at)
{
    switch (at) {
      case ARR_E: return VAL_V;
      case ARR_B: return VAL_B;
      case ARR_C: return VAL_C;
      case ARR_N: return VAL_N;
      case ARR_Z: return VAL_Z;
      case ARR_R: return VAL_R;
    }
    unreachable();
    return VAL_V;
}

static inline char arrt2char(enum ArrType at)
{
    char c = '\0';
    switch (at) {
      case ARR_E: c = 'E'; break;
      case ARR_B: c = 'B'; break;
      case ARR_C: c = 'C'; break;
      case ARR_N: c = 'N'; break;
      case ARR_Z: c = 'Z'; break;
      case ARR_R: c = 'R'; break;
    }
    return c;
}
