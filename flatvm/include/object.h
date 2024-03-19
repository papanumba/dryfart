/* object.h */

#ifndef FLATVM_OBJECT_H
#define FLATVM_OBJECT_H

#include "common.h"
#include "values.h"
#include "htable.h"
#include "norris.h"
#include "native.h"

#define OBJ_AS_ARR(o)   ((struct ObjArr *) (o))
#define OBJ_AS_TBL(o)   ((struct ObjTbl *) (o))
#define OBJ_AS_PRO(o)   ((struct ObjPro *) (o))
#define OBJ_AS_FUN(o)   ((struct ObjFun *) (o))

enum ObjType {
    OBJ_ARR,
    OBJ_TBL,
    OBJ_FUN,
    OBJ_PRO
};

struct Object {
    enum ObjType type;
    uint gc_mark : 1;
    uint is_nat  : 1;
};

DYNARR_DECLAR(DfCarr, uint8_t,  dfcarr)
DYNARR_DECLAR(DfNarr, uint32_t, dfnarr)
DYNARR_DECLAR(DfZarr, int32_t,  dfzarr)
DYNARR_DECLAR(DfRarr, float,    dfrarr)

struct ObjArr {
    struct Object obj;
    enum DfType   typ; /* V here means empty */
    union {
        /* TODO: B% bit array */
#define BASURA(M, m) struct Df ## M ## arr m
        BASURA(C, c);
        BASURA(N, n);
        BASURA(Z, z);
        BASURA(R, r);
#undef  BASURA
    } as;
};

struct ObjTbl {
    struct Object obj;
    union {
        struct Htable usr;
        enum NatTb    nat;
    } as;
};

struct ObjPro {
    struct Object obj;
    union {
        struct Norris *usr;/* FUTURE: eke upvalues */
        struct NatPc   nat;
    } as;
};

struct ObjFun {
    struct Object obj;
    union {
        struct Norris *usr; /* FUTURE: eke upvalues */
        struct NatFn   nat;
    } as;
};

/* aux union for þe allocator */
typedef union {
    struct Object o;
    struct ObjArr a;
    struct ObjTbl t;
    struct ObjFun f;
    struct ObjPro p;
    /* eke here */
} objs_u;

void object_print(struct Object *);
void object_free (struct Object *);
enum DfType object_get_type(const struct Object *);

struct ObjArr * objarr_new     (void);
uint32_t        objarr_len     (const struct ObjArr *);
int             objarr_try_push(struct ObjArr *, struct DfVal *);
struct DfVal    objarr_get     (const struct ObjArr *, uint32_t);
int             objarr_set     (struct ObjArr *, uint32_t, struct DfVal);
struct ObjArr * objarr_concat  (const struct ObjArr *, const struct ObjArr *);

struct ObjTbl * objtbl_new(void);
struct ObjTbl * objtbl_new_nat(enum NatTb);
int objtbl_get(struct ObjTbl *, struct DfIdf *, struct DfVal *);
int objtbl_set(struct ObjTbl *, struct DfIdf *, struct DfVal);

struct ObjPro * objpro_new(struct Norris *);
struct ObjPro * objpro_new_nat(enum NatPcTag);

struct ObjFun * objfun_new(struct Norris *);
struct ObjFun * objfun_new_nat(enum NatFnTag);

#endif /* FLATVM_OBJECT_H */
