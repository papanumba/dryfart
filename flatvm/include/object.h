/* object.h */

#ifndef FLATVM_OBJECT_H
#define FLATVM_OBJECT_H

#include "common.h"
#include "values.h"
#include "htable.h"

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
    int gc_mark;
};

enum ArrType {
    ARR_E,
    ARR_B,
    ARR_C,
    ARR_N,
    ARR_Z,
    ARR_R
};

struct ObjArr {
    struct Object obj;
    size_t        len;
    size_t        cap;
    enum ArrType  typ;
    union {
        uint8_t  *b; /* packed bit array */
        char     *c;
        uint32_t *n;
        int32_t  *z;
        float    *r;
    }             as;
};

struct ObjTbl {
    struct Object obj;
    struct Htable tbl;
};

struct ObjPro {
    struct Object obj;
    struct Norris *norr;
    uint line;
    /* FUTURE: eke upvalues */
};

struct ObjFun {
    struct Object obj;
    struct Norris *norr;
    uint line;
    /* FUTURE: eke upvalues */
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
int  object_eq   (struct Object *, struct Object *);
void object_free (struct Object *);

struct ObjArr * objarr_new     (void);
int             objarr_try_push(struct ObjArr *, struct DfVal *);
struct DfVal    objarr_get     (struct ObjArr *, uint32_t);
int             objarr_set     (struct ObjArr *, uint32_t, struct DfVal *);
struct ObjArr * objarr_concat  (struct ObjArr *, struct ObjArr *);

struct ObjTbl * objtbl_new     (void);
struct ObjPro * objpro_new     (struct Norris *, uint);
struct ObjFun * objfun_new     (struct Norris *, uint);


#endif /* FLATVM_OBJECT_H */
